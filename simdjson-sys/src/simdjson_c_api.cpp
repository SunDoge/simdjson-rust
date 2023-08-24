#include "simdjson_c_api.h"
#include "simdjson.h"
#include <cstdint>
#include <string_view>
#include <utility>

using namespace simdjson;

namespace {

template <typename U, typename T> inline U object_to_pointer(T &&t) {
  return reinterpret_cast<U>(new T(std::move(t)));
}

} // namespace

#define IMPL_CLASS(name, type)                                                 \
  void name##_free(name *r) { delete reinterpret_cast<type *>(r); }

// Use value_unsafe because we always check the error code first.
#define IMPL_RESULT(name, type)                                                \
  IMPL_CLASS(name##_result, simdjson_result<type>)                             \
  int name##_result_error(const name##_result *r) {                            \
    auto code = reinterpret_cast<const simdjson_result<type> *>(r)->error();   \
    return static_cast<int>(code);                                             \
  }                                                                            \
  name *name##_result_value_unsafe(name##_result *r) {                         \
    auto result = reinterpret_cast<simdjson_result<type> *>(r);                \
    return object_to_pointer<name *>(std::move(*result).value_unsafe());       \
  }

#define IMPL_PRIMITIVE_RESULT(name)                                            \
  IMPL_CLASS(name##_result, simdjson_result<name>)                             \
  int name##_result_error(const name##_result *r) {                            \
    auto code = reinterpret_cast<const simdjson_result<name> *>(r)->error();   \
    return static_cast<int>(code);                                             \
  }                                                                            \
  name name##_result_value_unsafe(name##_result *r) {                          \
    auto result = reinterpret_cast<simdjson_result<name> *>(r);                \
    return std::move(*result).value_unsafe();                                  \
  }

#define IMPL_AT_POINTER(self, type)                                            \
  SJ_OD_value_result *self##_at_pointer(self *self, const char *s,             \
                                        size_t len) {                          \
    auto result =                                                              \
        reinterpret_cast<type *>(self)->at_pointer(std::string_view(s, len));  \
    return object_to_pointer<SJ_OD_value_result *>(std::move(result));         \
  }

// IMPL_CLASS(SJ_padded_string, padded_string)
// IMPL_RESULT(SJ_padded_string, padded_string)
IMPL_CLASS(SJ_OD_parser, ondemand::parser)
IMPL_CLASS(SJ_OD_document, ondemand::document)
IMPL_RESULT(SJ_OD_document, ondemand::document)
IMPL_CLASS(STD_string_view, std::string_view)
IMPL_RESULT(STD_string_view, std::string_view)
IMPL_CLASS(SJ_OD_value, ondemand::value)
IMPL_RESULT(SJ_OD_value, ondemand::value)
IMPL_CLASS(SJ_OD_array, ondemand::array)
IMPL_RESULT(SJ_OD_array, ondemand::array)
IMPL_CLASS(SJ_OD_object, ondemand::object)
IMPL_RESULT(SJ_OD_object, ondemand::object)
IMPL_CLASS(SJ_OD_raw_json_string, ondemand::raw_json_string)
IMPL_RESULT(SJ_OD_raw_json_string, ondemand::raw_json_string)
IMPL_CLASS(SJ_OD_array_iterator, ondemand::array_iterator)
IMPL_RESULT(SJ_OD_array_iterator, ondemand::array_iterator)
IMPL_CLASS(SJ_OD_object_iterator, ondemand::object_iterator)
IMPL_RESULT(SJ_OD_object_iterator, ondemand::object_iterator)
IMPL_CLASS(SJ_OD_field, ondemand::field)
IMPL_RESULT(SJ_OD_field, ondemand::field)

IMPL_PRIMITIVE_RESULT(uint64_t)
IMPL_PRIMITIVE_RESULT(int64_t)
IMPL_PRIMITIVE_RESULT(double)
IMPL_PRIMITIVE_RESULT(bool)
IMPL_PRIMITIVE_RESULT(size_t)

// SJ_padded_string *SJ_padded_string_new(const char *s, size_t len) {
//   return object_to_pointer<SJ_padded_string *>(padded_string(s, len));
// }

// SJ_padded_string_result *SJ_padded_string_load(const char *path) {
//   return object_to_pointer<SJ_padded_string_result *>(
//       padded_string::load(path));
// }
// size_t SJ_padded_string_length(const SJ_padded_string *ps) {
//   return reinterpret_cast<const padded_string *>(ps)->length();
// }
// const uint8_t *SJ_padded_string_u8data(const SJ_padded_string *ps) {
//   return reinterpret_cast<const padded_string *>(ps)->u8data();
// }

