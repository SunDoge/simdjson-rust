//! Tape parser for simdjson DOM tape format.
//!
//! The simdjson DOM parser serializes parsed JSON into a "tape" - an array of
//! 64-bit words. Each word encodes its node type in the top 8 bits and a
//! payload (e.g., string offset, number of elements, or sibling index) in the
//! lower 56 bits.
//!
//! ## Tape type byte values (top 8 bits of each tape word)
//!
//! | Char | Hex  | Meaning                                         |
//! |------|------|-------------------------------------------------|
//! | `r`  | 0x72 | Root node (always first and last entries)       |
//! | `{`  | 0x7B | Start of object (payload = end-of-scope index)  |
//! | `}`  | 0x7D | End of object (payload = start index)           |
//! | `[`  | 0x5B | Start of array  (payload = end-of-scope index)  |
//! | `]`  | 0x5D | End of array    (payload = start index)         |
//! | `"`  | 0x22 | String (payload = byte offset into string_buf)  |
//! | `l`  | 0x6C | Signed integer  (next tape word = i64 bits)     |
//! | `u`  | 0x75 | Unsigned integer(next tape word = u64)          |
//! | `d`  | 0x64 | Double          (next tape word = f64 bits)     |
//! | `t`  | 0x74 | Boolean true                                    |
//! | `f`  | 0x66 | Boolean false                                   |
//! | `n`  | 0x6E | Null                                            |

use std::str;

use snafu::prelude::*;

/// The error type returned by Tape operations.
///
/// Each variant names the concrete failure and carries the positions and
/// lengths relevant to it, so callers can inspect *what* went wrong and
/// *where*, rather than parsing a formatted message.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    /// The tape cursor advanced past the end of the tape.
    #[snafu(display("tape position {pos} out of bounds (len={len})"))]
    OutOfBounds { pos: usize, len: usize },

    /// The tape word's tag byte is not a recognised `TapeType`.
    #[snafu(display("unknown tape type byte 0x{tag:02X} at position {pos}"))]
    UnknownTapeType { tag: u8, pos: usize },

    /// The tape word's type is recognised but not valid at this position
    /// (e.g. a closing `}` where a value is expected, or a non-string node
    /// where an object key is expected).
    #[snafu(display("unexpected tape type 0x{found:02X} at position {pos}"))]
    UnexpectedTapeType { found: u8, pos: usize },

    /// A string node's offset or length runs past the end of `string_buf`.
    #[snafu(display("string at offset {offset} out of bounds (string_buf len={len})"))]
    StringOutOfBounds { offset: usize, len: usize },

    /// A number node is missing its second (value) tape word.
    #[snafu(display("value word missing for tape node at position {pos}"))]
    MissingValueWord { pos: usize },

    /// The string bytes at the node's offset are not valid UTF-8.
    #[snafu(display("invalid UTF-8 in string at offset {pos}"))]
    InvalidUtf8 {
        source: std::str::Utf8Error,
        pos: usize,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Mask to extract the lower 56 bits (payload) from a tape word.
const PAYLOAD_MASK: u64 = 0x00FF_FFFF_FFFF_FFFF;

/// Tape type byte values encoded in the top 8 bits of each tape word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TapeType {
    Root        = b'r',
    StartObject = b'{',
    EndObject   = b'}',
    StartArray  = b'[',
    EndArray    = b']',
    String      = b'"',
    Int64       = b'l',
    Uint64      = b'u',
    Double      = b'd',
    True        = b't',
    False       = b'f',
    Null        = b'n',
}

