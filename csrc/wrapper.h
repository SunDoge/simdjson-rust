#pragma once

#include "simdjson.h"
#include "cxx.h"

namespace simdjson
{
    namespace ffi
    {

        // using simdjson::padded_string;
        // // using simdjson::simdjson_result;
        using simdjson::error_code;
        // using simdjson::dom::array;
        // using simdjson::dom::element;
        // using simdjson::dom::object;
        // using simdjson::dom::parser;
        // using simdjson::internal::tape_ref;

        using namespace simdjson::dom;

        using array_iterator = simdjson::dom::array::iterator;
        using object_iterator = simdjson::dom::object::iterator;

        // using simdjson::dom::parser;
        // using simdjson::dom::element;

        struct ElementResult;
        struct StringResult;
        struct ArrayResult;
        struct ObjectResult;
        // struct NumberResult;
        struct BoolResult;
        struct ArrayIterator;
        struct ObjectIterator;
        struct KeyValuePair;
        struct U64Result;
        struct I64Result;
        struct F64Result;

        std::unique_ptr<parser> parser_new(size_t max_capacity);

        ElementResult parser_load(parser &p, rust::Str path);

        ElementResult parser_parse_string(parser &p, rust::Str s);

        ElementResult parser_parse_padded_string(parser &p, const padded_string &s);

        std::unique_ptr<padded_string> padded_string_from_string(rust::Str s);

        // // uint8_t tape_ref_type(const tape_ref &tr);

        // // uint64_t tape_ref_next_tape_value(const tape_ref &tr);

        StringResult element_get_string(const element &elm);

        ArrayResult element_get_array(const element &elm);

        ObjectResult element_get_object(const element &elm);

        // NumberResult element_get_number(const element &elm);
        U64Result element_get_u64(const element &elm);
        I64Result element_get_i64(const element &elm);
        F64Result element_get_f64(const element &elm);

        BoolResult element_get_bool(const element &elm);

        bool element_is_null(const element &elm);

        ElementResult element_at(const element &elm, rust::Str s);

        ElementResult element_at_index(const element &elm, size_t index);

        ElementResult element_at_key(const element &elm, rust::Str s);

        ElementResult array_at(const array &arr, rust::Str s);

        ElementResult array_at_index(const array &arr, size_t index);

        ElementResult object_at(const object &obj, rust::Str s);

        ElementResult object_at_key(const object &obj, rust::Str s);

        ElementResult object_at_key_case_insensitive(const object &obj, rust::Str s);

        ArrayIterator array_get_iterator(const array &arr);

        std::unique_ptr<element> array_iterator_next(ArrayIterator &iter);

        ObjectIterator object_get_iterator(const object &obj);

        KeyValuePair object_iterator_next(ObjectIterator &iter);

    } // namespace ffi
} // namespace simdjson