SJ_OD_parser *SJ_OD_parser_new(size_t max_capacity) {
  return object_to_pointer<SJ_OD_parser *>(ondemand::parser(max_capacity));
}

SJ_OD_document_result *
SJ_OD_parser_iterate_padded_string(SJ_OD_parser *parser,
                                   const SJ_padded_string *s) {
  auto doc = reinterpret_cast<ondemand::parser *>(parser)->iterate(
      *reinterpret_cast<const padded_string *>(s));
  // return reinterpret_cast<SJ_OD_document_result *>(new
  // simdjson_result<ondemand::document>(std::move(doc)));
  return object_to_pointer<SJ_OD_document_result *>(std::move(doc));
}

SJ_OD_document_result *
SJ_OD_parser_iterate_padded_string_view(SJ_OD_parser *parser, const char *json,
                                        size_t len, size_t allocated) {
  auto doc = reinterpret_cast<ondemand::parser *>(parser)->iterate(json, len,
                                                                   allocated);
  return object_to_pointer<SJ_OD_document_result *>(std::move(doc));
}

SJ_OD_value_result *SJ_OD_document_get_value(SJ_OD_document *doc) {
  auto value = reinterpret_cast<ondemand::document *>(doc)->get_value();
  return object_to_pointer<SJ_OD_value_result *>(std::move(value));
}

// self, self's real name, output value, how to get output value
#define IMPL_GET(self, real_name, value, method)                               \
  value##_result *self##_##method(self *r) {                                   \
    auto result = reinterpret_cast<real_name *>(r)->method();                  \
    return object_to_pointer<value##_result *>(std::move(result));             \
  }

// ondemand::value
IMPL_GET(SJ_OD_value, ondemand::value, SJ_OD_object, get_object)
IMPL_GET(SJ_OD_value, ondemand::value, SJ_OD_array, get_array)
IMPL_GET(SJ_OD_value, ondemand::value, uint64_t, get_uint64)
IMPL_GET(SJ_OD_value, ondemand::value, int64_t, get_int64)
IMPL_GET(SJ_OD_value, ondemand::value, double, get_double)
IMPL_GET(SJ_OD_value, ondemand::value, SJ_OD_raw_json_string,
         get_raw_json_string)
IMPL_GET(SJ_OD_value, ondemand::value, STD_string_view, get_wobbly_string)
IMPL_AT_POINTER(SJ_OD_value, ondemand::value)

// ondemand::document
IMPL_GET(SJ_OD_document, ondemand::document, SJ_OD_object, get_object)
IMPL_GET(SJ_OD_document, ondemand::document, SJ_OD_array, get_array)
IMPL_GET(SJ_OD_document, ondemand::document, uint64_t, get_uint64)
IMPL_GET(SJ_OD_document, ondemand::document, int64_t, get_int64)
IMPL_GET(SJ_OD_document, ondemand::document, double, get_double)
IMPL_GET(SJ_OD_document, ondemand::document, SJ_OD_raw_json_string,
         get_raw_json_string)
IMPL_GET(SJ_OD_document, ondemand::document, STD_string_view, get_wobbly_string)
IMPL_AT_POINTER(SJ_OD_document, ondemand::document)

STD_string_view_result *SJ_OD_value_get_string(SJ_OD_value *self,
                                               bool allow_replacement) {
  auto result =
      reinterpret_cast<ondemand::value *>(self)->get_string(allow_replacement);
  return object_to_pointer<STD_string_view_result *>(std::move(result));
}

STD_string_view_result *SJ_OD_document_get_string(SJ_OD_document *self,
                                                  bool allow_replacement) {
  auto result = reinterpret_cast<ondemand::document *>(self)->get_string(
      allow_replacement);
  return object_to_pointer<STD_string_view_result *>(std::move(result));
}

const char *STD_string_view_data(STD_string_view *sv) {
  return reinterpret_cast<std::string_view *>(sv)->data();
}

size_t STD_string_view_size(STD_string_view *sv) {
  return reinterpret_cast<std::string_view *>(sv)->size();
}