impl TapeType {
    /// Decode the tape type from the top byte of a tape word.
    #[inline]
    pub fn from_tape_word(word: u64) -> Option<Self> {
        let tag = (word >> 56) as u8;
        match tag {
            b'r' => Some(Self::Root),
            b'{' => Some(Self::StartObject),
            b'}' => Some(Self::EndObject),
            b'[' => Some(Self::StartArray),
            b']' => Some(Self::EndArray),
            b'"' => Some(Self::String),
            b'l' => Some(Self::Int64),
            b'u' => Some(Self::Uint64),
            b'd' => Some(Self::Double),
            b't' => Some(Self::True),
            b'f' => Some(Self::False),
            b'n' => Some(Self::Null),
            _ => None,
        }
    }
}
/// A zero-copy representation of a JSON value parsed from the simdjson tape.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Value<'a> {
    Null,
    Bool(bool),
    Int64(i64),
    Uint64(u64),
    Double(f64),
    String(&'a str),
    Array(Vec<Value<'a>>),
    Object(Vec<(&'a str, Value<'a>)>),
}

impl<'a> Value<'a> {
    /// Look up a field in a JSON object by key.
    pub fn get(&self, key: &str) -> Option<&Value<'a>> {
        match self {
            Value::Object(fields) => fields.iter().find(|(k, _)| *k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Look up an element in a JSON array by index.
    pub fn get_index(&self, index: usize) -> Option<&Value<'a>> {
        match self {
            Value::Array(arr) => arr.get(index),
            _ => None,
        }
    }

    /// Returns true if the value is Null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns the boolean value if this is a Bool.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the i64 value if this is an Int64 or a compatible Uint64.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int64(i) => Some(*i),
            Value::Uint64(u) => (*u).try_into().ok(),
            _ => None,
        }
    }

    /// Returns the u64 value if this is a Uint64 or a compatible Int64.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::Uint64(u) => Some(*u),
            Value::Int64(i) => (*i).try_into().ok(),
            _ => None,
        }
    }

    /// Returns the f64 value if this is a Double.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Double(f) => Some(*f),
            _ => None,
        }
    }

    /// Returns the borrowed string slice if this is a String.
    pub fn as_str(&self) -> Option<&'a str> {
        match self {
            Value::String(s) => Some(*s),
            _ => None,
        }
    }

    /// Returns the borrowed array slice if this is an Array.
    pub fn as_array(&self) -> Option<&[Value<'a>]> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns the borrowed object slice if this is an Object.
    pub fn as_object(&self) -> Option<&[(&'a str, Value<'a>)]> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }
}

/// A cursor over the simdjson DOM tape.
///
/// `TapeRef` borrows both the tape and the string buffer slices from a
/// [`simdjson_sys::dom_ffi::TapeView`]. It maintains a `pos` cursor that
/// advances as values are decoded.
///
/// The lifetime `'a` is tied to the [`crate::dom::Parser`] that owns the
/// underlying memory, ensuring the tape remains valid.
#[derive(Debug, Clone, Copy)]
pub struct TapeRef<'a> {
    pub(crate) tape: &'a [u64],
    pub(crate) string_buf: &'a [u8],
    /// Current index into `tape`.
    pub(crate) pos: usize,
}

impl<'a> TapeRef<'a> {
    /// Create a new tape cursor positioned at index 0 (the root `r` node).
    pub fn new(tape: &'a [u64], string_buf: &'a [u8]) -> Self {
        Self {
            tape,
            string_buf,
            pos: 0,
        }
    }

    /// Returns the current position in the tape.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns the raw tape word at the current position.
    #[inline]
    pub(crate) fn current_word(&self) -> u64 {
        self.tape[self.pos]
    }

    /// Public accessor for the raw tape word (needed by serde module).
    #[inline]
    pub fn current_word_public(&self) -> u64 {
        self.current_word()
    }

    /// Returns the type of the current tape node.
    #[inline]
    pub fn tape_type(&self) -> Option<TapeType> {
        if self.pos >= self.tape.len() {
            return None;
        }
        TapeType::from_tape_word(self.current_word())
    }

    /// Returns the lower-56-bit payload of the current tape word.
    #[inline]
    pub(crate) fn payload(&self) -> u64 {
        self.current_word() & PAYLOAD_MASK
    }

    /// Public accessor for payload (needed by serde module).
    #[inline]
    pub fn payload_public(&self) -> u64 {
        self.payload()
    }

