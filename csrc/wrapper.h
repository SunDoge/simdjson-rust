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

        // using simdjson::dom::parser;
        // using simdjson::dom::element;


        struct ElementResult;
        struct StringResult;
       


        std::unique_ptr<parser> parser_new(size_t max_capacity);

        ElementResult parser_load(parser &p, rust::Str path);

        ElementResult parser_parse_string(parser &p, rust::Str s);

        ElementResult parser_parse_padded_string(parser &p, const padded_string &s);

        std::unique_ptr<padded_string> padded_string_from_string(rust::Str s);

        // // uint8_t tape_ref_type(const tape_ref &tr);

        // // uint64_t tape_ref_next_tape_value(const tape_ref &tr);

        StringResult element_get_string(const element &elm);

        // std::unique_ptr<array> element_get_array(const element &elm);

        // std::unique_ptr<object> element_get_object(const element &elm);

        // uint64_t element_get_number(const element &elm);

        // bool element_is_null(const element &elm);

        // std::unique_ptr<element> element_at(const element &elm, rust::Str s);

        // std::unique_ptr<element> element_at_index(const element &elm, size_t index);

        // std::unique_ptr<element> element_at_key(const element &elm, rust::Str s);

        // std::unique_ptr<element> array_at(const array &arr, rust::Str s);

        // std::unique_ptr<element> array_at_index(const array &arr, size_t index);

        // std::unique_ptr<element> object_at(const object &obj, rust::Str s);

        // std::unique_ptr<element> object_at_key(const object &obj, rust::Str s);

        // std::unique_ptr<element> object_at_key_case_insensitive(const object &obj, rust::Str s);

    } // namespace ffi
} // namespace simdjson