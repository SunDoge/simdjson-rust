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
        /// Parse a document, optionally copying it into padded storage.
        ///
        /// # Safety
        ///
        /// With `realloc_if_needed = false`, the allocation must provide 64
        /// initialized readable bytes beyond `json` for the duration of the call.
        // CXX-generated declarations trigger this lint despite the docs above.
        #[allow(clippy::missing_safety_doc)]
        unsafe fn parser_parse(
            parser: Pin<&mut parser>,
            json: &[u8],
            realloc_if_needed: bool,
        ) -> i32;
        /// Borrow the initialized tape and string buffer.
        ///
        /// # Safety
        ///
        /// The last parse must have succeeded, with no subsequent mutation.
        #[allow(clippy::missing_safety_doc)]
        unsafe fn parser_get_tape_view(parser: &parser) -> TapeView<'_>;
    }
}

/// Low-level minification without JSON validation or input padding.
#[cxx::bridge(namespace = "simdjson_sys")]
pub mod minify_ffi {
    unsafe extern "C++" {
        include!("simdjson_dom_bridge.h");

        /// Remove JSON whitespace outside strings. `output.len()` must be at
        /// least `json.len()`; otherwise returns CAPACITY without writing.
        /// `written` is zero on error; output contents on error are unspecified.
        /// This does not validate JSON syntax or UTF-8.
        fn minify(json: &[u8], output: &mut [u8], written: &mut usize) -> i32;
    }
}

pub const SIMDJSON_PADDING: usize = 64;
pub const SIMDJSON_MAXSIZE_BYTES: usize = 0xFFFFFFFF;
pub const DEFAULT_BATCH_SIZE: usize = 1000000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_parses_string_and_exposes_tape_view() {
        let mut parser = dom_ffi::parser_new(SIMDJSON_MAXSIZE_BYTES);
        let json = String::from(r#"{"answer":42,"message":"ok"}"#);

        // SAFETY: realloc=true lets simdjson own the padded input.
        assert_eq!(
            unsafe { dom_ffi::parser_parse(parser.pin_mut(), json.as_bytes(), true) },
            0
        );

        // SAFETY: the parse above succeeded.
        let view = unsafe { dom_ffi::parser_get_tape_view(parser.as_ref().unwrap()) };
        assert!(!view.tape.is_empty());
        assert_eq!(view.tape[0] >> 56, u64::from(b'r'));
        assert!(!view.string_buf.is_empty());
    }
    #[test]
    fn string_view_contains_only_initialized_prefix() {
        let mut parser = dom_ffi::parser_new(SIMDJSON_MAXSIZE_BYTES);
        for (json, expected) in [
            (r#"["longer string", "x"]"#, 24),
            // This integer's payload has a string tag in its high byte.
            ("[2449958197289549824]", 0),
            (r#""x""#, 6),
            ("null", 0),
        ] {
            // SAFETY: realloc=true copies input into padded storage.
            assert_eq!(
                unsafe { dom_ffi::parser_parse(parser.pin_mut(), json.as_bytes(), true) },
                0
            );
            // SAFETY: the last parse succeeded and no mutation intervened.
            let view = unsafe { dom_ffi::parser_get_tape_view(parser.as_ref().unwrap()) };
            assert_eq!(view.string_buf.len(), expected, "{json}");
        }
    }
}