    /// Returns the tape index of the closing `}` or `]` for the current
    /// `StartObject` or `StartArray` node.
    ///
    /// In simdjson's tape format, the opening bracket (`{` or `[`) stores a
    /// packed payload: the lower 32 bits are the **next-sibling index** (the
    /// tape position of the first element *after* the closing bracket), and
    /// the upper 24 bits encode the element count. The closing bracket is
    /// therefore at `next_sibling - 1`.
    #[inline]
    pub fn scope_close_idx(&self) -> usize {
        // Extract lower 32 bits = next sibling index.
        let next_sibling = (self.payload() & 0xFFFF_FFFF) as usize;
        // The closing `}` or `]` is one position before the next sibling.
        next_sibling.saturating_sub(1)
    }

    /// Returns the number of direct children in the current `[` / `{` scope.
    ///
    /// simdjson packs the element count into bits 32–55 of the opening
    /// bracket's tape word (mask `0xFFFFFF` after a `>> 32`). This matches
    /// `simdjson::internal::tape_ref::scope_count`.
    #[inline]
    pub fn scope_count(&self) -> usize {
        ((self.current_word() >> 32) & 0xFFFF_FF) as usize
    }

    /// Advance the cursor by `n` tape words.
    #[inline]
    pub fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    /// Move the cursor to an absolute tape index.
    #[inline]
    pub fn seek_public(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Decode the string at the current position.
    ///
    /// The string buffer layout is: 4-byte little-endian length prefix followed
    /// by raw UTF-8 bytes. The tape word payload is a byte offset into
    /// `string_buf`.
    ///
    /// The returned `&str` has lifetime `'a` (tied to the parser), so it can
    /// be used for zero-copy deserialization.
    #[inline]
    pub fn get_string(&self) -> Result<&'a str> {
        debug_assert_eq!(
            self.tape_type(),
            Some(TapeType::String),
            "get_string called on non-string node"
        );
        let offset = self.payload() as usize;
        ensure!(
            offset + 4 <= self.string_buf.len(),
            StringOutOfBoundsSnafu {
                offset,
                len: self.string_buf.len(),
            }
        );
        // First 4 bytes: little-endian u32 length.
        let len_bytes = &self.string_buf[offset..offset + 4];
        let len = u32::from_le_bytes(len_bytes.try_into().unwrap()) as usize;
        let start = offset + 4;
        let end = start + len;
        ensure!(
            end <= self.string_buf.len(),
            StringOutOfBoundsSnafu {
                offset,
                len: self.string_buf.len(),
            }
        );
        // SAFETY: simdjson validates UTF-8 while building the string buffer
        // during stage2; a parse that reaches this point has already passed
        // simdjson's UTF-8 check (a failure would have surfaced as a
        // `SimdJsonError::Utf8Error`/`StringError` from `parser_parse`).
        // Re-validating here showed up as ~11% of CPU in profiling, all of it
        // redundant.
        Ok(unsafe { str::from_utf8_unchecked(&self.string_buf[start..end]) })
    }

    /// Decode the `i64` value at the current position.
    ///
    /// The actual integer is stored in the *next* tape word (reinterpreted as
    /// `i64` bits).
    #[inline]
    pub fn get_int64(&self) -> Result<i64> {
        debug_assert_eq!(
            self.tape_type(),
            Some(TapeType::Int64),
            "get_int64 called on non-int64 node"
        );
        let next = self.pos + 1;
        ensure!(
            next < self.tape.len(),
            MissingValueWordSnafu { pos: self.pos }
        );
        Ok(self.tape[next] as i64)
    }

    /// Decode the `u64` value at the current position.
    #[inline]
    pub fn get_uint64(&self) -> Result<u64> {
        debug_assert_eq!(
            self.tape_type(),
            Some(TapeType::Uint64),
            "get_uint64 called on non-uint64 node"
        );
        let next = self.pos + 1;
        ensure!(
            next < self.tape.len(),
            MissingValueWordSnafu { pos: self.pos }
        );
        Ok(self.tape[next])
    }

