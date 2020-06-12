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

        // uint8_t tape_ref_type(const tape_ref &tr)
        // {
        //     auto tape_type = tr.tape_ref_type();
        //     return static_cast<uint8_t>(tape_type);
        // }

        // uint64_t tape_ref_next_tape_value(const tape_ref &tr)
        // {
        //     auto value = tr.next_tape_value<uint64_t>();
        //     return value;
        // }

        rust::Str element_get_string(const element &elm)
        {
            auto result = elm.get<const char *>();
            return rust::Str(result.value());
        }

        std::unique_ptr<array> element_get_array(const element &elm)
        {
            auto result = elm.get<array>();
            return std::make_unique<array>(result.value());
        }

        std::unique_ptr<object> element_get_object(const element &elm)
        {
            auto result = elm.get<object>();
            return std::make_unique<object>(result.value());
        }

        uint64_t element_get_number(const element &elm)
        {
            auto result = elm.get<uint64_t>();
            return result.value();
        }

        bool element_is_null(const element &elm)
        {
            return elm.is_null();
        }

        std::unique_ptr<element> element_at(const element &elm, rust::Str s)
        {
            auto json_pointer = std::string_view(s.data(), s.size());
            auto result = elm.at(json_pointer);
            return std::make_unique<element>(result.value());
        }

        std::unique_ptr<element> element_at_index(const element &elm, size_t index)
        {
            auto result = elm.at(index);
            return std::make_unique<element>(result.value());
        }

        std::unique_ptr<element> element_at_key(const element &elm, rust::Str s)
        {
            auto key = std::string_view(s.data(), s.size());
            auto result = elm.at_key(key);
            return std::make_unique<element>(result.value());
        }

        std::unique_ptr<element> array_at(const array &arr, rust::Str s)
        {
            auto json_pointer = std::string_view(s.data(), s.size());
            auto result = arr.at(json_pointer);
            return std::make_unique<element>(result.value());
        }

        std::unique_ptr<element> array_at_index(const array &arr, size_t index)
        {
            auto result = arr.at(index);
            return std::make_unique<element>(result.value());
        }

        std::unique_ptr<element> object_at(const object &obj, rust::Str s)
        {
            auto json_pointer = std::string_view(s.data(), s.size());
            auto result = obj.at(json_pointer);
            return std::make_unique<element>(result.value());
        }

        std::unique_ptr<element> object_at_key(const object &obj, rust::Str s)
        {
            auto key = std::string_view(s.data(), s.size());
            auto result = obj.at_key(key);
            return std::make_unique<element>(result.value());
        }

        std::unique_ptr<element> object_at_key_case_insensitive(const object &obj, rust::Str s)
        {
            auto key = std::string_view(s.data(), s.size());
            auto result = obj.at_key_case_insensitive(key);
            return std::make_unique<element>(result.value());
        }
    } // namespace ffi
} // namespace simdjson