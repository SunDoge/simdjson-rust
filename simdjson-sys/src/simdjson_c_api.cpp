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

// template <typename U, typename T>
// auto simdjson_result_to_struct(simdjson_result<T> &&sr) {
//   T value;
//   const error_code error = std::move(sr).get(value);
//   return {static_cast<int>(error),
//           reinterpret_cast<U>(new T(std::move(value)))};
// }

// template <typename T>
// inline int enum_result_to_number_result(simdjson_result<T>&& enum_result) {
//   T inner;
//   auto error = std::move(enum_result).get(inner);
//   if (error == error_code::SUCCESS) {

//   }
// }

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
  SJ_OD_value_result *self##_at_pointer(self *r, const char *s, size_t len) {  \
    auto result =                                                              \
        reinterpret_cast<type *>(r)->at_pointer(std::string_view(s, len));     \
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
IMPL_CLASS(SJ_OD_number, ondemand::number)
IMPL_RESULT(SJ_OD_number, ondemand::number)

IMPL_PRIMITIVE_RESULT(uint64_t)
IMPL_PRIMITIVE_RESULT(int64_t)
IMPL_PRIMITIVE_RESULT(double)
IMPL_PRIMITIVE_RESULT(bool)
IMPL_PRIMITIVE_RESULT(size_t)
IMPL_PRIMITIVE_RESULT(int)

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
IMPL_GET(SJ_OD_value, ondemand::value, bool, get_bool)
IMPL_GET(SJ_OD_value, ondemand::value, SJ_OD_raw_json_string,
         get_raw_json_string)
IMPL_GET(SJ_OD_value, ondemand::value, STD_string_view, get_wobbly_string)
IMPL_GET(SJ_OD_value, ondemand::value, bool, is_null)
IMPL_GET(SJ_OD_value, ondemand::value, int, type)
IMPL_GET(SJ_OD_value, ondemand::value, SJ_OD_number, get_number)

IMPL_AT_POINTER(SJ_OD_value, ondemand::value)

STD_string_view_result *SJ_OD_value_get_string(SJ_OD_value *self,
                                               bool allow_replacement) {
  auto result =
      reinterpret_cast<ondemand::value *>(self)->get_string(allow_replacement);
  return object_to_pointer<STD_string_view_result *>(std::move(result));
}

// ondemand::document
IMPL_GET(SJ_OD_document, ondemand::document, SJ_OD_object, get_object)
IMPL_GET(SJ_OD_document, ondemand::document, SJ_OD_array, get_array)
IMPL_GET(SJ_OD_document, ondemand::document, uint64_t, get_uint64)
IMPL_GET(SJ_OD_document, ondemand::document, int64_t, get_int64)
IMPL_GET(SJ_OD_document, ondemand::document, double, get_double)
IMPL_GET(SJ_OD_document, ondemand::document, bool, get_bool)
IMPL_GET(SJ_OD_document, ondemand::document, SJ_OD_raw_json_string,
         get_raw_json_string)
IMPL_GET(SJ_OD_document, ondemand::document, STD_string_view, get_wobbly_string)
IMPL_GET(SJ_OD_document, ondemand::document, bool, is_null)
IMPL_GET(SJ_OD_document, ondemand::document, int, type)
IMPL_GET(SJ_OD_document, ondemand::document, SJ_OD_number, get_number)

IMPL_AT_POINTER(SJ_OD_document, ondemand::document)

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

// ondemand::number
#define IMPL_GET_PRIMITIVE(self, real_name, value, method)                     \
  value self##_##method(self *r) {                                             \
    return reinterpret_cast<real_name *>(r)->method();                         \
  }

IMPL_GET_PRIMITIVE(SJ_OD_number, ondemand::number, uint64_t, get_uint64)
IMPL_GET_PRIMITIVE(SJ_OD_number, ondemand::number, int64_t, get_int64)
IMPL_GET_PRIMITIVE(SJ_OD_number, ondemand::number, double, get_double)

int SJ_OD_number_get_number_type(SJ_OD_number *self) {
  return static_cast<int>(
      reinterpret_cast<ondemand::number *>(self)->get_number_type());
}

// New macros for dom
#define IMPL_HANDLE(name, type)                                                \
  void name##_free(name *r) { delete reinterpret_cast<type *>(r); }            \
  inline type *cast_to_type(name *r) { return reinterpret_cast<type *>(r); }   \
  inline name *move_to_handle(type &&r) {                                      \
    return object_to_pointer<name *>(std::move(r));                            \
  }

