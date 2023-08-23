#include "simdjson_c_api.h"
#include "simdjson.h"

#define IMPL_FREE(name)                                                          \
    void SJ_##name##_free(SJ_##name *r)                                          \
    {                                                                            \
        delete reinterpret_cast<simdjson::name *>(r);                            \
    }                                                                            \
    void SJ_simdjson_result_of_##name##_free(SJ_simdjson_result_of_##name *r)    \
    {                                                                            \
        delete reinterpret_cast<simdjson::simdjson_result<simdjson::name> *>(r); \
    }

IMPL_FREE(padded_string)

SJ_padded_string *SJ_padded_string_new(const char *s, size_t len)
{
    return reinterpret_cast<SJ_padded_string *>(new simdjson::padded_string(s, len));
}

// void SJ_padded_string_free(SJ_padded_string* r) {
//     delete reinterpret_cast<simdjson::padded_string*>(r);
// } 
