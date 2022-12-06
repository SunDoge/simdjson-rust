#pragma once

#include "rust/cxx.h"
#include "simdjson.h"

namespace ffi
{
    using OndemandParser = simdjson::ondemand::parser;
    using PaddedString = simdjson::padded_string;
    using ErrorCode = simdjson::error_code;
    using OndemandDocument = simdjson::ondemand::document;
    using OndemandObject = simdjson::ondemand::object;
    using OndemandValue = simdjson::ondemand::value;
    using OndemandArray = simdjson::ondemand::array;
    using OndemandArrayIterator = simdjson::ondemand::array_iterator;

    int get_int();

    // ondemand::parser
    std::unique_ptr<OndemandParser> ondemand_parser_new(size_t max_capacity);
    std::unique_ptr<OndemandDocument> ondemand_parser_iterate(OndemandParser &self, const PaddedString &ps, ErrorCode &code);

    // ondemand::document
    std::unique_ptr<OndemandObject> ondemand_document_get_object(OndemandDocument &doc, ErrorCode &code);
    std::unique_ptr<OndemandValue> ondemand_document_at_pointer(OndemandDocument &doc, const rust::Str json_pointer, ErrorCode &code);

    // ondemand::value
    uint64_t ondemand_value_get_uint64(OndemandValue &value, ErrorCode &code);
    std::unique_ptr<OndemandArray> ondemand_value_get_array(OndemandValue &value, ErrorCode &code);

    // ondemand::object
    std::unique_ptr<OndemandValue> ondemand_object_at_pointer(OndemandObject &obj, const rust::Str json_pointer, ErrorCode &code);

    // ondemand::array
    std::unique_ptr<OndemandArrayIterator> ondemand_array_begin(OndemandArray &arr, ErrorCode &code);
    std::unique_ptr<OndemandArrayIterator> ondemand_array_end(OndemandArray &arr, ErrorCode &code);

    // ondemand::array_iterator
    bool ondemand_array_iterator_equal(const OndemandArrayIterator &lhs, const OndemandArrayIterator &rhs);
    bool ondemand_array_iterator_not_equal(const OndemandArrayIterator &lhs, const OndemandArrayIterator &rhs);
    std::unique_ptr<OndemandValue> ondemand_array_iterator_next(OndemandArrayIterator &iter);


    // padded_string
    std::unique_ptr<PaddedString> padded_string_load(const std::string &filename, ErrorCode &code);
    std::unique_ptr<PaddedString> padded_string_from_str(const rust::Str s);

} // namespace ffi