IMPL_HANDLE(SJ_DOM_parser, dom::parser)
IMPL_HANDLE(SJ_DOM_array, dom::array)
IMPL_HANDLE(SJ_DOM_element, dom::element)
IMPL_HANDLE(SJ_DOM_object, dom::object)
IMPL_HANDLE(SJ_DOM_array_iterator, dom::array::iterator)
IMPL_HANDLE(SJ_DOM_object_iterator, dom::object::iterator)
IMPL_HANDLE(SJ_DOM_document, dom::document)
IMPL_HANDLE(SJ_DOM_document_stream, dom::document_stream)
IMPL_HANDLE(SJ_DOM_document_stream_iterator, dom::document_stream::iterator)

// dom::parser
SJ_DOM_parser *SJ_DOM_parser_new(size_t max_capacity) {
  return object_to_pointer<SJ_DOM_parser *>(dom::parser(max_capacity));
}

SJ_DOM_element_result SJ_DOM_parser_parse(SJ_DOM_parser *parser,
                                          const char *json, size_t len) {
  dom::element value;
  const auto error = reinterpret_cast<dom::parser *>(parser)
                         ->parse(json, len, false)
                         .get(value); // The string is padded, so false.
  return {static_cast<int>(error), move_to_handle(std::move(value))};
}
SJ_DOM_element_result SJ_DOM_parser_parse_into_document(SJ_DOM_parser *parser,
                                                        SJ_DOM_document *doc,
                                                        const char *json,
                                                        size_t len) {
  dom::element value;
  const auto error = cast_to_type(parser)
                         ->parse_into_document(
                             *reinterpret_cast<dom::document *>(doc), json, len)
                         .get(value);
  return {static_cast<int>(error), move_to_handle(std::move(value))};
}
SJ_DOM_document_stream_result SJ_DOM_parser_parse_many(SJ_DOM_parser *parser,
                                                       const char *json,
                                                       size_t len,
                                                       size_t batch_size) {
  dom::document_stream value;
  const auto error =
      cast_to_type(parser)->parse_many(json, len, batch_size).get(value);
  return {static_cast<int>(error), move_to_handle(std::move(value))};
}

// dom::element
int SJ_DOM_element_type(SJ_DOM_element *self) {
  return static_cast<int>(reinterpret_cast<dom::element *>(self)->type());
}

SJ_DOM_array_result SJ_DOM_element_get_array(SJ_DOM_element *self) {
  dom::array res;
  const error_code error = cast_to_type(self)->get_array().get(res);
  return {static_cast<int>(error), move_to_handle(std::move(res))};
}
SJ_DOM_object_result SJ_DOM_element_get_object(SJ_DOM_element *self) {
  dom::object res;
  const error_code error = cast_to_type(self)->get_object().get(res);
  return {static_cast<int>(error), move_to_handle(std::move(res))};
}

SJ_string_view_result SJ_DOM_element_get_string(SJ_DOM_element *self) {
  std::string_view res;
  const error_code error = cast_to_type(self)->get_string().get(res);
  return {static_cast<int>(error), {.data = res.data(), .len = res.size()}};
}

SJ_uint64_t_result SJ_DOM_element_get_uint64(SJ_DOM_element *self) {
  uint64_t res = 0;
  const error_code error = cast_to_type(self)->get_uint64().get(res);
  return {static_cast<int>(error), res};
}
SJ_int64_t_result SJ_DOM_element_get_int64(SJ_DOM_element *self) {
  int64_t res = 0;
  const error_code error = cast_to_type(self)->get_int64().get(res);
  return {static_cast<int>(error), res};
}
SJ_double_result SJ_DOM_element_get_double(SJ_DOM_element *self) {
  double res = 0.0;
  const error_code error = cast_to_type(self)->get_double().get(res);
  return {static_cast<int>(error), res};
}
SJ_DOM_element_result SJ_DOM_element_at_pointer(SJ_DOM_element *self,
                                                const char *json, size_t len) {
  dom::element res;
  const error_code error =
      cast_to_type(self)->at_pointer(std::string_view(json, len)).get(res);
  return {static_cast<int>(error), move_to_handle(std::move(res))};
}

// dom::array
SJ_DOM_array_iterator *SJ_DOM_array_begin(SJ_DOM_array *self) {
  return move_to_handle(cast_to_type(self)->begin());
}
SJ_DOM_array_iterator *SJ_DOM_array_end(SJ_DOM_array *self) {
  return move_to_handle(cast_to_type(self)->end());
}
size_t SJ_DOM_array_size(SJ_DOM_array *self) {
  return cast_to_type(self)->size();
}
size_t SJ_DOM_array_number_of_slots(SJ_DOM_array *self) {
  return cast_to_type(self)->number_of_slots();
}
SJ_DOM_element_result SJ_DOM_array_at(SJ_DOM_array *self, size_t index) {
  dom::element res;
  const error_code error = cast_to_type(self)->at(index).get(res);
  return {static_cast<int>(error), move_to_handle(std::move(res))};
}
SJ_DOM_element_result SJ_DOM_array_at_pointer(SJ_DOM_array *self,
                                              const char *json, size_t len) {
  dom::element res;
  const error_code error =
      cast_to_type(self)->at_pointer(std::string_view(json, len)).get(res);
  return {static_cast<int>(error), move_to_handle(std::move(res))};
}

