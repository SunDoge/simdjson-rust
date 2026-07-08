//! `serde::Deserializer` implementation backed by the simdjson tape.
//!
//! This allows you to deserialize any `serde::Deserialize` type directly from
//! simdjson's DOM tape without first converting to an intermediate AST.
//!
//! # Example
//!
//! ```rust
//! use serde::Deserialize;
//! use simdjson_rust::serde::from_str;
//!
//! #[derive(Debug, Deserialize, PartialEq)]
//! struct Point {
//!     x: f64,
//!     y: f64,
//! }
//!
//! let p: Point = from_str(r#"{"x": 1.0, "y": 2.0}"#).unwrap();
//! assert_eq!(p, Point { x: 1.0, y: 2.0 });
//! ```

use serde::{
    Deserialize,
    de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor},
};
use snafu::prelude::*;

use crate::{
    dom::Parser,
    tape::{self, TapeRef, TapeType},
};

/// The error type returned by the Serde deserializer.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("serde error: {message}"))]
    Custom { message: String },

    /// A malformed or unsupported tape state was encountered while
    /// deserializing. Carries the structured [`tape::Error`] so callers can
    /// match on the concrete failure.
    #[snafu(transparent)]
    Tape {
        #[snafu(backtrace)]
        source: tape::Error,
    },

    #[snafu(display("dom error: {source}"))]
    Dom {
        #[snafu(backtrace)]
        source: crate::dom::Error,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<crate::dom::Error> for Error {
    fn from(source: crate::dom::Error) -> Self {
        Error::Dom { source }
    }
}

impl de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Custom {
            message: msg.to_string(),
        }
    }
}

/// Deserialize `T` from a JSON string using the simdjson DOM tape.
pub fn from_str<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T> {
    let mut parser = Parser::default();
    let tape = parser.parse_str(json)?;
    let mut de = TapeDeserializer { tape };
    T::deserialize(&mut de)
}

/// Deserialize `T` from a JSON byte slice using the simdjson DOM tape.
pub fn from_bytes<T: for<'de> Deserialize<'de>>(json: &[u8]) -> Result<T> {
    let mut parser = Parser::default();
    let tape = parser.parse_bytes(json)?;
    let mut de = TapeDeserializer { tape };
    T::deserialize(&mut de)
}

/// Deserialize `T` directly from a parsed `TapeRef` cursor.
///
/// This allows you to reuse the `Parser` instance across multiple documents,
/// which is highly recommended for performance.
pub fn from_tape<'de, T: Deserialize<'de>>(tape: TapeRef<'de>) -> Result<T> {
    let mut de = TapeDeserializer { tape };
    T::deserialize(&mut de)
}

/// A `serde::Deserializer` that reads directly from the simdjson tape.
///
/// The lifetime `'de` is tied to the parser that owns the tape, enabling
/// zero-copy `&str` deserialization.
pub struct TapeDeserializer<'de> {
    tape: TapeRef<'de>,
}

