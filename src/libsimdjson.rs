#[cxx::bridge(namespace = simdjson::ffi)]
pub mod ffi {


    extern "C" {
        include!("csrc/wrapper.h");
        type parser;
        type element;
        type padded_string;
        // type tape_ref;

        type array;
        type object;

        fn parser_new(max_capacity: usize) -> UniquePtr<parser>;
        fn parser_load(p: &mut parser, path: &str) -> Result<UniquePtr<element>>;
        fn parser_parse_string(p: &mut parser, s: &str) -> Result<UniquePtr<element>>;
        fn parser_parse_padded_string(p: &mut parser, s: &padded_string) -> Result<UniquePtr<element>>;

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
        fn array_at_index(arr: &array,  index: usize) -> Result<UniquePtr<element>>;

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
        let element = ffi::parser_load(&mut parser, "../json-examples/twitter.json").unwrap();
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
