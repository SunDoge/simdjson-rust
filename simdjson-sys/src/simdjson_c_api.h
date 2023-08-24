#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define DEFINE_CLASS(name)                                                     \
  typedef struct name name;                                                    \
  void name##_free(name *r);

// `value` method will take the ownership of T.
#define DEFINE_RESULT(name)                                                    \
  DEFINE_CLASS(name##_result)                                                  \
  int name##_result_error(const name##_result *r);                             \
  name *name##_result_value_unsafe(name##_result *r);

// `value` method will free simdjson_result.
#define DEFINE_PRIMITIVE_RESULT(name)                                          \
  DEFINE_CLASS(name##_result)                                                  \
  int name##_result_error(const name##_result *r);                             \
  name name##_result_value_unsafe(name##_result *r);

#define DEFINE_GET(self, value, method)                                        \
  value##_result *self##_##method(self *r);

#define DEFINE_AT_POINTER(self)                                                \
  SJ_OD_value_result *self##_at_pointer(self *self, const char *s, size_t len);

#ifdef __cplusplus
extern "C" {
#endif

// SJ for simdjson, OD for ondemand
DEFINE_CLASS(SJ_padded_string)
DEFINE_RESULT(SJ_padded_string)
DEFINE_CLASS(SJ_OD_parser)
DEFINE_CLASS(SJ_OD_document)
DEFINE_RESULT(SJ_OD_document)
DEFINE_CLASS(SJ_OD_value)
DEFINE_RESULT(SJ_OD_value)
DEFINE_CLASS(SJ_OD_array)
DEFINE_RESULT(SJ_OD_array)
DEFINE_CLASS(SJ_OD_object)
DEFINE_RESULT(SJ_OD_object)
DEFINE_CLASS(SJ_OD_raw_json_string)
DEFINE_RESULT(SJ_OD_raw_json_string)
DEFINE_CLASS(STD_string_view)
DEFINE_RESULT(STD_string_view)
DEFINE_CLASS(SJ_OD_array_iterator)
DEFINE_RESULT(SJ_OD_array_iterator)
DEFINE_CLASS(SJ_OD_object_iterator)
DEFINE_RESULT(SJ_OD_object_iterator)
DEFINE_CLASS(SJ_OD_field)
DEFINE_RESULT(SJ_OD_field)

DEFINE_PRIMITIVE_RESULT(uint64_t)
DEFINE_PRIMITIVE_RESULT(int64_t)
DEFINE_PRIMITIVE_RESULT(double)
DEFINE_PRIMITIVE_RESULT(bool)
DEFINE_PRIMITIVE_RESULT(size_t)

// padded_string
SJ_padded_string *SJ_padded_string_new(const char *s, size_t len);
SJ_padded_string_result *
SJ_padded_string_load(const char *path); // null terminated string.
size_t SJ_padded_string_length(const SJ_padded_string *ps);
const uint8_t *SJ_padded_string_u8data(const SJ_padded_string *ps);

// ondemand::parser
SJ_OD_parser *SJ_OD_parser_new(size_t max_capacity);
SJ_OD_document_result *
SJ_OD_parser_iterate_padded_string(SJ_OD_parser *parser,
                                   const SJ_padded_string *s);
SJ_OD_document_result *
SJ_OD_parser_iterate_padded_string_view(SJ_OD_parser *parser, const char *json,
                                        size_t len, size_t allocated);

// ondemand::value
DEFINE_GET(SJ_OD_value, uint64_t, get_uint64)
DEFINE_GET(SJ_OD_value, int64_t, get_int64)
DEFINE_GET(SJ_OD_value, double, get_double)
DEFINE_GET(SJ_OD_value, bool, get_bool)
DEFINE_GET(SJ_OD_value, SJ_OD_array, get_array)
DEFINE_GET(SJ_OD_value, SJ_OD_object, get_object)
DEFINE_GET(SJ_OD_value, SJ_OD_raw_json_string, get_raw_json_string)
DEFINE_GET(SJ_OD_value, STD_string_view, get_wobbly_string)
DEFINE_AT_POINTER(SJ_OD_value)

// ondemand::document
SJ_OD_value_result *SJ_OD_document_get_value(SJ_OD_document *doc);
DEFINE_GET(SJ_OD_document, uint64_t, get_uint64)
DEFINE_GET(SJ_OD_document, int64_t, get_int64)
DEFINE_GET(SJ_OD_document, double, get_double)
DEFINE_GET(SJ_OD_document, bool, get_bool)
DEFINE_GET(SJ_OD_document, SJ_OD_array, get_array)
DEFINE_GET(SJ_OD_document, SJ_OD_object, get_object)
DEFINE_GET(SJ_OD_document, SJ_OD_raw_json_string, get_raw_json_string)
DEFINE_GET(SJ_OD_document, STD_string_view, get_wobbly_string)
DEFINE_AT_POINTER(SJ_OD_document)

// get_string is special.
STD_string_view_result *SJ_OD_value_get_string(SJ_OD_value *value,
                                               bool allow_replacement);
STD_string_view_result *SJ_OD_document_get_string(SJ_OD_document *doc);

// std::string_view
const char *STD_string_view_data(STD_string_view *sv);
size_t STD_string_view_size(STD_string_view *sv);

// ondemand::array
DEFINE_GET(SJ_OD_array, size_t, count_elements)
DEFINE_GET(SJ_OD_array, bool, is_empty)
DEFINE_GET(SJ_OD_array, bool, reset)
DEFINE_GET(SJ_OD_array, SJ_OD_array_iterator, begin)
DEFINE_GET(SJ_OD_array, SJ_OD_array_iterator, end)
DEFINE_GET(SJ_OD_array, STD_string_view, raw_json)
DEFINE_AT_POINTER(SJ_OD_array)

SJ_OD_value_result *SJ_OD_array_at(SJ_OD_array *array, size_t index);

// ondemand::array_iterator
DEFINE_GET(SJ_OD_array_iterator, SJ_OD_value, get)
bool SJ_OD_array_iterator_not_equal(const SJ_OD_array_iterator *lhs,
                                    const SJ_OD_array_iterator *rhs);
void SJ_OD_array_iterator_step(SJ_OD_array_iterator *self);

// ondemand::object
DEFINE_GET(SJ_OD_object, SJ_OD_object_iterator, begin)
DEFINE_GET(SJ_OD_object, SJ_OD_object_iterator, end)
DEFINE_GET(SJ_OD_object, STD_string_view, raw_json)
DEFINE_GET(SJ_OD_object, bool, is_empty)
DEFINE_GET(SJ_OD_object, bool, reset)
DEFINE_GET(SJ_OD_object, size_t, count_fields)
DEFINE_AT_POINTER(SJ_OD_object)

SJ_OD_value_result *SJ_OD_object_find_field(SJ_OD_object *object,
                                            const char *data, size_t len);
SJ_OD_value_result *SJ_OD_object_find_field_unordered(SJ_OD_object *object,
                                                      const char *data,
                                                      size_t len);

// ondemand::object_iterator
DEFINE_GET(SJ_OD_object_iterator, SJ_OD_field, get)
bool SJ_OD_object_iterator_not_equal(const SJ_OD_object_iterator *lhs,
                                     const SJ_OD_object_iterator *rhs);
void SJ_OD_object_iterator_step(SJ_OD_object_iterator *self);

// ondemand::field
STD_string_view_result *SJ_OD_field_unescaped_key(SJ_OD_field *self,
                                                  bool allow_replacement);
SJ_OD_value *SJ_OD_field_value(SJ_OD_field *self);
SJ_OD_value *SJ_OD_field_take_value(SJ_OD_field *self);

#ifdef __cplusplus
}
#endif
