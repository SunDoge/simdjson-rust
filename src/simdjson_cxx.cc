#include "include/simdjson_cxx.h"
#include "simdjson-rust/src/bridge.rs.h"
#include "simdjson/error.h"

namespace ffi {
int get_int() { return 1; }
template <typename Output, typename Func>
auto check(Func func, ErrorCode &code) {
  Output output;
  func().tie(output, code);
  return output;
}

template <typename Output, typename Func>
std::unique_ptr<Output> check_unique_ptr(Func func, ErrorCode &code) {
  Output output;
  func().tie(output, code);
  return std::make_unique<Output>(std::move(output));
}

template <typename Result, typename Output>
Result
into_unique_ptr_result(simdjson::simdjson_result<Output> &&output_result) {
  Output output;
  auto code = std::move(output_result).get(output);
  return Result{
      .value = std::make_unique<Output>(std::move(output)),
      .code = code,
  };
}

template <typename Result, typename Output>
Result into_result(simdjson::simdjson_result<Output> &&output_result) {
  Output output;
  auto code = std::move(output_result).get(output);
  return Result{
      .value = output,
      .code = code,
  };
}

rust::Str string_view_to_rust_str(std::string_view sv) {
  return rust::Str(sv.data(), sv.size());
}

std::string_view rust_str_to_string_view(rust::Str s) {
  return std::string_view(s.data(), s.size());
}

// ondemand::parser
std::unique_ptr<OndemandParser> ondemand_parser_new(size_t max_capacity) {
  return std::make_unique<OndemandParser>(max_capacity);
}

OndemandDocumentResult ondemand_parser_iterate(OndemandParser &p,
                                               const PaddedString &ps) {
  //   return check_unique_ptr<OndemandDocument>([&] { return p.iterate(ps); },
  //                                             code);
  // OndemandDocument value;
  // auto code = p.iterate(ps).get(value);
  // return OndemandDocumentResult{
  //     .value = std::make_unique<OndemandDocument>(std::move(value)),
  //     .code = code,
  // };
  return into_unique_ptr_result<OndemandDocumentResult>(p.iterate(ps));
}

// ondemand::document
OndemandObjectResult ondemand_document_get_object(OndemandDocument &doc) {
  // OndemandObject obj;
  // doc.get_object().tie(obj, code);
  // return std::make_unique<OndemandObject>(std::move(obj));
  // return check_unique_ptr<OndemandObject>([&] { return doc.get_object(); },
  //                                         code);
  return into_unique_ptr_result<OndemandObjectResult>(doc.get_object());
}

OndemandValueResult ondemand_document_get_value(OndemandDocument &doc) {
  return into_unique_ptr_result<OndemandValueResult>(doc.get_value());
}

OndemandValueResult ondemand_document_at_pointer(OndemandDocument &doc,
                                                 const rust::Str json_pointer) {
  // OndemandValue value;
  // doc.at_pointer(std::string_view(json_pointer.data(),
  // json_pointer.size())).tie(value, code); return
  // std::make_unique<OndemandValue>(std::move(value));
  // return check_unique_ptr<OndemandValue>(
  //     [&] { return doc.at_pointer(rust_str_to_string_view(json_pointer)); },
  //     code);

  return into_unique_ptr_result<OndemandValueResult>(
      doc.at_pointer(rust_str_to_string_view(json_pointer)));
}
std::unique_ptr<OndemandValue>
ondemand_document_find_field(OndemandDocument &doc, const rust::Str key,
                             ErrorCode &code) {
  OndemandValue value;
  doc.find_field(std::string_view(key.data(), key.size())).tie(value, code);
  return std::make_unique<OndemandValue>(std::move(value));
}
std::unique_ptr<OndemandValue>
ondemand_document_find_field_unordered(OndemandDocument &doc,
                                       const rust::Str key, ErrorCode &code) {
  OndemandValue value;
  doc.find_field_unordered(std::string_view(key.data(), key.size()))
      .tie(value, code);
  return std::make_unique<OndemandValue>(std::move(value));
}
std::unique_ptr<OndemandArray>
ondemand_document_get_array(OndemandDocument &doc, ErrorCode &code) {
  OndemandArray arr;
  doc.get_array().tie(arr, code);
  return std::make_unique<OndemandArray>(std::move(arr));
}
uint64_t ondemand_document_get_uint64(OndemandDocument &doc, ErrorCode &code) {
  uint64_t v;
  doc.get_uint64().tie(v, code);
  return v;
}
uint64_t ondemand_document_get_uint64_in_string(OndemandDocument &doc,
                                                ErrorCode &code) {
  uint64_t v;
  doc.get_uint64_in_string().tie(v, code);
  return v;
}
int64_t ondemand_document_get_int64(OndemandDocument &doc, ErrorCode &code) {
  int64_t v;
  doc.get_int64().tie(v, code);
  return v;
}
int64_t ondemand_document_get_int64_in_string(OndemandDocument &doc,
                                              ErrorCode &code) {
  int64_t v;
  doc.get_int64_in_string().tie(v, code);
  return v;
}
double ondemand_document_get_double(OndemandDocument &doc, ErrorCode &code) {
  double v;
  doc.get_double().tie(v, code);
  return v;
}
double ondemand_document_get_double_in_string(OndemandDocument &doc,
                                              ErrorCode &code) {
  double v;
  doc.get_double_in_string().tie(v, code);
  return v;
}
rust::Str ondemand_document_get_string(OndemandDocument &doc, ErrorCode &code) {
  std::string_view sv;
  doc.get_string().tie(sv, code);
  return rust::Str(sv.data(), sv.size());
}
bool ondemand_document_get_bool(OndemandDocument &doc, ErrorCode &code) {
  bool v;
  doc.get_bool().tie(v, code);
  return v;
}
std::unique_ptr<OndemandRawJsonString>
ondemand_document_get_raw_json_string(OndemandDocument &doc, ErrorCode &code) {
  OndemandRawJsonString rjs;
  doc.get_raw_json_string().tie(rjs, code);
  return std::make_unique<OndemandRawJsonString>(std::move(rjs));
}
bool ondemand_document_is_null(OndemandDocument &doc, ErrorCode &code) {
  bool v;
  doc.is_null().tie(v, code);
  return v;
}
OndemandJsonType ondemand_document_type(OndemandDocument &doc,
                                        ErrorCode &code) {
  OndemandJsonType jt;
  doc.type().tie(jt, code);
  return jt;
}
bool ondemand_document_is_scalar(OndemandDocument &doc, ErrorCode &code) {
  return check<bool>([&] { return doc.is_scalar(); }, code);
}
bool ondemand_document_is_negative(OndemandDocument &doc) {
  return doc.is_negative();
}
bool ondemand_document_is_integer(OndemandDocument &doc, ErrorCode &code) {
  return check<bool>([&] { return doc.is_integer(); }, code);
}

// ondemand::value
uint64_t ondemand_value_get_uint64(OndemandValue &value, ErrorCode &code) {
  uint64_t v;
  value.get_uint64().tie(v, code);
  return v;
}
std::unique_ptr<OndemandArray> ondemand_value_get_array(OndemandValue &value,
                                                        ErrorCode &code) {
  OndemandArray arr;
  value.get_array().tie(arr, code);
  return std::make_unique<OndemandArray>(std::move(arr));
}
std::unique_ptr<OndemandObject> ondemand_value_get_object(OndemandValue &value,
                                                          ErrorCode &code) {
  OndemandObject obj;
  value.get_object().tie(obj, code);
  return std::make_unique<OndemandObject>(std::move(obj));
}
std::unique_ptr<OndemandValue> ondemand_value_find_field(OndemandValue &value,
                                                         const rust::Str key,
                                                         ErrorCode &code) {
  OndemandValue v;
  value.find_field(std::string_view(key.data(), key.size())).tie(v, code);
  return std::make_unique<OndemandValue>(std::move(v));
}
std::unique_ptr<OndemandValue>
ondemand_value_find_field_unordered(OndemandValue &value, const rust::Str key,
                                    ErrorCode &code) {
  OndemandValue v;
  value.find_field_unordered(std::string_view(key.data(), key.size()))
      .tie(value, code);
  return std::make_unique<OndemandValue>(std::move(v));
}

// ondemand::object
std::unique_ptr<OndemandValue>
ondemand_object_at_pointer(OndemandObject &obj, const rust::Str json_pointer,
                           ErrorCode &code) {
  OndemandValue v;
  obj.at_pointer(std::string_view(json_pointer.data(), json_pointer.size()))
      .tie(v, code);
  return std::make_unique<OndemandValue>(std::move(v));
}

std::unique_ptr<OndemandObjectIterator>
ondemand_object_begin(OndemandObject &arr, ErrorCode &code) {
  OndemandObjectIterator iter;
  arr.begin().tie(iter, code);
  return std::make_unique<OndemandObjectIterator>(std::move(iter));
}

std::unique_ptr<OndemandObjectIterator> ondemand_object_end(OndemandObject &arr,
                                                            ErrorCode &code) {
  OndemandObjectIterator iter;
  arr.end().tie(iter, code);
  return std::make_unique<OndemandObjectIterator>(std::move(iter));
}
rust::Str ondemand_object_raw_json(OndemandObject &obj, ErrorCode &code) {
  std::string_view rv;
  obj.raw_json().tie(rv, code);
  return rust::Str(rv.data(), rv.size());
}

// ondemand::object_iterator
bool ondemand_object_iterator_not_equal(const OndemandObjectIterator &lhs,
                                        const OndemandObjectIterator &rhs) {
  return lhs != rhs;
}
OndemandObjectIterator &
ondemand_object_iterator_next(OndemandObjectIterator &iter) {
  return ++iter;
}
std::unique_ptr<OndemandField>
ondemand_object_iterator_get(OndemandObjectIterator &iter, ErrorCode &code) {
  OndemandField field;
  (*iter).tie(field, code);
  // OndemandValue value;
  // std::tie(key, value) = field;
  return std::make_unique<OndemandField>(std::move(field));
}

// ondemand::array
std::unique_ptr<OndemandArrayIterator> ondemand_array_begin(OndemandArray &arr,
                                                            ErrorCode &code) {
  OndemandArrayIterator iter;
  arr.begin().tie(iter, code);
  return std::make_unique<OndemandArrayIterator>(std::move(iter));
}
std::unique_ptr<OndemandArrayIterator> ondemand_array_end(OndemandArray &arr,
                                                          ErrorCode &code) {
  OndemandArrayIterator iter;
  arr.end().tie(iter, code);
  return std::make_unique<OndemandArrayIterator>(std::move(iter));
}
std::unique_ptr<OndemandValue>
ondemand_array_at(OndemandArray &arr, size_t index, ErrorCode &code) {
  OndemandValue v;
  arr.at(index).tie(v, code);
  return std::make_unique<OndemandValue>(std::move(v));
}
size_t ondemand_array_count_elements(OndemandArray &arr, ErrorCode &code) {
  size_t n;
  arr.count_elements().tie(n, code);
  return n;
}
bool ondemand_array_is_empty(OndemandArray &arr, ErrorCode &code) {
  bool res;
  arr.is_empty().tie(res, code);
  return res;
}
std::unique_ptr<OndemandValue>
ondemand_array_at_pointer(OndemandArray &arr, const rust::Str json_pointer,
                          ErrorCode &code) {
  OndemandValue v;
  arr.at_pointer(std::string_view(json_pointer.data(), json_pointer.size()))
      .tie(v, code);
  return std::make_unique<OndemandValue>(std::move(v));
}

// ondemand::array_iterator
bool ondemand_array_iterator_equal(const OndemandArrayIterator &lhs,
                                   const OndemandArrayIterator &rhs) {
  return lhs == rhs;
}
bool ondemand_array_iterator_not_equal(const OndemandArrayIterator &lhs,
                                       const OndemandArrayIterator &rhs) {
  return lhs != rhs;
}
OndemandArrayIterator &
ondemand_array_iterator_next(OndemandArrayIterator &iter) {
  return ++iter;
}
std::unique_ptr<OndemandValue>
ondemand_array_iterator_get(OndemandArrayIterator &iter, ErrorCode &code) {
  OndemandValue v;
  (*iter).tie(v, code);
  return std::make_unique<OndemandValue>(std::move(v));
}

// ondemand::field
// rust::Str ondemand_field_unescaped_key(OndemandField &field, ErrorCode &code) {
//   std::string_view sv;
//   field.unescaped_key().tie(sv, code);
//   return rust::Str(sv.data(), sv.size());
// }
std::unique_ptr<OndemandValue> ondemand_field_value(OndemandField &field) {
  return std::make_unique<OndemandValue>(std::move(field.value()));
}

std::unique_ptr<OndemandRawJsonString>
ondemand_field_key(const OndemandField &field) {
  return std::make_unique<OndemandRawJsonString>(std::move(field.key()));
}

// ondemand::raw_json_string
// rust::Str ondemand_raw_json_string_unescape(const OndemandRawJsonString &rjs,
// OndemandValue v, ErrorCode &code) {
//     auto sv = rjs.unesacpe
// }

// padded_string
std::unique_ptr<PaddedString> padded_string_load(const std::string &filename,
                                                 ErrorCode &code) {
  PaddedString ps;
  PaddedString::load(filename).tie(ps, code);
  return std::make_unique<PaddedString>(std::move(ps));
}

std::unique_ptr<PaddedString> padded_string_from_str(const rust::Str s) {

  return std::make_unique<PaddedString>(s.data(), s.size());
}

} // namespace ffi
