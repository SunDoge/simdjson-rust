#[cxx::bridge(namespace = simdjson::ffi)]
pub mod ffi {

    struct ElementResult {
        value: UniquePtr<element>,
        code: i32,
    }

    struct StringResult {
        value: UniquePtr<CxxString>,
        code: i32,
    }

    struct BoolResult {
        value: bool,
        code: i32,
    }

    struct NumberResult {
        value: u64,
        code: i32,
    }

    struct ArrayResult {
        value: UniquePtr<array>,
        code: i32,
    }

    struct ObjectResult {
        value: UniquePtr<object>,
        code: i32,
    }

    struct KeyValuePair {
        key: UniquePtr<CxxString>,
        value: UniquePtr<element>,
    }

    extern "C" {
        include!("csrc/wrapper.h");
        type parser;
        type element;
        // type padded_string;
        // // type tape_ref;

        type array;
        type object;

        // type simdjson_result;

        fn parser_new(max_capacity: usize) -> UniquePtr<parser>;
        fn parser_load(p: &mut parser, path: &str) -> ElementResult;
        fn parser_parse_string(p: &mut parser, s: &str) -> ElementResult;
        // fn parser_parse_padded_string(
        //     p: &mut parser,
        //     s: &padded_string,
        //     code: &mut i32,
        // ) -> UniquePtr<element>;

        // fn padded_string_from_string(s: &str) -> UniquePtr<padded_string>;

        // fn tape_ref_type(tr: &tape_ref) -> u8;
        // fn tape_ref_next_tape_value(tr: &tape_ref) -> u64;

        fn element_get_string(elm: &element) -> StringResult;
        fn element_get_array(elm: &element) -> ArrayResult;
        fn element_get_object(elm: &element) -> ObjectResult;
        fn element_get_number(elm: &element) -> NumberResult;
        fn element_get_bool(elm: &element) -> BoolResult;
        fn element_is_null(elm: &element) -> bool;
        fn element_at(elm: &element, json_pointer: &str) -> ElementResult;
        fn element_at_index(elm: &element, index: usize) -> ElementResult;
        fn element_at_key(elm: &element, key: &str) -> ElementResult;

        fn array_at(arr: &array, json_pointer: &str) -> ElementResult;
        fn array_at_index(arr: &array, index: usize) -> ElementResult;

        fn object_at(obj: &object, json_pointer: &str) -> ElementResult;
        fn object_at_key(obj: &object, key: &str) -> ElementResult;
        fn object_at_key_case_insensitive(obj: &object, key: &str) -> ElementResult;
        
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
        let mut code = 0;
        let result = ffi::parser_load(&mut parser, "json-examples/twitter.json");
        // dbg!(parser.load);
        println!("parse code: {}", result.code);
        let result = ffi::parser_parse_string(&mut parser, r#""1234""#);
        println!("parse code: {}", result.code);
        println!("value: {:?}", ffi::element_get_string(&result.value).value);
    }

    // #[test]
    // fn parse_padded_string() {
    //     let ps = ffi::padded_string_from_string("[1]");
    //     let mut parser = ffi::parser_new(SIMDJSON_MAXSIZE_BYTES);
    //     let mut code = 0;
    //     let element = ffi::parser_parse_padded_string(&mut parser, &ps, &mut code);
    //     println!("parse code: {}", code);
    // }
}
