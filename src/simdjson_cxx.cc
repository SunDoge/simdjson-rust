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

    std::unique_ptr<PaddedString> padded_string_load(const std::string &filename, ErrorCode &code)
    {
        PaddedString ps;
        PaddedString::load(filename).tie(ps, code);
        return std::make_unique<PaddedString>(std::move(ps));
    }

} // namespace ffi
