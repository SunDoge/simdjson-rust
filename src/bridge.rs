#[cxx::bridge(namespace = ffi)]
pub(crate) mod ffi {

    #[derive(Debug)]
    #[repr(i32)]
    enum ErrorCode {
        SUCCESS = 0,
        ///< No error
        CAPACITY,
        ///< This parser can't support a document that big
        MEMALLOC,
        ///< Error allocating memory, most likely out of memory
        TAPE_ERROR,
        ///< Something went wrong, this is a generic error
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
        PARSER_IN_USE,
        ///< parser is already in use.
        OUT_OF_ORDER_ITERATION,
        ///< tried to iterate an array or object out of order
        INSUFFICIENT_PADDING,
        ///< The JSON doesn't have enough padding for simdjson to safely parse it.
        INCOMPLETE_ARRAY_OR_OBJECT,
        ///< The document ends early.
        SCALAR_DOCUMENT_AS_VALUE,
        ///< A scalar document is treated as a value.
        OUT_OF_BOUNDS,
        ///< Attempted to access location outside of document.
        TRAILING_CONTENT,
        ///< Unexpected trailing content in the JSON input
        NUM_ERROR_CODES,
    }

    unsafe extern "C++" {
        include!("include/simdjson_cxx.h");

        type ErrorCode;

        fn get_int() -> i32;

        type OndemandParser;
        type OndemandDocument;
        type OndemandValue;
        type OndemandObject;
        type OndemandArray;
        type OndemandArrayIterator;
        type OndemandField;
        type OndemandObjectIterator;
        type OndemandRawJsonString;

        type PaddedString;

        // ondemand::parser
        fn ondemand_parser_new(max_capacity: usize) -> UniquePtr<OndemandParser>;
        fn ondemand_parser_iterate(
            p: Pin<&mut OndemandParser>,
            ps: &PaddedString,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandDocument>;

        // ondemand::document
        fn ondemand_document_at_pointer(
            doc: Pin<&mut OndemandDocument>,
            json_pointer: &str,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandValue>;
        fn ondemand_document_get_object(
            doc: Pin<&mut OndemandDocument>,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandObject>;
        fn ondemand_document_find_field(
            doc: Pin<&mut OndemandDocument>,
            key: &str,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandValue>;
        fn ondemand_document_find_field_unordered(
            doc: Pin<&mut OndemandDocument>,
            key: &str,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandValue>;

        // ondemand::value
        fn ondemand_value_get_uint64(value: Pin<&mut OndemandValue>, code: &mut ErrorCode) -> u64;
        fn ondemand_value_get_array(
            value: Pin<&mut OndemandValue>,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandArray>;
        fn ondemand_value_get_object(
            value: Pin<&mut OndemandValue>,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandObject>;
        fn ondemand_value_find_field(
            doc: Pin<&mut OndemandValue>,
            key: &str,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandValue>;
        fn ondemand_value_find_field_unordered(
            doc: Pin<&mut OndemandValue>,
            key: &str,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandValue>;

        // ondemand::object
        fn ondemand_object_at_pointer(
            obj: Pin<&mut OndemandObject>,
            json_pointer: &str,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandValue>;
        fn ondemand_object_begin(
            obj: Pin<&mut OndemandObject>,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandObjectIterator>;
        fn ondemand_object_end(
            obj: Pin<&mut OndemandObject>,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandObjectIterator>;
        fn ondemand_object_raw_json<'a>(
            obj: Pin<&mut OndemandObject>,
            code: &mut ErrorCode,
        ) -> &'a str;

        // ondemand::object_iterator
        fn ondemand_object_iterator_not_equal(
            lhs: &OndemandObjectIterator,
            rhs: &OndemandObjectIterator,
        ) -> bool;
        fn ondemand_object_iterator_next(
            iter: Pin<&mut OndemandObjectIterator>,
        ) -> Pin<&mut OndemandObjectIterator>;
        fn ondemand_object_iterator_get(
            iter: Pin<&mut OndemandObjectIterator>,
            // key: Pin<&mut OndemandRawJsonString>,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandField>;

        // ondemand::array
        fn ondemand_array_begin(
            arr: Pin<&mut OndemandArray>,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandArrayIterator>;
        fn ondemand_array_end(
            arr: Pin<&mut OndemandArray>,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandArrayIterator>;
        fn ondemand_array_at(
            arr: Pin<&mut OndemandArray>,
            index: usize,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandValue>;

        // ondemand::array_iterator
        fn ondemand_array_iterator_equal(
            lhs: &OndemandArrayIterator,
            rhs: &OndemandArrayIterator,
        ) -> bool;
        fn ondemand_array_iterator_not_equal(
            lhs: &OndemandArrayIterator,
            rhs: &OndemandArrayIterator,
        ) -> bool;
        fn ondemand_array_iterator_next(
            iter: Pin<&mut OndemandArrayIterator>,
        ) -> Pin<&mut OndemandArrayIterator>;
        fn ondemand_array_iterator_get(
            iter: Pin<&mut OndemandArrayIterator>,
            code: &mut ErrorCode,
        ) -> UniquePtr<OndemandValue>;

        // ondemand::field
        fn ondemand_field_unescaped_key<'a>(
            field: Pin<&mut OndemandField>,
            code: &mut ErrorCode,
        ) -> &'a str;
        fn ondemand_field_value(field: Pin<&mut OndemandField>) -> UniquePtr<OndemandValue>;
        fn ondemand_field_key(field: &OndemandField) -> UniquePtr<OndemandRawJsonString>;

        // ondemand::raw_json_string
        // fn ondemand_raw_json_string_unescape<'a>(
        //     rjs: &OndemandRawJsonString,
        //     value: Pin<&mut OndemandValue>,
        // ) -> &'a str;

        // padded_string
        fn padded_string_load(
            filename: &CxxString,
            code: &mut ErrorCode,
        ) -> UniquePtr<PaddedString>;
        fn padded_string_from_str(s: &str) -> UniquePtr<PaddedString>;

    }
}

macro_rules! check {
    ($func:expr, $($x:expr), + $(,)?) => {
        {
            use crate::bridge::ffi::ErrorCode;

            let mut code = ErrorCode::SUCCESS;

            let res = $func($($x),+, &mut code);

            // if code == ErrorCode::SUCCESS {
            //     Ok(res)
            // } else {
            //     Err(code.into())
            // }
            match code {
                ErrorCode::SUCCESS => Ok(res),
                _ => Err(code.into())
            }
        }
    };
}

pub(crate) use check;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(ffi::get_int(), 1);
    }

    #[test]
    fn new_parser() {
        let _parser = ffi::ondemand_parser_new(1024);
    }
}
