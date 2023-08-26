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
  SJ_OD_value_result *self##_at_pointer(self *r, const char *s, size_t len);

#define DEFINE_GET_PRIMITIVE(self, value, method)                              \
  value self##_##method(self *r);

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
DEFINE_CLASS(SJ_OD_number)
DEFINE_RESULT(SJ_OD_number)

DEFINE_PRIMITIVE_RESULT(uint64_t)
DEFINE_PRIMITIVE_RESULT(int64_t)
DEFINE_PRIMITIVE_RESULT(double)
DEFINE_PRIMITIVE_RESULT(bool)
DEFINE_PRIMITIVE_RESULT(size_t)
DEFINE_PRIMITIVE_RESULT(int)

// padded_string
// SJ_padded_string *SJ_padded_string_new(const char *s, size_t len);
// SJ_padded_string_result *
// SJ_padded_string_load(const char *path); // null terminated string.
// size_t SJ_padded_string_length(const SJ_padded_string *ps);
// const uint8_t *SJ_padded_string_u8data(const SJ_padded_string *ps);

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
DEFINE_GET(SJ_OD_value, bool, is_null)
DEFINE_GET(SJ_OD_value, int, type)
DEFINE_GET(SJ_OD_value, SJ_OD_number, get_number)
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
DEFINE_GET(SJ_OD_document, bool, is_null)
DEFINE_GET(SJ_OD_document, int, type)
DEFINE_GET(SJ_OD_document, SJ_OD_number, get_number)
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

// ondemand::number
DEFINE_GET_PRIMITIVE(SJ_OD_number, int, get_number_type)
DEFINE_GET_PRIMITIVE(SJ_OD_number, uint64_t, get_uint64)
DEFINE_GET_PRIMITIVE(SJ_OD_number, int64_t, get_int64)
DEFINE_GET_PRIMITIVE(SJ_OD_number, double, get_double)

// New macros for dom

#define DEFINE_HANDLE(name)                                                    \
  typedef struct name name;                                                    \
  void name##_free(name *r);

#define DEFINE_HANDLE_RESULT(name)                                             \
  typedef struct name##_result {                                               \
    int error;                                                                 \
    name *value;                                                               \
  } name##_result;

// Add prefix SJ_ so we can use allowlist.
#define DEFINE_PRIMITIVE_RESULT_V2(name)                                       \
  typedef struct SJ_##name##_result {                                          \
    int error;                                                                 \
    name value;                                                                \
  } SJ_##name##_result;

#define DEFINE_GET_V2(self, value, method) value self##_##method(self *r);

DEFINE_HANDLE(SJ_DOM_parser)

DEFINE_HANDLE(SJ_DOM_element)
DEFINE_HANDLE_RESULT(SJ_DOM_element)
DEFINE_HANDLE(SJ_DOM_array)
DEFINE_HANDLE_RESULT(SJ_DOM_array)
DEFINE_HANDLE(SJ_DOM_object)
DEFINE_HANDLE_RESULT(SJ_DOM_object)
DEFINE_HANDLE(SJ_DOM_array_iterator)
DEFINE_HANDLE(SJ_DOM_object_iterator)
DEFINE_HANDLE(SJ_DOM_document)
DEFINE_HANDLE_RESULT(SJ_DOM_document)
DEFINE_HANDLE(SJ_DOM_document_stream)
DEFINE_HANDLE_RESULT(SJ_DOM_document_stream)
DEFINE_HANDLE(SJ_DOM_document_stream_iterator)

DEFINE_PRIMITIVE_RESULT_V2(uint64_t)
DEFINE_PRIMITIVE_RESULT_V2(int64_t)
DEFINE_PRIMITIVE_RESULT_V2(double)
DEFINE_PRIMITIVE_RESULT_V2(bool)
DEFINE_PRIMITIVE_RESULT_V2(size_t)
DEFINE_PRIMITIVE_RESULT_V2(int)

typedef struct SJ_string_view {
  const char *data;
  size_t len;
} SJ_string_view;

typedef struct SJ_string_view_result {
  int error;
  SJ_string_view value;
} SJ_string_view_result;

typedef struct SJ_DOM_key_value_pair {
  SJ_string_view key;
  SJ_DOM_element *value;
} SJ_DOM_key_value_pair;