/// Internal helper: deserialize a single JSON value from a mutable tape cursor.
///
/// This is the workhorse used by all `SeqAccess`/`MapAccess`/`VariantAccess`
/// impls so they can share the same tape cursor.
///
/// Dispatches on the raw tag byte of the current tape word rather than going
/// through `Option<TapeType>`, avoiding an enum round-trip and a second match
/// on every node.
#[inline]
fn deserialize_value<'de, V: Visitor<'de>>(
    tape: &mut TapeRef<'de>,
    visitor: V,
) -> Result<V::Value> {
    if tape.pos >= tape.tape.len() {
        return Err(tape::OutOfBoundsSnafu {
            pos: tape.pos,
            len: tape.tape.len(),
        }
        .build()
        .into());
    }
    let word = tape.current_word_public();
    let tag = (word >> 56) as u8;
    match tag {
        b'r' => {
            tape.skip(1);
            deserialize_value(tape, visitor)
        }
        b'n' => {
            tape.skip(1);
            visitor.visit_unit()
        }
        b't' => {
            tape.skip(1);
            visitor.visit_bool(true)
        }
        b'f' => {
            tape.skip(1);
            visitor.visit_bool(false)
        }
        b'l' => {
            let v = tape.get_int64()?;
            tape.skip(2);
            visitor.visit_i64(v)
        }
        b'u' => {
            let v = tape.get_uint64()?;
            tape.skip(2);
            visitor.visit_u64(v)
        }
        b'd' => {
            let v = tape.get_double()?;
            tape.skip(2);
            visitor.visit_f64(v)
        }
        b'"' => {
            let s: &'de str = tape.get_string()?;
            tape.skip(1);
            visitor.visit_borrowed_str(s)
        }
        b'[' => {
            let end_idx = tape.scope_close_idx();
            let count = tape.scope_count();
            tape.skip(1); // past `[`
            let result = visitor.visit_seq(TapeSeqAccess {
                tape,
                end_idx,
                count,
            })?;
            tape.skip(1); // past `]`
            Ok(result)
        }
        b'{' => {
            let end_idx = tape.scope_close_idx();
            tape.skip(1); // past `{`
            let result = visitor.visit_map(TapeMapAccess { tape, end_idx })?;
            tape.skip(1); // past `}`
            Ok(result)
        }
        _ => match TapeType::from_tape_word(word) {
            // Recognised tag that isn't a valid value position (e.g. a stray
            // `}` / `]`). `tag` is known, so this is unexpected, not unknown.
            Some(_) => Err(tape::UnexpectedTapeTypeSnafu {
                found: tag,
                pos: tape.pos(),
            }
            .build()
            .into()),
            None => Err(tape::UnknownTapeTypeSnafu {
                tag,
                pos: tape.pos(),
            }
            .build()
            .into()),
        },
    }
}

/// Skip a complete JSON value on the tape without deserializing it.
fn skip_value(tape: &mut TapeRef<'_>) -> Result<()> {
    match tape.tape_type() {
        Some(TapeType::Null) | Some(TapeType::True) | Some(TapeType::False) => {
            tape.skip(1);
        }
        Some(TapeType::Int64) | Some(TapeType::Uint64) | Some(TapeType::Double) => {
            tape.skip(2);
        }
        Some(TapeType::String) => {
            tape.skip(1);
        }
        Some(TapeType::StartArray) | Some(TapeType::StartObject) => {
            // scope_close_idx = position of the closing `]` / `}` entry
            let end_idx = tape.scope_close_idx();
            tape.seek_public(end_idx);
            tape.skip(1); // past the closing bracket
        }
        Some(t) => {
            return Err(tape::UnexpectedTapeTypeSnafu {
                found: t as u8,
                pos: tape.pos(),
            }
            .build()
            .into());
        }
        None => {
            return Err(tape::UnknownTapeTypeSnafu {
                tag: (tape.current_word_public() >> 56) as u8,
                pos: tape.pos(),
            }
            .build()
            .into());
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// SeqAccess for arrays
// ---------------------------------------------------------------------------

struct TapeSeqAccess<'a, 'de: 'a> {
    tape: &'a mut TapeRef<'de>,
    end_idx: usize,
    /// Element count read from the `[` tape word, used as a `size_hint` so
    /// serde can pre-allocate the destination `Vec` and avoid a grow storm.
    count: usize,
}

impl<'a, 'de> SeqAccess<'de> for TapeSeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        if self.tape.pos() >= self.end_idx {
            return Ok(None);
        }
        seed.deserialize(&mut TapeRefDeserializer { tape: self.tape })
            .map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.count)
    }
}

// ---------------------------------------------------------------------------
// MapAccess for objects
// ---------------------------------------------------------------------------

struct TapeMapAccess<'a, 'de: 'a> {
    tape: &'a mut TapeRef<'de>,
    end_idx: usize,
}

impl<'a, 'de> MapAccess<'de> for TapeMapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        if self.tape.pos() >= self.end_idx {
            return Ok(None);
        }
        // Keys in object tape are always String nodes.
        let word = self.tape.current_word_public();
        let tag = (word >> 56) as u8;
        if tag != b'"' {
            return Err(tape::UnexpectedTapeTypeSnafu {
                found: tag,
                pos: self.tape.pos(),
            }
            .build()
            .into());
        }
        let key: &'de str = self.tape.get_string()?;
        self.tape.skip(1);
        let key_deserializer = de::value::BorrowedStrDeserializer::<Error>::new(key);
        seed.deserialize(key_deserializer).map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        seed.deserialize(&mut TapeRefDeserializer { tape: self.tape })
    }
}

// ---------------------------------------------------------------------------
// EnumAccess / VariantAccess
// ---------------------------------------------------------------------------