    /// Decode the `f64` value at the current position.
    #[inline]
    pub fn get_double(&self) -> Result<f64> {
        debug_assert_eq!(
            self.tape_type(),
            Some(TapeType::Double),
            "get_double called on non-double node"
        );
        let next = self.pos + 1;
        ensure!(
            next < self.tape.len(),
            MissingValueWordSnafu { pos: self.pos }
        );
        Ok(f64::from_bits(self.tape[next]))
    }

    /// Parse the value at the current tape position into a zero-copy `Value`.
    ///
    /// After a successful call the cursor is advanced past the parsed value,
    /// ready to decode the next element.
    pub fn parse_value(&mut self) -> Result<Value<'a>> {
        ensure!(
            self.pos < self.tape.len(),
            OutOfBoundsSnafu {
                pos: self.pos,
                len: self.tape.len(),
            }
        );

        match self.tape_type() {
            Some(TapeType::Root) => {
                // Skip the root node and parse the document value.
                self.skip(1);
                self.parse_value()
            }
            Some(TapeType::Null) => {
                self.skip(1);
                Ok(Value::Null)
            }
            Some(TapeType::True) => {
                self.skip(1);
                Ok(Value::Bool(true))
            }
            Some(TapeType::False) => {
                self.skip(1);
                Ok(Value::Bool(false))
            }
            Some(TapeType::Int64) => {
                let v = self.get_int64()?;
                self.skip(2); // type word + value word
                Ok(Value::Int64(v))
            }
            Some(TapeType::Uint64) => {
                let v = self.get_uint64()?;
                self.skip(2);
                Ok(Value::Uint64(v))
            }
            Some(TapeType::Double) => {
                let v = self.get_double()?;
                self.skip(2);
                Ok(Value::Double(v))
            }
            Some(TapeType::String) => {
                let s = self.get_string()?;
                self.skip(1);
                Ok(Value::String(s))
            }
            Some(TapeType::StartArray) => {
                // scope_close_idx = position of the matching `]` node
                let end_idx = self.scope_close_idx();
                self.skip(1); // move past `[`
                let mut arr = Vec::new();
                while self.pos < end_idx {
                    let v = self.parse_value()?;
                    arr.push(v);
                }
                // Now at the `]` node; skip it.
                self.skip(1);
                Ok(Value::Array(arr))
            }
            Some(TapeType::StartObject) => {
                // scope_close_idx = position of the matching `}` node
                let end_idx = self.scope_close_idx();
                self.skip(1); // move past `{`
                let mut obj = Vec::new();
                while self.pos < end_idx {
                    // Each field is: key (String node) then value
                    let key = self.get_string()?;
                    self.skip(1); // past key string word
                    let value = self.parse_value()?;
                    obj.push((key, value));
                }
                // Now at the `}` node; skip it.
                self.skip(1);
                Ok(Value::Object(obj))
            }
            Some(t) => UnexpectedTapeTypeSnafu {
                found: t as u8,
                pos: self.pos,
            }
            .fail(),
            None => UnknownTapeTypeSnafu {
                tag: (self.current_word() >> 56) as u8,
                pos: self.pos,
            }
            .fail(),
        }
    }

    /// Visit the tape using a [`TapeVisitor`].
    ///
    /// This is the low-level API for custom traversal. The visitor receives
    /// structured callbacks as the tape is walked.
    pub fn visit<V: TapeVisitor<'a>>(&mut self, visitor: &mut V) -> Result<V::Output> {
        ensure!(
            self.pos < self.tape.len(),
            OutOfBoundsSnafu {
                pos: self.pos,
                len: self.tape.len(),
            }
        );

        match self.tape_type() {
            Some(TapeType::Root) => {
                self.skip(1);
                self.visit(visitor)
            }
            Some(TapeType::Null) => {
                self.skip(1);
                visitor.visit_null()
            }
            Some(TapeType::True) => {
                self.skip(1);
                visitor.visit_bool(true)
            }
            Some(TapeType::False) => {
                self.skip(1);
                visitor.visit_bool(false)
            }
            Some(TapeType::Int64) => {
                let v = self.get_int64()?;
                self.skip(2);
                visitor.visit_i64(v)
            }
            Some(TapeType::Uint64) => {
                let v = self.get_uint64()?;
                self.skip(2);
                visitor.visit_u64(v)
            }
            Some(TapeType::Double) => {
                let v = self.get_double()?;
                self.skip(2);
                visitor.visit_f64(v)
            }
            Some(TapeType::String) => {
                let s = self.get_string()?;
                self.skip(1);
                visitor.visit_str(s)
            }
            Some(TapeType::StartArray) => {
                let end_idx = self.scope_close_idx();
                self.skip(1);
                let result = visitor.visit_array(self, end_idx)?;
                self.skip(1); // past `]`
                Ok(result)
            }
            Some(TapeType::StartObject) => {
                let end_idx = self.scope_close_idx();
                self.skip(1);
                let result = visitor.visit_object(self, end_idx)?;
                self.skip(1); // past `}`
                Ok(result)
            }
            Some(t) => UnexpectedTapeTypeSnafu {
                found: t as u8,
                pos: self.pos,
            }
            .fail(),
            None => UnknownTapeTypeSnafu {
                tag: (self.current_word() >> 56) as u8,
                pos: self.pos,
            }
            .fail(),
        }
    }
}

