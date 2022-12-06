#pragma once

#include "rust/cxx.h"
#include "simdjson.h"

namespace ffi
{
    using OndemandParser = simdjson::ondemand::parser;
    using PaddedString = simdjson::padded_string;
    using ErrorCode = simdjson::error_code;
    using OndemandDocument = simdjson::ondemand::document;
    using PaddedStringResult = simdjson::simdjson_result<PaddedString>;

    int get_int();

    std::unique_ptr<OndemandParser> new_ondemand_parser(size_t max_capacity);
    std::unique_ptr<OndemandDocument> ondemand_parser_iterate(OndemandParser &self, const PaddedString &ps, ErrorCode &code);

    std::unique_ptr<PaddedString> padded_string_load(const std::string &filename, ErrorCode &code);

} // namespace ffi