// dom::parser
SJ_DOM_parser *SJ_DOM_parser_new(size_t max_capacity);
SJ_DOM_element_result SJ_DOM_parser_parse(SJ_DOM_parser *parser,
                                          const char *json, size_t len);
SJ_DOM_element_result SJ_DOM_parser_parse_into_document(SJ_DOM_parser *parser,
                                                      SJ_DOM_document *doc,
                                                      const char *json,
                                                      size_t len);
SJ_DOM_document_stream_result SJ_DOM_parser_parse_many(SJ_DOM_parser *parser,
                                                       const char *json,
                                                       size_t len,
                                                       size_t batch_size);

// dom::element
DEFINE_GET_V2(SJ_DOM_element, int, type)
DEFINE_GET_V2(SJ_DOM_element, SJ_DOM_array_result, get_array)
DEFINE_GET_V2(SJ_DOM_element, SJ_DOM_object_result, get_object)
DEFINE_GET_V2(SJ_DOM_element, SJ_string_view_result, get_string)
DEFINE_GET_V2(SJ_DOM_element, SJ_int64_t_result, get_int64)
DEFINE_GET_V2(SJ_DOM_element, SJ_uint64_t_result, get_uint64)
DEFINE_GET_V2(SJ_DOM_element, SJ_double_result, get_double)
DEFINE_GET_V2(SJ_DOM_element, SJ_bool_result, get_bool)
SJ_DOM_element_result SJ_DOM_element_at_pointer(SJ_DOM_element *element,
                                                const char *s, size_t len);

// dom::array
DEFINE_GET_V2(SJ_DOM_array, SJ_DOM_array_iterator *, begin)
DEFINE_GET_V2(SJ_DOM_array, SJ_DOM_array_iterator *, end)
DEFINE_GET_V2(SJ_DOM_array, size_t, size)
DEFINE_GET_V2(SJ_DOM_array, size_t, number_of_slots)

SJ_DOM_element_result SJ_DOM_array_at(SJ_DOM_array *array, size_t index);
SJ_DOM_element_result SJ_DOM_array_at_pointer(SJ_DOM_array *array,
                                              const char *s, size_t len);

// dom::object
DEFINE_GET_V2(SJ_DOM_object, SJ_DOM_object_iterator *, begin)
DEFINE_GET_V2(SJ_DOM_object, SJ_DOM_object_iterator *, end)
DEFINE_GET_V2(SJ_DOM_object, size_t, size)

SJ_DOM_element_result SJ_DOM_object_at_pointer(SJ_DOM_object *object,
                                               const char *s, size_t len);
SJ_DOM_element_result SJ_DOM_object_at_key(SJ_DOM_object *object, const char *s,
                                           size_t len);
SJ_DOM_element_result
SJ_DOM_object_at_key_case_insensitive(SJ_DOM_object *object, const char *s,
                                      size_t len);

// dom::iterator
DEFINE_GET_V2(SJ_DOM_array_iterator, SJ_DOM_element *, get)
DEFINE_GET_V2(SJ_DOM_array_iterator, void, step)
bool SJ_DOM_array_iterator_not_equal(SJ_DOM_array_iterator *lhs,
                                     SJ_DOM_array_iterator *rhs);

DEFINE_GET_V2(SJ_DOM_object_iterator, SJ_DOM_key_value_pair, get)
DEFINE_GET_V2(SJ_DOM_object_iterator, void, step)
bool SJ_DOM_object_iterator_not_equal(SJ_DOM_object_iterator *lhs,
                                      SJ_DOM_object_iterator *rhs);

// dom::document
SJ_DOM_document *SJ_DOM_document_new();
DEFINE_GET_V2(SJ_DOM_document, SJ_DOM_element *, root)
DEFINE_GET_V2(SJ_DOM_document_stream, SJ_DOM_document_stream_iterator *, begin)
DEFINE_GET_V2(SJ_DOM_document_stream, SJ_DOM_document_stream_iterator *, end)
DEFINE_GET_V2(SJ_DOM_document_stream_iterator, SJ_DOM_element_result, get)
DEFINE_GET_V2(SJ_DOM_document_stream_iterator, void, step)
bool SJ_DOM_document_stream_iterator_not_equal(
    SJ_DOM_document_stream_iterator *lhs, SJ_DOM_document_stream_iterator *rhs);

#ifdef __cplusplus
}
#endif
