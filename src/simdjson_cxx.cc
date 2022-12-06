#include "include/simdjson_cxx.h"

namespace ffi
{
    int get_int()
    {
        return 1;
    }

    std::unique_ptr<OndemandParser> new_ondemand_parser(size_t max_capacity)
    {
        return std::make_unique<OndemandParser>(max_capacity);
    }

    std::unique_ptr<OndemandDocument> ondemand_parser_iterate(OndemandParser &p, const PaddedString &ps, ErrorCode &code)
    {
        OndemandDocument doc;
        p.iterate(ps).tie(doc, code);
        return std::make_unique<OndemandDocument>(std::move(doc));
    }

    std::unique_ptr<OndemandObject> ondemand_document_get_object(OndemandDocument &doc, ErrorCode &code)
    {
        OndemandObject obj;
        doc.get_object().tie(obj, code);
        return std::make_unique<OndemandObject>(std::move(obj));
    }

    std::unique_ptr<OndemandValue> ondemand_document_at_pointer(OndemandDocument &doc,const std::string &json_pointer, ErrorCode &code)
    {
        OndemandValue value;
        doc.at_pointer(json_pointer).tie(value, code);
        return std::make_unique<OndemandValue>(std::move(value));
    }

    uint64_t ondemand_value_get_uint64(OndemandValue &value, ErrorCode &code)
    {
        uint64_t v;
        value.get_uint64().tie(v, code);
        return v;
    }

    std::unique_ptr<PaddedString> padded_string_load(const std::string &filename, ErrorCode &code)
    {
        PaddedString ps;
        PaddedString::load(filename).tie(ps, code);
        return std::make_unique<PaddedString>(std::move(ps));
    }

} // namespace ffi
