#[cxx::bridge(namespace = simdjson::ffi)]
pub mod ffi {

    struct ElementResult {
        value: UniquePtr<element>,
        code: i32,
    }

    struct StringResult {
        value: String,
        code: i32,
    }

    struct BoolResult {
        value: bool,
        code: i32,
    }

    // struct NumberResult {
    //     value: u64,
    //     code: i32,
    // }
    struct U64Result {
        value: u64,
        code: i32,
    }

    struct I64Result {
        value: i64,
        code: i32,
    }

    struct F64Result {
        value: f64,
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
        key: String,
        value: UniquePtr<element>,
    }

    // struct ArrayIterator {
    //     begin: UniquePtr<array_iterator>,
    //     end: UniquePtr<array_iterator>,
    // }

    // struct ObjectIterator {
    //     begin: UniquePtr<object_iterator>,
    //     end: UniquePtr<object_iterator>,
    // }

    extern "C" {
        include!("csrc/wrapper.h");
        type parser;
        type element;
        type padded_string;
        // // type tape_ref;
        type document_stream;

        type array;
        type object;

        // type array_iterator;
        // type object_iterator;
        type ArrayIterator;
        type ObjectIterator;
        type DocumentStreamIterator;

        // type simdjson_result;

        fn parser_new(max_capacity: usize) -> UniquePtr<parser>;
        fn parser_load(p: &mut parser, path: &str) -> ElementResult;
        fn parser_parse(p: &mut parser, s: &str) -> ElementResult;
        fn parser_parse_padded(p: &mut parser, s: &padded_string) -> ElementResult;

        fn padded_string_from_string(s: &str) -> UniquePtr<padded_string>;

        // fn tape_ref_type(tr: &tape_ref) -> u8;
        // fn tape_ref_next_tape_value(tr: &tape_ref) -> u64;

        fn element_get_string(elm: &element) -> StringResult;
        fn element_get_array(elm: &element) -> ArrayResult;
        fn element_get_object(elm: &element) -> ObjectResult;
        // fn element_get_number(elm: &element) -> NumberResult;
        fn element_get_u64(elm: &element) -> U64Result;
        fn element_get_i64(elm: &element) -> I64Result;
        fn element_get_f64(elm: &element) -> F64Result;

        fn element_get_bool(elm: &element) -> BoolResult;
        fn element_is_null(elm: &element) -> bool;
        fn element_at(elm: &element, json_pointer: &str) -> ElementResult;
        fn element_at_index(elm: &element, index: usize) -> ElementResult;
        fn element_at_key(elm: &element, key: &str) -> ElementResult;

        fn element_get_type(elm: &element) -> i32;

        fn array_at(arr: &array, json_pointer: &str) -> ElementResult;
        fn array_at_index(arr: &array, index: usize) -> ElementResult;

        fn object_at(obj: &object, json_pointer: &str) -> ElementResult;
        fn object_at_key(obj: &object, key: &str) -> ElementResult;
        fn object_at_key_case_insensitive(obj: &object, key: &str) -> ElementResult;

        fn array_get_iterator(arr: &array) -> UniquePtr<ArrayIterator>;
        fn array_iterator_next(iter: &mut ArrayIterator) -> UniquePtr<element>;

        fn object_get_iterator(obj: &object) -> UniquePtr<ObjectIterator>;
        fn object_iterator_next(iter: &mut ObjectIterator);
        fn object_iterator_has_next(iter: &ObjectIterator) -> bool;
        fn object_iterator_key(iter: &ObjectIterator) -> String;
        fn object_iterator_value(iter: &ObjectIterator) -> UniquePtr<element>;

        fn element_minify(elm: &element) -> &str;
        fn object_minify(obj: &object) -> &str;
        fn array_minify(arr: &array) -> &str;

        fn parser_load_many(
            p: &mut parser,
            path: &str,
            batch_size: usize,
        ) -> UniquePtr<DocumentStreamIterator>;
        fn document_stream_iterator_next(iter: &mut DocumentStreamIterator) -> ElementResult;

        fn parser_parse_many(
            p: &mut parser,
            s: &str,
            batch_size: usize,
        ) -> UniquePtr<DocumentStreamIterator>;
        fn parser_parse_many_padded(
            p: &mut parser,
            s: &padded_string,
            batch_size: usize,
        ) -> UniquePtr<DocumentStreamIterator>;
    }
}

pub const SIMDJSON_MAXSIZE_BYTES: usize = 0xFFFFFFFF;
pub const DEFAULT_BATCH_SIZE: usize = 1000000;

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
        let result = ffi::parser_load(&mut parser, "json-examples/twitter.json");
        // dbg!(parser.load);
        println!("parse code: {}", result.code);
        let result = ffi::parser_parse(&mut parser, r#""1234""#);
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
