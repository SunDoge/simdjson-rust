//! High-level DOM parser wrapping `simdjson_sys`.
//!
//! This module provides a safe, ergonomic Rust API over the simdjson DOM
//! parser. After parsing, the result is available as a zero-copy
//! [`Value`](crate::tape::Value), or as a raw
//! [`TapeRef`](crate::tape::TapeRef) for custom traversal.

use simdjson_sys::{SIMDJSON_PADDING, dom_ffi};
use snafu::prelude::*;

use crate::{error::SimdJsonError, tape::TapeRef};

/// The error type returned by the DOM parser.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("simdjson error: {source}"))]
    SimdJson { source: SimdJsonError },

    #[snafu(display(
        "the JSON input buffer must have at least {} bytes of extra capacity",
        SIMDJSON_PADDING
    ))]
    InsufficientPadding,

    #[snafu(display("tape error: {source}"))]
    Tape {
        #[snafu(backtrace)]
        source: crate::tape::Error,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<crate::tape::Error> for Error {
    fn from(source: crate::tape::Error) -> Self {
        Error::Tape { source }
    }
}

/// A simdjson DOM parser.
///
/// The parser owns an internal buffer. After calling one of the `parse*`
/// methods the tape and string buffer remain valid until the next parse call.
///
/// # Example
///
/// ```rust
/// use simdjson_rust::dom::Parser;
///
/// let mut parser = Parser::default();
/// let value = parser.parse_str(r#"{"hello": "world"}"#).unwrap();
/// println!("{value:?}");
/// ```
pub struct Parser {
    inner: cxx::UniquePtr<dom_ffi::parser>,
    /// Reuse the padded buffer to avoid reallocation on every parse.
    padded: Vec<u8>,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new(simdjson_sys::SIMDJSON_MAXSIZE_BYTES)
    }
}

impl Parser {
    /// Create a parser that can handle documents up to `max_capacity` bytes.
    pub fn new(max_capacity: usize) -> Self {
        Self {
            inner: dom_ffi::parser_new(max_capacity),
            padded: Vec::new(),
        }
    }

    /// Parse a JSON byte slice.
    ///
    /// The slice must have at least [`SIMDJSON_PADDING`] bytes of extra
    /// capacity beyond its length. Use [`parse_str`](Self::parse_str) or
    /// [`parse_padded`](Self::parse_padded) for convenience variants that
    /// handle padding automatically.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SimdJson`] if the parser rejects the input.
    pub fn parse_bytes_with_padding(&mut self, json: &[u8]) -> Result<TapeRef<'_>> {
        let code = dom_ffi::parser_parse(self.inner.pin_mut(), json, false);
        SimdJsonError::check_code(code).context(SimdJsonSnafu)?;
        Ok(self.tape_ref())
    }

    /// Parse a JSON string, automatically adding the required padding.
    ///
    /// The padding is added on a private copy of the string, so the original
    /// `json` argument does not need to be mutable.
    pub fn parse_str(&mut self, json: &str) -> Result<TapeRef<'_>> {
        self.parse_bytes(json.as_bytes())
    }

    /// Parse a JSON byte slice, automatically adding the required padding.
    pub fn parse_bytes(&mut self, json: &[u8]) -> Result<TapeRef<'_>> {
        // simdjson reads up to SIMDJSON_PADDING bytes past the end of the
        // slice pointer when realloc_if_needed=false. We allocate a buffer
        // with that extra space, copy the JSON into it, zero the tail, then
        // pass a slice of the original length. simdjson uses the slice length
        // as the document length but reads the zero-padded tail unsafely.
        let json_len = json.len();
        self.padded.clear();
        self.padded.reserve(json_len + SIMDJSON_PADDING);
        self.padded.extend_from_slice(json);
        // Zero-fill the SIMDJSON_PADDING extra bytes that simdjson will read.
        self.padded
            .extend(std::iter::repeat_n(0u8, SIMDJSON_PADDING));

        // Pass only the JSON length; the zero tail is read past the slice end.
        let code = dom_ffi::parser_parse(self.inner.pin_mut(), &self.padded[..json_len], false);
        SimdJsonError::check_code(code).context(SimdJsonSnafu)?;
        Ok(self.tape_ref())
    }

    /// Parse a JSON string that is already padded (has `>= SIMDJSON_PADDING`
    /// bytes of extra space in its allocation).
    ///
    /// The string must have `s.capacity() >= s.len() + SIMDJSON_PADDING`.
    pub fn parse_padded(&mut self, json: &mut String) -> Result<TapeRef<'_>> {
        ensure!(
            json.capacity() >= json.len() + SIMDJSON_PADDING,
            InsufficientPaddingSnafu
        );
        let code = dom_ffi::parser_parse(self.inner.pin_mut(), json.as_bytes(), false);
        SimdJsonError::check_code(code).context(SimdJsonSnafu)?;
        Ok(self.tape_ref())
    }

    /// Returns a reference to the current tape.
    ///
    /// This is only meaningful after a successful `parse*` call.
    pub fn tape_ref(&self) -> TapeRef<'_> {
        let view =
            dom_ffi::parser_get_tape_view(self.inner.as_ref().expect("parser must not be null"));
        TapeRef::new(view.tape, view.string_buf)
    }

    /// Parse a JSON string and return a zero-copy `Value`.
    pub fn parse_to_value(&mut self, json: &str) -> Result<crate::tape::Value<'_>> {
        let mut tape = self.parse_str(json)?;
        Ok(tape.parse_value()?)
    }

    /// Parse a JSON byte slice and return a zero-copy `Value`.
    pub fn parse_bytes_to_value(&mut self, json: &[u8]) -> Result<crate::tape::Value<'_>> {
        let mut tape = self.parse_bytes(json)?;
        Ok(tape.parse_value()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_null_tape() -> Result<()> {
        let mut p = Parser::default();
        let tape = p.parse_str("null")?;
        // First word is always the root 'r' node
        assert_eq!(tape.tape_type(), Some(crate::tape::TapeType::Root));
        Ok(())
    }

    #[test]
    fn parse_object_tape() -> Result<()> {
        let mut p = Parser::default();
        let tape = p.parse_str(r#"{"answer":42}"#)?;
        // First word is root 'r'
        assert_eq!(tape.tape_type(), Some(crate::tape::TapeType::Root));
        Ok(())
    }

    #[test]
    fn parse_to_value_works() -> Result<()> {
        let mut p = Parser::default();
        let v = p.parse_to_value(r#"{"answer":42,"ok":true,"name":"rust"}"#)?;
        assert_eq!(v.get("answer").unwrap().as_i64(), Some(42));
        assert_eq!(v.get("ok").unwrap().as_bool(), Some(true));
        assert_eq!(v.get("name").unwrap().as_str(), Some("rust"));
        Ok(())
    }
}
