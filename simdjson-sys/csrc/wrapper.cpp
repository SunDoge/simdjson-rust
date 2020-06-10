#include "wrapper.h"
#include <iostream>

namespace simdjson
{
    namespace ffi
    {
        // using simdjson::dom::element;
        // using simdjson::dom::parser;

        std::unique_ptr<parser> parser_new(size_t max_capacity)
        {
            return std::make_unique<parser>(max_capacity);
        }

        std::unique_ptr<element> parser_load(parser &p, rust::Str path)
        {
            const std::string &cpath = std::string(path);
            auto result = p.load(cpath);
            return std::make_unique<element>(result.value());
        }

        std::unique_ptr<element> parser_parse_string(parser &p, rust::Str s)
        {
            const std::string &cs = std::string(s);
            auto result = p.parse(cs);
            return std::make_unique<element>(result.value());
        }

        std::unique_ptr<element> parser_parse_padded_string(parser &p, const padded_string &s)
        {
            auto result = p.parse(s);
            return std::make_unique<element>(result.value());
        }

        std::unique_ptr<padded_string> padded_string_from_string(rust::Str s)
        {
            const std::string &cs = std::string(s);
            return std::make_unique<padded_string>(cs);
        }

    } // namespace ffi
} // namespace simdjson