// dom::array::iterator
SJ_DOM_element *SJ_DOM_array_iterator_get(SJ_DOM_array_iterator *self) {
  return move_to_handle(**cast_to_type(self));
}
bool SJ_DOM_array_iterator_not_equal(SJ_DOM_array_iterator *lhs,
                                     SJ_DOM_array_iterator *rhs) {
  return *cast_to_type(lhs) != *cast_to_type(rhs);
}
void SJ_DOM_array_iterator_step(SJ_DOM_array_iterator *self) {
  ++(*cast_to_type(self));
}

// dom::object
SJ_DOM_object_iterator *SJ_DOM_object_begin(SJ_DOM_object *self) {
  return move_to_handle(cast_to_type(self)->begin());
}
SJ_DOM_object_iterator *SJ_DOM_object_end(SJ_DOM_object *self) {
  return move_to_handle(cast_to_type(self)->end());
}
size_t SJ_DOM_object_size(SJ_DOM_object *self) {
  return cast_to_type(self)->size();
}
SJ_DOM_element_result SJ_DOM_object_at_pointer(SJ_DOM_object *self,
                                               const char *json, size_t len) {
  dom::element res;
  const error_code error =
      cast_to_type(self)->at_pointer(std::string_view(json, len)).get(res);
  return {static_cast<int>(error), move_to_handle(std::move(res))};
}
SJ_DOM_element_result SJ_DOM_object_at_key(SJ_DOM_object *self,
                                           const char *json, size_t len) {
  dom::element res;
  const error_code error =
      cast_to_type(self)->at_key(std::string_view(json, len)).get(res);
  return {static_cast<int>(error), move_to_handle(std::move(res))};
}
SJ_DOM_element_result SJ_DOM_object_at_key_case_insensitive(SJ_DOM_object *self,
                                                            const char *json,
                                                            size_t len) {
  dom::element res;
  const error_code error =
      cast_to_type(self)
          ->at_key_case_insensitive(std::string_view(json, len))
          .get(res);
  return {static_cast<int>(error), move_to_handle(std::move(res))};
}

// dom::object::iterator
SJ_DOM_key_value_pair SJ_DOM_object_iterator_get(SJ_DOM_object_iterator *self) {
  dom::key_value_pair pair = **cast_to_type(self);
  return {.key = {.data = pair.key.data(), .len = pair.key.size()},
          .value = move_to_handle(std::move(pair.value))};
}
bool SJ_DOM_object_iterator_not_equal(SJ_DOM_object_iterator *lhs,
                                      SJ_DOM_object_iterator *rhs) {
  return *cast_to_type(lhs) != *cast_to_type(rhs);
}
void SJ_DOM_object_iterator_step(SJ_DOM_object_iterator *self) {
  ++(*cast_to_type(self));
}

// dom::document
SJ_DOM_document *SJ_DOM_document_new() {
  return object_to_pointer<SJ_DOM_document *>(dom::document());
}

SJ_DOM_element *SJ_DOM_document_root(SJ_DOM_document *self) {
  return move_to_handle(cast_to_type(self)->root());
}
SJ_DOM_document_stream_iterator *
SJ_DOM_document_stream_begin(SJ_DOM_document_stream *self) {
  return move_to_handle(cast_to_type(self)->begin());
}
SJ_DOM_document_stream_iterator *
SJ_DOM_document_stream_end(SJ_DOM_document_stream *self) {
  return move_to_handle(cast_to_type(self)->end());
}
SJ_DOM_element_result
SJ_DOM_document_stream_iterator_get(SJ_DOM_document_stream_iterator *self) {
  dom::element res;
  const error_code error = cast_to_type(self)->operator*().get(res);
  return {static_cast<int>(error), move_to_handle(std::move(res))};
}
void SJ_DOM_document_stream_iterator_step(
    SJ_DOM_document_stream_iterator *self) {
  ++(*cast_to_type(self));
}
bool SJ_DOM_document_stream_iterator_not_equal(
    SJ_DOM_document_stream_iterator *lhs,
    SJ_DOM_document_stream_iterator *rhs) {
  return *cast_to_type(lhs) != *cast_to_type(rhs);
}
