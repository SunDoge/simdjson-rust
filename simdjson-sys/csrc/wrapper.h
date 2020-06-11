#pragma once

#include "simdjson.h"
#include "cxx.h"

namespace simdjson
{
    namespace ffi
    {

        using simdjson::padded_string;
        using simdjson::simdjson_result;
        using simdjson::dom::element;
        using simdjson::dom::parser;
        using simdjson::internal::tape_ref;

        std::unique_ptr<parser> parser_new(size_t max_capacity);

        std::unique_ptr<element> parser_load(parser &p, rust::Str path);

        std::unique_ptr<element> parser_parse_string(parser &p, rust::Str s);

        std::unique_ptr<element> parser_parse_padded_string(parser &p, const padded_string &s);

        std::unique_ptr<padded_string> padded_string_from_string(rust::Str s);

        uint8_t tape_ref_type(const tape_ref & tr);

        uint64_t tape_ref_next_tape_value(const tape_ref &tr);

        rust::Str element_get_str(const tape_ref &tr);
    } // namespace ffi
} // namespace simdjson