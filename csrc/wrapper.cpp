#include "wrapper.h"
#include <iostream>
#include "src/libsimdjson.rs.h"

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

        ElementResult parser_load(parser &p, rust::Str path)
        {
            element value;
            error_code error;
            const std::string &cpath = std::string(path);
            p.load(cpath).tie(value, error);

            return ElementResult{
                .value = std::make_unique<element>(value),
                .code = int(error),
            };
        }

        ElementResult parser_parse_string(parser &p, rust::Str s)
        {
            element value;
            error_code error;
            const std::string &cs = std::string(s);
            p.parse(cs).tie(value, error);
            return ElementResult{
                .value = std::make_unique<element>(value),
                .code = int(error),
            };
        }

        ElementResult parser_parse_padded_string(parser &p, const padded_string &s)
        {
            element value;
            error_code error;
            p.parse(s).tie(value, error);
            return ElementResult{
                .value = std::make_unique<element>(value),
                .code = int(error),
            };
        }

        std::unique_ptr<padded_string> padded_string_from_string(rust::Str s)
        {
            const std::string &cs = std::string(s);
            return std::make_unique<padded_string>(cs);
        }

        // // uint8_t tape_ref_type(const tape_ref &tr)
        // // {
        // //     auto tape_type = tr.tape_ref_type();
        // //     return static_cast<uint8_t>(tape_type);
        // // }

        // // uint64_t tape_ref_next_tape_value(const tape_ref &tr)
        // // {
        // //     auto value = tr.next_tape_value<uint64_t>();
        // //     return value;
        // // }

        StringResult element_get_string(const element &elm)
        {
            const char *value;
            error_code error;
            elm.get<const char *>().tie(value, error);

            return StringResult{
                .value = rust::String(value),
                .code = int(error),
            };
        }

        ArrayResult element_get_array(const element &elm)
        {
            array value;
            error_code error;
            elm.get<array>().tie(value, error);
            return ArrayResult{
                .value = std::make_unique<array>(value),
                .code = int(error),
            };
        }

        ObjectResult element_get_object(const element &elm)
        {
            object value;
            error_code error;
            elm.get<object>().tie(value, error);
            return ObjectResult{
                .value = std::make_unique<object>(value),
                .code = int(error),
            };
        }

        // NumberResult element_get_number(const element &elm)
        // {
        //     uint64_t value;
        //     error_code error;
        //     elm.get<uint64_t>().tie(value, error);
        //     return NumberResult{
        //         .value = value,
        //         .code = int(error),
        //     };
        // }

        U64Result element_get_u64(const element &elm)
        {
            uint64_t value;
            error_code error;
            elm.get<uint64_t>().tie(value, error);
            return U64Result{
                .value = value,
                .code = int(error),
            };
        }

        I64Result element_get_i64(const element &elm)
        {
            int64_t value;
            error_code error;
            elm.get<int64_t>().tie(value, error);
            return I64Result{
                .value = value,
                .code = int(error),
            };
        }

        F64Result element_get_f64(const element &elm)
        {
            double value;
            error_code error;
            elm.get<double>().tie(value, error);
            return F64Result{
                .value = value,
                .code = int(error),
            };
        }

        BoolResult element_get_bool(const element &elm)
        {
            bool value;
            error_code error;
            elm.get<bool>().tie(value, error);
            return BoolResult{
                .value = value,
                .code = int(error),
            };
        }

        bool element_is_null(const element &elm)
        {
            return elm.is_null();
        }

        ElementResult element_at(const element &elm, rust::Str s)
        {
            element value;
            error_code error;
            auto json_pointer = std::string_view(s.data(), s.size());
            elm.at(json_pointer).tie(value, error);
            return ElementResult{
                .value = std::make_unique<element>(value),
                .code = int(error),
            };
        }

        ElementResult element_at_index(const element &elm, size_t index)
        {
            element value;
            error_code error;
            elm.at(index).tie(value, error);
            return ElementResult{
                .value = std::make_unique<element>(value),
                .code = int(error),
            };
        }

        ElementResult element_at_key(const element &elm, rust::Str s)
        {
            element value;
            error_code error;
            auto key = std::string_view(s.data(), s.size());
            elm.at_key(key).tie(value, error);
            return ElementResult{
                .value = std::make_unique<element>(value),
                .code = int(error),
            };
        }

        int element_get_type(const element &elm)
        {
            return int(elm.type());
        }

        ElementResult array_at(const array &arr, rust::Str s)
        {
            element value;
            error_code error;
            auto json_pointer = std::string_view(s.data(), s.size());
            arr.at(json_pointer).tie(value, error);
            return ElementResult{
                .value = std::make_unique<element>(value),
                .code = int(error),
            };
        }

        ElementResult array_at_index(const array &arr, size_t index)
        {
            element value;
            error_code error;
            arr.at(index).tie(value, error);
            return ElementResult{
                .value = std::make_unique<element>(value),
                .code = int(error),
            };
        }

        ElementResult object_at(const object &obj, rust::Str s)
        {
            element value;
            error_code error;
            auto json_pointer = std::string_view(s.data(), s.size());
            obj.at(json_pointer).tie(value, error);
            return ElementResult{
                .value = std::make_unique<element>(value),
                .code = int(error),
            };
        }

        ElementResult object_at_key(const object &obj, rust::Str s)
        {
            element value;
            error_code error;
            auto key = std::string_view(s.data(), s.size());
            obj.at_key(key).tie(value, error);
            return ElementResult{
                .value = std::make_unique<element>(value),
                .code = int(error),
            };
        }

        ElementResult object_at_key_case_insensitive(const object &obj, rust::Str s)
        {
            element value;
            error_code error;
            auto key = std::string_view(s.data(), s.size());
            obj.at_key_case_insensitive(key).tie(value, error);
            return ElementResult{
                .value = std::make_unique<element>(value),
                .code = int(error),
            };
        }

        std::unique_ptr<ArrayIterator> array_get_iterator(const array &arr)
        {
            auto iter = ArrayIterator{
                .begin = arr.begin(),
                .end = arr.end(),
            };
            return std::make_unique<ArrayIterator>(iter);
        }

        std::unique_ptr<element> array_iterator_next(ArrayIterator &iter)
        {
            // if (iter.begin != iter.end)
            // {
            //     element elm = **(iter.begin);
            //     ++(*(iter.begin));
            //     return std::make_unique<element>(elm);
            // }
            // else
            // {
            //     return nullptr;
            // }
            if (iter.begin != iter.end)
            {
                element elm = *iter.begin;
                ++(iter.begin);
                return std::make_unique<element>(elm);
            }
            else
            {
                return nullptr;
            }
        }

        std::unique_ptr<ObjectIterator> object_get_iterator(const object &obj)
        {
            auto iter = ObjectIterator{
                .begin = obj.begin(),
                .end = obj.end(),
            };
            return std::make_unique<ObjectIterator>(iter);
        }

        void object_iterator_next(ObjectIterator &iter)
        {
            // if (iter.begin != iter.end)
            // {
            //     key_value_pair kvp = *iter.begin;
            //     // auto out = KeyValuePair{
            //     //     .key = rust::String(kvp.key.data()),
            //     //     .value = std::make_unique<element>(kvp.value),
            //     // };
            //     ++(iter.begin);
            //     // return out;
            // }
            // else
            // {

            // }
            ++(iter.begin);
        }

        bool object_iterator_has_next(const ObjectIterator &iter)
        {
            return iter.begin != iter.end;
        }

        rust::String object_iterator_key(const ObjectIterator &iter)
        {
            return rust::String(iter.begin.key().data());
        }
        std::unique_ptr<element> object_iterator_value(const ObjectIterator &iter)
        {
            return std::make_unique<element>(iter.begin.value());
        }

        rust::Str element_minify(const element &elm)
        {
            auto s = std::string(minify(elm));
            return rust::Str(s);
        }

        rust::Str object_minify(const object &obj)
        {
            auto s = std::string(minify(obj));
            return rust::Str(s);
        }

        rust::Str array_minify(const array &arr)
        {
            auto s = std::string(minify(arr));
            return rust::Str(s);
        }

        std::unique_ptr<DocumentStreamIterator> parser_load_many(parser &p, rust::Str path, size_t batch_size)
        {
            // std::unique_ptr<document_stream> stream = nullptr;
            auto cpath = std::string(path);

            auto stream = p.load_many(cpath, batch_size);

            // return std::make_unique<document_stream>()
            return document_stream_get_iterator(stream);
        }
        std::unique_ptr<DocumentStreamIterator> document_stream_get_iterator(document_stream &stream)
        {
            auto iter = DocumentStreamIterator{
                .begin = stream.begin(),
                .end = stream.end(),
            };
            return std::make_unique<DocumentStreamIterator>(iter);
        }
        ElementResult document_stream_iterator_next(DocumentStreamIterator &iter)
        {
            if (iter.begin != iter.end)
            {
                element value;
                error_code error;
                (*iter.begin).tie(value, error);
                ++(iter.begin);
                return ElementResult{
                    .value = std::make_unique<element>(value),
                    .code = int(error),
                };
            }
            else
            {
                return ElementResult{
                    .value = nullptr,
                    .code = 0,
                };
            }
        }

        std::unique_ptr<DocumentStreamIterator> parser_parse_many(parser &p, rust::Str s, size_t batch_size)
        {
            auto cs = std::string(s);
            auto stream = p.parse_many(cs, batch_size);
            return document_stream_get_iterator(stream);
        }

        std::unique_ptr<DocumentStreamIterator> parser_parse_many_padded(parser &p, const padded_string &s, size_t batch_size)
        {
            auto stream = p.parse_many(s, batch_size);
            return document_stream_get_iterator(stream);
        }

    } // namespace ffi
} // namespace simdjson