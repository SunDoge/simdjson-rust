#pragma once

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#define DEFINE_CLASS(name)    \
    typedef struct name name; \
    void name##_free(name *r);

// `value` method will take the ownership of T.
#define DEFINE_RESULT(name)                          \
    DEFINE_CLASS(name##_result)                      \
    int name##_result_error(const name##_result *r); \
    name *name##_result_value(name##_result *r);

// `value` method will free simdjson_result.
#define DEFINE_PRIMITIVE_RESULT(name)                \
    DEFINE_CLASS(name##_result)                      \
    int name##_result_error(const name##_result *r); \
    name name##_result_value(name##_result *r);

#define DEFINE_GET(self, value) \
    value##_result *self##_get_##value(self *r);

#ifdef __cplusplus
extern "C"
{
#endif

    // SJ for simdjson, OD for ondemand
    DEFINE_CLASS(SJ_padded_string);
    DEFINE_RESULT(SJ_padded_string);
    DEFINE_CLASS(SJ_OD_parser);
    DEFINE_CLASS(SJ_OD_document);
    DEFINE_RESULT(SJ_OD_document);
    DEFINE_CLASS(SJ_OD_value);
    DEFINE_RESULT(SJ_OD_value);
    DEFINE_CLASS(SJ_OD_array);
    DEFINE_RESULT(SJ_OD_array);
    DEFINE_CLASS(SJ_OD_object);
    DEFINE_RESULT(SJ_OD_object);
    DEFINE_CLASS(SJ_OD_raw_json_string);
    DEFINE_RESULT(SJ_OD_raw_json_string);

    DEFINE_PRIMITIVE_RESULT(uint64_t);
    DEFINE_PRIMITIVE_RESULT(int64_t);
    DEFINE_PRIMITIVE_RESULT(double);
    DEFINE_PRIMITIVE_RESULT(bool);

    SJ_padded_string *SJ_padded_string_new(const char *s, size_t len);
    SJ_padded_string_result *SJ_padded_string_load(const char *path); // null terminated string.

    SJ_OD_parser *SJ_OD_parser_new(size_t max_capacity);
    SJ_OD_document_result *SJ_OD_parser_iterate_padded_string(SJ_OD_parser *parser, const SJ_padded_string *s);
    SJ_OD_document_result *SJ_OD_parser_iterate_padded_string_view(SJ_OD_parser *parser, const char *json, size_t len, size_t allocated);

    SJ_OD_value_result *SJ_OD_document_get_value(SJ_OD_document *doc);
    DEFINE_GET(SJ_OD_value, uint64_t);
    DEFINE_GET(SJ_OD_value, int64_t);
    DEFINE_GET(SJ_OD_value, double);
    DEFINE_GET(SJ_OD_value, bool);
    DEFINE_GET(SJ_OD_value, SJ_OD_array);
    DEFINE_GET(SJ_OD_value, SJ_OD_object);
    DEFINE_GET(SJ_OD_value, SJ_OD_raw_json_string);

    DEFINE_GET(SJ_OD_document, uint64_t);
    DEFINE_GET(SJ_OD_document, int64_t);
    DEFINE_GET(SJ_OD_document, double);
    DEFINE_GET(SJ_OD_document, bool);
    DEFINE_GET(SJ_OD_document, SJ_OD_array);
    DEFINE_GET(SJ_OD_document, SJ_OD_object);
    DEFINE_GET(SJ_OD_document, SJ_OD_raw_json_string);

    typedef struct SJ_string_view
    {
        const char *ptr;
        size_t len;
    } SJ_string_view;
    DEFINE_PRIMITIVE_RESULT(SJ_string_view); // Does not holding the memory.
    SJ_string_view_result *SJ_OD_value_get_string(SJ_OD_value *value, bool allow_replacement);
    SJ_string_view_result *SJ_OD_document_get_string(SJ_OD_document *doc);

    SJ_string_view_result *SJ_OD_value_get_wobbly_string(SJ_OD_value *value);
    SJ_string_view_result *SJ_OD_document_get_wobbly_string(SJ_OD_document *doc);

#ifdef __cplusplus
}
#endif
