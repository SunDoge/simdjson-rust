#![allow(clippy::missing_safety_doc)]

#[cxx::bridge(namespace = "simdjson_sys::dom")]
pub mod dom_ffi {
    struct TapeView<'a> {
        tape: &'a [u64],
        string_buf: &'a [u8],
    }

    unsafe extern "C++" {
        include!("simdjson.h");
        include!("simdjson_dom_bridge.h");

        #[namespace = "simdjson::dom"]
        type parser;

        fn parser_new(max_capacity: usize) -> UniquePtr<parser>;
        fn parser_parse(parser: Pin<&mut parser>, json: &[u8], realloc_if_needed: bool) -> i32;
        fn parser_get_tape_view(parser: &parser) -> TapeView<'_>;
    }
}

pub const SIMDJSON_PADDING: usize = 64;
pub const SIMDJSON_MAXSIZE_BYTES: usize = 0xFFFFFFFF;
pub const DEFAULT_BATCH_SIZE: usize = 1000000;

pub struct Parser {
    inner: cxx::UniquePtr<dom_ffi::parser>,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new(SIMDJSON_MAXSIZE_BYTES)
    }
}

impl Parser {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            inner: dom_ffi::parser_new(max_capacity),
        }
    }

    pub fn parse_string(&mut self, json: &mut String) -> i32 {
        if json.capacity() < json.len() + SIMDJSON_PADDING {
            json.reserve(SIMDJSON_PADDING);
        }

        dom_ffi::parser_parse(self.inner.pin_mut(), json.as_bytes(), false)
    }

    pub fn get_tape_view(&self) -> dom_ffi::TapeView<'_> {
        dom_ffi::parser_get_tape_view(self.inner.as_ref().expect("parser must not be null"))
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn parser_parses_string_and_exposes_tape_view() {
        let mut parser = Parser::default();
        let mut json = String::from(r#"{"answer":42,"message":"ok"}"#);

        assert_eq!(parser.parse_string(&mut json), 0);

        let view = parser.get_tape_view();
        assert!(!view.tape.is_empty());
        assert_eq!(view.tape[0] >> 56, u64::from(b'r'));
        assert!(!view.string_buf.is_empty());
    }
}