struct TapeEnumAccess<'a, 'de: 'a> {
    tape: &'a mut TapeRef<'de>,
    end_idx: usize,
}

impl<'a, 'de> EnumAccess<'de> for TapeEnumAccess<'a, 'de> {
    type Error = Error;
    type Variant = TapeVariantAccess<'a, 'de>;

    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant)> {
        let word = self.tape.current_word_public();
        let tag = (word >> 56) as u8;
        if tag != b'"' {
            return Err(tape::UnexpectedTapeTypeSnafu {
                found: tag,
                pos: self.tape.pos(),
            }
            .build()
            .into());
        }
        let name: &'de str = self.tape.get_string()?;
        self.tape.skip(1);
        let name_deserializer = de::value::BorrowedStrDeserializer::<Error>::new(name);
        let val = seed.deserialize(name_deserializer)?;
        Ok((
            val,
            TapeVariantAccess {
                tape: self.tape,
                end_idx: self.end_idx,
            },
        ))
    }
}

struct TapeVariantAccess<'a, 'de: 'a> {
    tape: &'a mut TapeRef<'de>,
    #[allow(dead_code)]
    end_idx: usize,
}

impl<'a, 'de> VariantAccess<'de> for TapeVariantAccess<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value> {
        seed.deserialize(&mut TapeRefDeserializer { tape: self.tape })
    }

    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        deserialize_value(self.tape, visitor)
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        deserialize_value(self.tape, visitor)
    }
}

// ---------------------------------------------------------------------------
// TapeRefDeserializer - thin wrapper that delegates to `deserialize_value`
// ---------------------------------------------------------------------------

/// A thin `serde::Deserializer` that mutates a shared `TapeRef` cursor.
///
/// This is used by `SeqAccess`, `MapAccess`, and `VariantAccess` to
/// delegate element deserialization.
struct TapeRefDeserializer<'a, 'de: 'a> {
    tape: &'a mut TapeRef<'de>,
}

macro_rules! forward_to_deserialize_value {
    ($($fn:ident)*) => {
        $(
            fn $fn<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
                deserialize_value(self.tape, visitor)
            }
        )*
    };
}

impl<'a, 'de> de::Deserializer<'de> for &mut TapeRefDeserializer<'a, 'de> {
    type Error = Error;

    // All other methods can forward to `deserialize_any`.
    forward_to_deserialize_value! {
        deserialize_bool
        deserialize_i8 deserialize_i16 deserialize_i32 deserialize_i64
        deserialize_u8 deserialize_u16 deserialize_u32 deserialize_u64
        deserialize_f32 deserialize_f64
        deserialize_char deserialize_str deserialize_string
        deserialize_bytes deserialize_byte_buf
        deserialize_unit
        deserialize_seq deserialize_map
        deserialize_identifier
    }

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        deserialize_value(self.tape, visitor)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if self.tape.tape_type() == Some(TapeType::Null) {
            self.tape.skip(1);
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        match self.tape.tape_type() {
            Some(TapeType::String) => {
                let s: &'de str = self.tape.get_string()?;
                self.tape.skip(1);
                let enum_deserializer = de::value::StrDeserializer::<Error>::new(s);
                visitor.visit_enum(enum_deserializer)
            }
            Some(TapeType::StartObject) => {
                let end_idx = self.tape.scope_close_idx();
                self.tape.skip(1); // past `{`
                let result = visitor.visit_enum(TapeEnumAccess {
                    tape: self.tape,
                    end_idx,
                })?;
                self.tape.skip(1); // past `}`
                Ok(result)
            }
            _ => deserialize_value(self.tape, visitor),
        }
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        skip_value(self.tape)?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        deserialize_value(self.tape, visitor)
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        deserialize_value(self.tape, visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        deserialize_value(self.tape, visitor)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        deserialize_value(self.tape, visitor)
    }
}

// ---------------------------------------------------------------------------
// TapeDeserializer (top-level entry point)
// ---------------------------------------------------------------------------

