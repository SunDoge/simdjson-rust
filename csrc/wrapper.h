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
        using document_stream_iterator = simdjson::dom::document_stream::iterator;

        // using simdjson::dom::parser;
        // using simdjson::dom::element;

        struct ElementResult;
        struct StringResult;
        struct ArrayResult;
        struct ObjectResult;
        // struct NumberResult;
        struct BoolResult;
        // struct ArrayIterator;
        // struct ObjectIterator;
        struct KeyValuePair;
        struct U64Result;
        struct I64Result;
        struct F64Result;
        struct PaddedStringResult;

        struct ArrayIterator
        {
            array_iterator begin;
            array_iterator end;
        };

        struct ObjectIterator
        {
            object_iterator begin;
            object_iterator end;
        };

        struct DocumentStreamIterator
        {
            document_stream_iterator begin;
            document_stream_iterator end;
        };

        std::unique_ptr<parser> parser_new(size_t max_capacity);
        ElementResult parser_load(parser &p, rust::Str path);
        ElementResult parser_parse(parser &p, rust::Str s);
        ElementResult parser_parse_padded(parser &p, const padded_string &s);

        std::unique_ptr<padded_string> padded_string_from_string(rust::Str s);
        PaddedStringResult padded_string_load(rust::Str s);

        // // uint8_t tape_ref_type(const tape_ref &tr);

        // // uint64_t tape_ref_next_tape_value(const tape_ref &tr);

        StringResult element_get_string(const element &elm);
        ArrayResult element_get_array(const element &elm);
        ObjectResult element_get_object(const element &elm);
        U64Result element_get_u64(const element &elm);
        I64Result element_get_i64(const element &elm);
        F64Result element_get_f64(const element &elm);
        BoolResult element_get_bool(const element &elm);
        bool element_is_null(const element &elm);
        ElementResult element_at(const element &elm, rust::Str s);
        ElementResult element_at_index(const element &elm, size_t index);
        ElementResult element_at_key(const element &elm, rust::Str s);
        int element_get_type(const element &elm);

        ElementResult array_at(const array &arr, rust::Str s);
        ElementResult array_at_index(const array &arr, size_t index);

        ElementResult object_at(const object &obj, rust::Str s);
        ElementResult object_at_key(const object &obj, rust::Str s);
        ElementResult object_at_key_case_insensitive(const object &obj, rust::Str s);

        std::unique_ptr<ArrayIterator> array_get_iterator(const array &arr);
        std::unique_ptr<element> array_iterator_next(ArrayIterator &iter);

        std::unique_ptr<ObjectIterator> object_get_iterator(const object &obj);
        void object_iterator_next(ObjectIterator &iter);
        bool object_iterator_has_next(const ObjectIterator &iter);
        rust::String object_iterator_key(const ObjectIterator &iter);
        std::unique_ptr<element> object_iterator_value(const ObjectIterator &iter);

        // For display
        rust::String element_minify(const element &elm);
        rust::String object_minify(const object &obj);
        rust::String array_minify(const array &arr);

        // For load many and parse many
        std::unique_ptr<document_stream> parser_load_many(parser &p, rust::Str path, size_t batch_size);
        std::unique_ptr<DocumentStreamIterator> document_stream_get_iterator(document_stream &stream);
        void document_stream_iterator_next(DocumentStreamIterator &iter);
        bool document_stream_iterator_has_next(const DocumentStreamIterator &iter);
        ElementResult document_stream_iterator_value(DocumentStreamIterator &iter);

        
        std::unique_ptr<document_stream> parser_parse_many(parser &p, rust::Str s, size_t batch_size);
        std::unique_ptr<document_stream> parser_parse_many_padded(parser &p, const padded_string &s, size_t batch_size);
    } // namespace ffi
} // namespace simdjson