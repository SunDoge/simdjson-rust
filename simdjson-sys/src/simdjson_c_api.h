#pragma once

#include <stddef.h>

#define FUNC_FOR_RESULT(name)                                                                  \
    typedef struct SJ_simdjson_result_of_##name SJ_simdjson_result_of_##name;                  \
    int SJ_simdjson_result_of_##name##_error(const SJ_simdjson_result_of_##name *r); \
    SJ_##name *SJ_simdjson_result_of_##name##_value(const SJ_simdjson_result_of_##name *r);

#define FUNC_FOR_OBJECT(name) \
    typedef struct SJ_##name SJ_##name; \
    void SJ_##name##_free(SJ_##name *r);

#ifdef __cplusplus
extern "C"
{
#endif

    // SJ = simdjson::*
    FUNC_FOR_OBJECT(padded_string)
    FUNC_FOR_RESULT(padded_string)

    SJ_padded_string *SJ_padded_string_new(const char *s, size_t len);
    SJ_simdjson_result_of_padded_string SJ_padded_string_load(const char *path); // null terminated string.



#ifdef __cplusplus
}
#endif