/// Callback-based visitor interface for the simdjson tape.
///
/// Implement this trait to build custom data structures from the tape without
/// allocating intermediate `Value` nodes.
pub trait TapeVisitor<'a> {
    /// The value produced after visiting a complete JSON value.
    type Output;

    fn visit_null(&mut self) -> Result<Self::Output>;
    fn visit_bool(&mut self, v: bool) -> Result<Self::Output>;
    fn visit_i64(&mut self, v: i64) -> Result<Self::Output>;
    fn visit_u64(&mut self, v: u64) -> Result<Self::Output>;
    fn visit_f64(&mut self, v: f64) -> Result<Self::Output>;

    /// Visit a string value. The `s` borrow is tied to the tape lifetime `'a`.
    fn visit_str(&mut self, s: &'a str) -> Result<Self::Output>;

    /// Visit an array. The implementor should call `tape.visit(self)` for each
    /// element while `tape.pos() < end_idx`.
    fn visit_array(&mut self, tape: &mut TapeRef<'a>, end_idx: usize) -> Result<Self::Output>;

    /// Visit an object. Fields are key–value pairs; the implementor should
    /// read key strings and call `tape.visit(self)` for values while
    /// `tape.pos() < end_idx`.
    fn visit_object(&mut self, tape: &mut TapeRef<'a>, end_idx: usize) -> Result<Self::Output>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tape_type_decoding() {
        let words: &[u64] = &[
            (b'r' as u64) << 56 | 3,
            (b'n' as u64) << 56,
            (b'r' as u64) << 56,
        ];
        let mut r = TapeRef::new(words, &[]);
        assert_eq!(r.tape_type(), Some(TapeType::Root));
        r.skip(1);
        assert_eq!(r.tape_type(), Some(TapeType::Null));
    }

    #[test]
    fn tape_bool_types() {
        let words: &[u64] = &[(b't' as u64) << 56, (b'f' as u64) << 56];
        let r = TapeRef::new(words, &[]);
        assert_eq!(r.tape_type(), Some(TapeType::True));
        let mut r2 = r;
        r2.skip(1);
        assert_eq!(r2.tape_type(), Some(TapeType::False));
    }

    #[test]
    fn tape_type_from_tape_word() {
        assert_eq!(
            TapeType::from_tape_word((b'r' as u64) << 56),
            Some(TapeType::Root)
        );
        assert_eq!(
            TapeType::from_tape_word((b'{' as u64) << 56),
            Some(TapeType::StartObject)
        );
        assert_eq!(
            TapeType::from_tape_word((b'}' as u64) << 56),
            Some(TapeType::EndObject)
        );
        assert_eq!(
            TapeType::from_tape_word((b'[' as u64) << 56),
            Some(TapeType::StartArray)
        );
        assert_eq!(
            TapeType::from_tape_word((b']' as u64) << 56),
            Some(TapeType::EndArray)
        );
        assert_eq!(
            TapeType::from_tape_word((b'"' as u64) << 56),
            Some(TapeType::String)
        );
        assert_eq!(
            TapeType::from_tape_word((b'l' as u64) << 56),
            Some(TapeType::Int64)
        );
        assert_eq!(
            TapeType::from_tape_word((b'u' as u64) << 56),
            Some(TapeType::Uint64)
        );
        assert_eq!(
            TapeType::from_tape_word((b'd' as u64) << 56),
            Some(TapeType::Double)
        );
        assert_eq!(
            TapeType::from_tape_word((b't' as u64) << 56),
            Some(TapeType::True)
        );
        assert_eq!(
            TapeType::from_tape_word((b'f' as u64) << 56),
            Some(TapeType::False)
        );
        assert_eq!(
            TapeType::from_tape_word((b'n' as u64) << 56),
            Some(TapeType::Null)
        );
        assert_eq!(TapeType::from_tape_word(0xAA00_0000_0000_0000), None);
    }

    /// A simple visitor that counts nodes.
    #[cfg(feature = "serde")]
    struct NodeCounter {
        count: usize,
    }

    #[cfg(feature = "serde")]
    impl<'a> TapeVisitor<'a> for NodeCounter {
        type Output = usize;

        fn visit_null(&mut self) -> Result<usize> {
            self.count += 1;
            Ok(self.count)
        }

        fn visit_bool(&mut self, _v: bool) -> Result<usize> {
            self.count += 1;
            Ok(self.count)
        }

        fn visit_i64(&mut self, _v: i64) -> Result<usize> {
            self.count += 1;
            Ok(self.count)
        }

        fn visit_u64(&mut self, _v: u64) -> Result<usize> {
            self.count += 1;
            Ok(self.count)
        }

        fn visit_f64(&mut self, _v: f64) -> Result<usize> {
            self.count += 1;
            Ok(self.count)
        }

        fn visit_str(&mut self, _s: &'a str) -> Result<usize> {
            self.count += 1;
            Ok(self.count)
        }

        fn visit_array(&mut self, tape: &mut TapeRef<'a>, end_idx: usize) -> Result<usize> {
            while tape.pos() < end_idx {
                tape.visit(self)?;
            }
            Ok(self.count)
        }

        fn visit_object(&mut self, tape: &mut TapeRef<'a>, end_idx: usize) -> Result<usize> {
            while tape.pos() < end_idx {
                // Key
                tape.visit(self)?;
                // Value
                tape.visit(self)?;
            }
            Ok(self.count)
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn tape_visitor_counts_nodes() {
        use crate::dom::Parser;
        let mut p = Parser::default();
        // {"a": 1, "b": [true, false]} → nodes: "a", 1, "b", true, false = 5
        let mut tape = p.parse_str(r#"{"a":1,"b":[true,false]}"#).unwrap();
        let mut counter = NodeCounter { count: 0 };
        tape.visit(&mut counter).unwrap();
        assert_eq!(counter.count, 5);
    }

    #[test]
    fn parse_value_empty_array() {
        use crate::dom::Parser;
        let mut p = Parser::default();
        let mut tape = p.parse_str("[]").unwrap();
        let v = tape.parse_value().unwrap();
        assert_eq!(v, Value::Array(vec![]));
    }

    #[test]
    fn parse_value_empty_object() {
        use crate::dom::Parser;
        let mut p = Parser::default();
        let mut tape = p.parse_str("{}").unwrap();
        let v = tape.parse_value().unwrap();
        assert_eq!(v, Value::Object(vec![]));
    }

    #[test]
    fn parse_value_uint64() {
        use crate::dom::Parser;
        let mut p = Parser::default();
        // Large positive integer that simdjson stores as u64
        let mut tape = p.parse_str("18446744073709551615").unwrap(); // u64::MAX
        let v = tape.parse_value().unwrap();
        assert_eq!(v.as_u64(), Some(u64::MAX));
    }
}
