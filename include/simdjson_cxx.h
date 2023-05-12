#pragma once

#include "rust/cxx.h"
#include "simdjson.h"

namespace ffi {
using OndemandParser = simdjson::ondemand::parser;
using PaddedString = simdjson::padded_string;
using ErrorCode = simdjson::error_code;
using OndemandDocument = simdjson::ondemand::document;
using OndemandObject = simdjson::ondemand::object;
using OndemandValue = simdjson::ondemand::value;
using OndemandArray = simdjson::ondemand::array;
using OndemandArrayIterator = simdjson::ondemand::array_iterator;
using OndemandField = simdjson::ondemand::field;
using OndemandObjectIterator = simdjson::ondemand::object_iterator;
using OndemandRawJsonString = simdjson::ondemand::raw_json_string;
using OndemandJsonType = simdjson::ondemand::json_type;

struct OndemandDocumentResult;
struct OndemandValueResult;
struct OndemandObjectResult;

int get_int();

// ondemand::parser
std::unique_ptr<OndemandParser> ondemand_parser_new(size_t max_capacity);
OndemandDocumentResult ondemand_parser_iterate(OndemandParser &self,
                                               const PaddedString &ps);

// ondemand::document
OndemandObjectResult ondemand_document_get_object(OndemandDocument &doc);
OndemandValueResult ondemand_document_at_pointer(OndemandDocument &doc,
                                                 const rust::Str json_pointer);
std::unique_ptr<OndemandValue>
ondemand_document_find_field(OndemandDocument &doc, const rust::Str key,
                             ErrorCode &code);
std::unique_ptr<OndemandValue>
ondemand_document_find_field_unordered(OndemandDocument &doc,
                                       const rust::Str key, ErrorCode &code);
std::unique_ptr<OndemandArray>
ondemand_document_get_array(OndemandDocument &doc, ErrorCode &code);
uint64_t ondemand_document_get_uint64(OndemandDocument &doc, ErrorCode &code);
uint64_t ondemand_document_get_uint64_in_string(OndemandDocument &doc,
                                                ErrorCode &code);
int64_t ondemand_document_get_int64(OndemandDocument &doc, ErrorCode &code);
int64_t ondemand_document_get_int64_in_string(OndemandDocument &doc,
                                              ErrorCode &code);
double ondemand_document_get_double(OndemandDocument &doc, ErrorCode &code);
double ondemand_document_get_double_in_string(OndemandDocument &doc,
                                              ErrorCode &code);
rust::Str ondemand_document_get_string(OndemandDocument &doc, ErrorCode &code);
bool ondemand_document_get_bool(OndemandDocument &doc, ErrorCode &code);
std::unique_ptr<OndemandRawJsonString>
ondemand_document_get_raw_json_string(OndemandDocument &doc, ErrorCode &code);
bool ondemand_document_is_null(OndemandDocument &doc, ErrorCode &code);
OndemandJsonType ondemand_document_type(OndemandDocument &doc, ErrorCode &code);
bool ondemand_document_is_scalar(OndemandDocument &doc, ErrorCode &code);
bool ondemand_document_is_negative(OndemandDocument &doc);
bool ondemand_document_is_integer(OndemandDocument &doc, ErrorCode &code);

// ondemand::value
uint64_t ondemand_value_get_uint64(OndemandValue &value, ErrorCode &code);
std::unique_ptr<OndemandArray> ondemand_value_get_array(OndemandValue &value,
                                                        ErrorCode &code);
std::unique_ptr<OndemandObject> ondemand_value_get_object(OndemandValue &value,
                                                          ErrorCode &code);
std::unique_ptr<OndemandValue> ondemand_value_find_field(OndemandValue &value,
                                                         const rust::Str key,
                                                         ErrorCode &code);
std::unique_ptr<OndemandValue>
ondemand_value_find_field_unordered(OndemandValue &value, const rust::Str key,
                                    ErrorCode &code);

// ondemand::object
std::unique_ptr<OndemandValue>
ondemand_object_at_pointer(OndemandObject &obj, const rust::Str json_pointer,
                           ErrorCode &code);
std::unique_ptr<OndemandObjectIterator>
ondemand_object_begin(OndemandObject &obj, ErrorCode &code);
std::unique_ptr<OndemandObjectIterator> ondemand_object_end(OndemandObject &obj,
                                                            ErrorCode &code);
rust::Str ondemand_object_raw_json(OndemandObject &obj, ErrorCode &code);

// ondemand::object_iterator
bool ondemand_object_iterator_not_equal(const OndemandObjectIterator &lhs,
                                        const OndemandObjectIterator &rhs);
OndemandObjectIterator &
ondemand_object_iterator_next(OndemandObjectIterator &iter);
std::unique_ptr<OndemandField>
ondemand_object_iterator_get(OndemandObjectIterator &iter, ErrorCode &code);

// ondemand::array
std::unique_ptr<OndemandArrayIterator> ondemand_array_begin(OndemandArray &arr,
                                                            ErrorCode &code);
std::unique_ptr<OndemandArrayIterator> ondemand_array_end(OndemandArray &arr,
                                                          ErrorCode &code);
std::unique_ptr<OndemandValue> ondemand_array_at(OndemandArray &arr,
                                                 size_t index, ErrorCode &code);
size_t ondemand_array_count_elements(OndemandArray &arr, ErrorCode &code);
bool ondemand_array_is_empty(OndemandArray &arr, ErrorCode &code);
std::unique_ptr<OndemandValue>
ondemand_array_at_pointer(OndemandArray &arr, const rust::Str json_pointer,
                          ErrorCode &code);

// ondemand::array_iterator
bool ondemand_array_iterator_equal(const OndemandArrayIterator &lhs,
                                   const OndemandArrayIterator &rhs);
bool ondemand_array_iterator_not_equal(const OndemandArrayIterator &lhs,
                                       const OndemandArrayIterator &rhs);
OndemandArrayIterator &
ondemand_array_iterator_next(OndemandArrayIterator &iter);
std::unique_ptr<OndemandValue>
ondemand_array_iterator_get(OndemandArrayIterator &iter, ErrorCode &code);

// ondemand::field
rust::Str ondemand_field_unescaped_key(OndemandField &field, ErrorCode &code);
std::unique_ptr<OndemandValue> ondemand_field_value(OndemandField &field);
std::unique_ptr<OndemandRawJsonString>
ondemand_field_key(const OndemandField &field);

// ondemand::raw_json_string
// rust::Str ondemand_raw_json_string_unescape(const OndemandRawJsonString &rjs,
// OndemandValue v, ErrorCode &code);

// padded_string
std::unique_ptr<PaddedString> padded_string_load(const std::string &filename,
                                                 ErrorCode &code);
std::unique_ptr<PaddedString> padded_string_from_str(const rust::Str s);

} // namespace ffi