impl<'de> de::Deserializer<'de> for &mut TapeDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // Skip root if present.
        if self.tape.tape_type() == Some(TapeType::Root) {
            self.tape.skip(1);
        }
        deserialize_value(&mut self.tape, visitor)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if self.tape.tape_type() == Some(TapeType::Root) {
            self.tape.skip(1);
        }
        if self.tape.tape_type() == Some(TapeType::Null) {
            self.tape.skip(1);
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        if self.tape.tape_type() == Some(TapeType::Root) {
            self.tape.skip(1);
        }
        match self.tape.tape_type() {
            Some(TapeType::String) => {
                let s: &'de str = self.tape.get_string()?;
                self.tape.skip(1);
                let enum_deserializer = de::value::StrDeserializer::<Error>::new(s);
                visitor.visit_enum(enum_deserializer)
            }
            Some(TapeType::StartObject) => {
                let end_idx = self.tape.scope_close_idx();
                self.tape.skip(1);
                let result = visitor.visit_enum(TapeEnumAccess {
                    tape: &mut self.tape,
                    end_idx,
                })?;
                self.tape.skip(1); // past `}`
                Ok(result)
            }
            _ => {
                let t = &mut self.tape;
                deserialize_value(t, visitor)
            }
        }
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if self.tape.tape_type() == Some(TapeType::Root) {
            self.tape.skip(1);
        }
        skip_value(&mut self.tape)?;
        visitor.visit_unit()
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Nested {
        name: String,
        value: i64,
        tags: Vec<String>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "snake_case")]
    enum Color {
        Red,
        Green,
        Blue,
        Custom { r: u8, g: u8, b: u8 },
    }

    #[test]
    fn deserialize_struct() {
        let p: Point = from_str(r#"{"x": 1.5, "y": -2.0}"#).unwrap();
        assert_eq!(p, Point { x: 1.5, y: -2.0 });
    }

    #[test]
    fn deserialize_nested() {
        let n: Nested = from_str(r#"{"name":"hello","value":42,"tags":["a","b","c"]}"#).unwrap();
        assert_eq!(n.name, "hello");
        assert_eq!(n.value, 42);
        assert_eq!(n.tags, vec!["a", "b", "c"]);
    }

    #[test]
    fn deserialize_vec_of_ints() {
        let v: Vec<i64> = from_str("[1, 2, 3, -4]").unwrap();
        assert_eq!(v, vec![1, 2, 3, -4]);
    }

    #[test]
    fn deserialize_option_some() {
        let v: Option<i64> = from_str("42").unwrap();
        assert_eq!(v, Some(42));
    }

    #[test]
    fn deserialize_option_none() {
        let v: Option<i64> = from_str("null").unwrap();
        assert_eq!(v, None);
    }

    #[test]
    fn deserialize_bool_true() {
        assert!(from_str::<bool>("true").unwrap());
    }

    #[test]
    fn deserialize_bool_false() {
        assert!(!from_str::<bool>("false").unwrap());
    }

    #[test]
    fn deserialize_unit_enum() {
        let c: Color = from_str(r#""red""#).unwrap();
        assert_eq!(c, Color::Red);
    }

    #[test]
    fn deserialize_struct_enum() {
        let c: Color = from_str(r#"{"custom": {"r": 10, "g": 20, "b": 30}}"#).unwrap();
        assert_eq!(
            c,
            Color::Custom {
                r: 10,
                g: 20,
                b: 30
            }
        );
    }

    #[test]
    fn deserialize_f64() {
        let v: f64 = from_str("1.5").unwrap();
        assert_eq!(v, 1.5_f64);
    }

    #[test]
    fn deserialize_string() {
        let s: String = from_str(r#""hello world""#).unwrap();
        assert_eq!(s, "hello world");
    }

    #[test]
    fn deserialize_nested_arrays() {
        let v: Vec<Vec<i64>> = from_str("[[1,2],[3,4],[5]]").unwrap();
        assert_eq!(v, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct LargeObject {
        id: u64,
        name: String,
        active: bool,
        score: f64,
        tags: Vec<String>,
        meta: Option<String>,
    }

    #[test]
    fn deserialize_large_object() {
        let json = r#"{
            "id": 12345,
            "name": "simdjson",
            "active": true,
            "score": 9.99,
            "tags": ["fast", "safe", "rust"],
            "meta": null
        }"#;
        let v: LargeObject = from_str(json).unwrap();
        assert_eq!(v.id, 12345);
        assert_eq!(v.name, "simdjson");
        assert!(v.active);
        assert!((v.score - 9.99).abs() < 1e-6);
        assert_eq!(v.tags[0], "fast");
        assert!(v.meta.is_none());
    }
}
