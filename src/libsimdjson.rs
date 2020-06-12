#[cxx::bridge(namespace = simdjson::ffi)]
pub mod ffi {

    #[repr(u32)]
    enum error_code {
        SUCCESS = 0,
        ///< No error
        SUCCESS_AND_HAS_MORE,
        ///< @private No error and buffer still has more data
        CAPACITY,
        ///< This parser can't support a document that big
        MEMALLOC,
        ///< Error allocating memory, most likely out of memory
        TAPE_ERROR,
        ///< Something went wrong while writing to the tape (stage 2), this is a generic error
        DEPTH_ERROR,
        ///< Your document exceeds the user-specified depth limitation
        STRING_ERROR,
        ///< Problem while parsing a string
        T_ATOM_ERROR,
        ///< Problem while parsing an atom starting with the letter 't'
        F_ATOM_ERROR,
        ///< Problem while parsing an atom starting with the letter 'f'
        N_ATOM_ERROR,
        ///< Problem while parsing an atom starting with the letter 'n'
        NUMBER_ERROR,
        ///< Problem while parsing a number
        UTF8_ERROR,
        ///< the input is not valid UTF-8
        UNINITIALIZED,
        ///< unknown error, or uninitialized document
        EMPTY,
        ///< no structural element found
        UNESCAPED_CHARS,
        ///< found unescaped characters in a string.
        UNCLOSED_STRING,
        ///< missing quote at the end
        UNSUPPORTED_ARCHITECTURE,
        ///< unsupported architecture
        INCORRECT_TYPE,
        ///< JSON element has a different type than user expected
        NUMBER_OUT_OF_RANGE,
        ///< JSON number does not fit in 64 bits
        INDEX_OUT_OF_BOUNDS,
        ///< JSON array index too large
        NO_SUCH_FIELD,
        ///< JSON field not found in object
        IO_ERROR,
        ///< Error reading a file
        INVALID_JSON_POINTER,
        ///< Invalid JSON pointer reference
        INVALID_URI_FRAGMENT,
        ///< Invalid URI fragment
        UNEXPECTED_ERROR,
        ///< indicative of a bug in simdjson
        /** @private Number of error codes */
        NUM_ERROR_CODES,
    }

    extern "C" {
        include!("csrc/wrapper.h");
        type parser;
        type element;
        type padded_string;
        // type tape_ref;

        type array;
        type object;

        type error_code;

        fn parser_new(max_capacity: usize) -> UniquePtr<parser>;
        fn parser_load(p: &mut parser, path: &str) -> Result<UniquePtr<element>>;
        fn parser_parse_string(p: &mut parser, s: &str) -> Result<UniquePtr<element>>;
        fn parser_parse_padded_string(
            p: &mut parser,
            s: &padded_string,
        ) -> Result<UniquePtr<element>>;

        fn padded_string_from_string(s: &str) -> UniquePtr<padded_string>;

        // fn tape_ref_type(tr: &tape_ref) -> u8;
        // fn tape_ref_next_tape_value(tr: &tape_ref) -> u64;

        fn element_get_string(elm: &element) -> Result<&str>;
        fn element_get_array(elm: &element) -> Result<UniquePtr<array>>;
        fn element_get_object(elm: &element) -> Result<UniquePtr<object>>;
        fn element_get_number(elm: &element) -> Result<u64>;
        fn element_is_null(elm: &element) -> bool;
        fn element_at(elm: &element, json_pointer: &str) -> Result<UniquePtr<element>>;
        fn element_at_index(elm: &element, index: usize) -> Result<UniquePtr<element>>;
        fn element_at_key(elm: &element, key: &str) -> Result<UniquePtr<element>>;

        fn array_at(arr: &array, json_pointer: &str) -> Result<UniquePtr<element>>;
        fn array_at_index(arr: &array, index: usize) -> Result<UniquePtr<element>>;

        fn object_at(obj: &object, json_pointer: &str) -> Result<UniquePtr<element>>;
        fn object_at_key(obj: &object, key: &str) -> Result<UniquePtr<element>>;
        fn object_at_key_case_insensitive(obj: &object, key: &str) -> Result<UniquePtr<element>>;

    }
}

pub const SIMDJSON_MAXSIZE_BYTES: usize = 0xFFFFFFFF;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn get_parser() {
        let mut parser = ffi::parser_new(SIMDJSON_MAXSIZE_BYTES);
        let element = ffi::parser_load(&mut parser, "json-examples/twitter.json").unwrap();
        // dbg!(parser.load);
        let element = ffi::parser_parse_string(&mut parser, "[1]").unwrap();
    }

    #[test]
    fn parse_padded_string() {
        let ps = ffi::padded_string_from_string("[1]");
        let mut parser = ffi::parser_new(SIMDJSON_MAXSIZE_BYTES);
        let element = ffi::parser_parse_padded_string(&mut parser, &ps).unwrap();
    }
}