IMPL_GET(SJ_OD_array, ondemand::array, size_t, count_elements)
IMPL_GET(SJ_OD_array, ondemand::array, bool, is_empty)
IMPL_GET(SJ_OD_array, ondemand::array, bool, reset)
IMPL_GET(SJ_OD_array, ondemand::array, SJ_OD_array_iterator, begin)
IMPL_GET(SJ_OD_array, ondemand::array, SJ_OD_array_iterator, end)
IMPL_GET(SJ_OD_array, ondemand::array, STD_string_view, raw_json)
IMPL_AT_POINTER(SJ_OD_array, ondemand::array)

SJ_OD_value_result *SJ_OD_array_at(SJ_OD_array *array, size_t index) {
  auto result = reinterpret_cast<ondemand::array *>(array)->at(index);
  return object_to_pointer<SJ_OD_value_result *>(std::move(result));
}

// ondemand::array_iterator
SJ_OD_value_result *SJ_OD_array_iterator_get(SJ_OD_array_iterator *self) {
  auto ptr = reinterpret_cast<ondemand::array_iterator *>(self);
  return object_to_pointer<SJ_OD_value_result *>(**ptr);
}
bool SJ_OD_array_iterator_not_equal(const SJ_OD_array_iterator *lhs,
                                    const SJ_OD_array_iterator *rhs) {
  return *reinterpret_cast<const ondemand::array_iterator *>(lhs) !=
         *reinterpret_cast<const ondemand::array_iterator *>(rhs);
}
void SJ_OD_array_iterator_step(SJ_OD_array_iterator *self) {
  auto ptr = reinterpret_cast<ondemand::array_iterator *>(self);
  ++(*ptr);
}

// ondemand::object
IMPL_GET(SJ_OD_object, ondemand::object, SJ_OD_object_iterator, begin)
IMPL_GET(SJ_OD_object, ondemand::object, SJ_OD_object_iterator, end)
IMPL_GET(SJ_OD_object, ondemand::object, STD_string_view, raw_json)
IMPL_GET(SJ_OD_object, ondemand::object, bool, is_empty)
IMPL_GET(SJ_OD_object, ondemand::object, bool, reset)
IMPL_GET(SJ_OD_object, ondemand::object, size_t, count_fields)
IMPL_AT_POINTER(SJ_OD_object, ondemand::object)

SJ_OD_value_result *SJ_OD_object_find_field(SJ_OD_object *object,
                                            const char *data, size_t len) {
  auto result = reinterpret_cast<ondemand::object *>(object)->find_field(
      std::string_view(data, len));
  return object_to_pointer<SJ_OD_value_result *>(std::move(result));
}
SJ_OD_value_result *SJ_OD_object_find_field_unordered(SJ_OD_object *object,
                                                      const char *data,
                                                      size_t len) {
  auto result =
      reinterpret_cast<ondemand::object *>(object)->find_field_unordered(
          std::string_view(data, len));
  return object_to_pointer<SJ_OD_value_result *>(std::move(result));
}

// ondemand::object_iterator
SJ_OD_field_result *SJ_OD_object_iterator_get(SJ_OD_object_iterator *self) {
  auto ptr = reinterpret_cast<ondemand::object_iterator *>(self);
  return object_to_pointer<SJ_OD_field_result *>(**ptr);
}
bool SJ_OD_object_iterator_not_equal(const SJ_OD_object_iterator *lhs,
                                     const SJ_OD_object_iterator *rhs) {
  return *reinterpret_cast<const ondemand::object_iterator *>(lhs) !=
         *reinterpret_cast<const ondemand::object_iterator *>(rhs);
}
void SJ_OD_object_iterator_step(SJ_OD_object_iterator *self) {
  auto ptr = reinterpret_cast<ondemand::object_iterator *>(self);
  ++(*ptr);
}

// ondemand::field
STD_string_view_result *SJ_OD_field_unescaped_key(SJ_OD_field *self,
                                                  bool allow_replacement) {
  auto result = reinterpret_cast<ondemand::field *>(self)->unescaped_key(
      allow_replacement);
  return object_to_pointer<STD_string_view_result *>(std::move(result));
}
SJ_OD_value *SJ_OD_field_value(SJ_OD_field *self) {
  ondemand::value &value = reinterpret_cast<ondemand::field *>(self)->value();
  return reinterpret_cast<SJ_OD_value *>(&value);
}
SJ_OD_value *SJ_OD_field_take_value(SJ_OD_field *self) {
  auto field = reinterpret_cast<ondemand::field *>(self);
  auto value = std::move(*field).value();
  return object_to_pointer<SJ_OD_value *>(std::move(value));
}
