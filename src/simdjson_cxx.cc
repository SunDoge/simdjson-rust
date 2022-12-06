#include "include/simdjson_cxx.h"

namespace ffi
{
    int get_int()
    {
        return 1;
    }

    // ondemand::parser
    std::unique_ptr<OndemandParser> ondemand_parser_new(size_t max_capacity)
    {
        return std::make_unique<OndemandParser>(max_capacity);
    }

    std::unique_ptr<OndemandDocument> ondemand_parser_iterate(OndemandParser &p, const PaddedString &ps, ErrorCode &code)
    {
        OndemandDocument doc;
        p.iterate(ps).tie(doc, code);
        return std::make_unique<OndemandDocument>(std::move(doc));
    }

    // ondemand::document
    std::unique_ptr<OndemandObject> ondemand_document_get_object(OndemandDocument &doc, ErrorCode &code)
    {
        OndemandObject obj;
        doc.get_object().tie(obj, code);
        return std::make_unique<OndemandObject>(std::move(obj));
    }

    std::unique_ptr<OndemandValue> ondemand_document_at_pointer(OndemandDocument &doc, const rust::Str json_pointer, ErrorCode &code)
    {
        OndemandValue value;
        doc.at_pointer(std::string_view(json_pointer.data(), json_pointer.size())).tie(value, code);
        return std::make_unique<OndemandValue>(std::move(value));
    }

    // ondemand::value
    uint64_t ondemand_value_get_uint64(OndemandValue &value, ErrorCode &code)
    {
        uint64_t v;
        value.get_uint64().tie(v, code);
        return v;
    }
    std::unique_ptr<OndemandArray> ondemand_value_get_array(OndemandValue &value, ErrorCode &code)
    {
        OndemandArray arr;
        value.get_array().tie(arr, code);
        return std::make_unique<OndemandArray>(std::move(arr));
    }
    std::unique_ptr<OndemandObject> ondemand_value_get_object(OndemandValue &value, ErrorCode &code)
    {
        OndemandObject obj;
        value.get_object().tie(obj, code);
        return std::make_unique<OndemandObject>(std::move(obj));
    }

    // ondemand::object
    std::unique_ptr<OndemandValue> ondemand_object_at_pointer(OndemandObject &obj, const rust::Str json_pointer, ErrorCode &code)
    {
        OndemandValue v;
        obj.at_pointer(std::string_view(json_pointer.data(), json_pointer.size())).tie(v, code);
        return std::make_unique<OndemandValue>(std::move(v));
    }

    std::unique_ptr<OndemandObjectIterator> ondemand_object_begin(OndemandObject &arr, ErrorCode &code)
    {
        OndemandObjectIterator iter;
        arr.begin().tie(iter, code);
        return std::make_unique<OndemandObjectIterator>(std::move(iter));
    }

    std::unique_ptr<OndemandObjectIterator> ondemand_object_end(OndemandObject &arr, ErrorCode &code)
    {
        OndemandObjectIterator iter;
        arr.end().tie(iter, code);
        return std::make_unique<OndemandObjectIterator>(std::move(iter));
    }

    // ondemand::object_iterator
    bool ondemand_object_iterator_not_equal(const OndemandObjectIterator &lhs, const OndemandObjectIterator &rhs)
    {
        return lhs != rhs;
    }
    std::unique_ptr<OndemandField> ondemand_object_iterator_next(OndemandObjectIterator &iter)
    {
        auto v = std::make_unique<OndemandField>(std::move(*iter));
        ++iter;
        return v;
    }

    // ondemand::array
    std::unique_ptr<OndemandArrayIterator> ondemand_array_begin(OndemandArray &arr, ErrorCode &code)
    {
        OndemandArrayIterator iter;
        arr.begin().tie(iter, code);
        return std::make_unique<OndemandArrayIterator>(std::move(iter));
    }
    std::unique_ptr<OndemandArrayIterator> ondemand_array_end(OndemandArray &arr, ErrorCode &code)
    {
        OndemandArrayIterator iter;
        arr.end().tie(iter, code);
        return std::make_unique<OndemandArrayIterator>(std::move(iter));
    }

    bool ondemand_array_iterator_equal(const OndemandArrayIterator &lhs, const OndemandArrayIterator &rhs)
    {
        return lhs == rhs;
    }
    bool ondemand_array_iterator_not_equal(const OndemandArrayIterator &lhs, const OndemandArrayIterator &rhs)
    {
        return lhs != rhs;
    }
    std::unique_ptr<OndemandValue> ondemand_array_iterator_next(OndemandArrayIterator &iter)
    {
        auto v = std::make_unique<OndemandValue>(std::move(*iter));
        ++iter;
        return v;
    }

    // ondemand::field
    rust::Str ondemand_field_unescaped_key(OndemandField &field, ErrorCode &code)
    {
        std::string_view sv;
        field.unescaped_key().tie(sv, code);
        return rust::Str(sv.data(), sv.size());
    }

    // padded_string
    std::unique_ptr<PaddedString> padded_string_load(const std::string &filename, ErrorCode &code)
    {
        PaddedString ps;
        PaddedString::load(filename).tie(ps, code);
        return std::make_unique<PaddedString>(std::move(ps));
    }

    std::unique_ptr<PaddedString> padded_string_from_str(const rust::Str s)
    {

        return std::make_unique<PaddedString>(s.data(), s.size());
    }

} // namespace ffi
