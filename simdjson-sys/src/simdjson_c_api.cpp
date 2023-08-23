#include "simdjson_c_api.h"
#include "simdjson.h"

using namespace simdjson;

namespace
{
    template <typename U, typename T>
    U *object_to_pointer(T &&t)
    {
        return reinterpret_cast<U *>(new T(std::move(t)));
    }

}

#define IMPL_CLASS(name, type)              \
    void name##_free(name *r)               \
    {                                       \
        delete reinterpret_cast<type *>(r); \
    }

// Use value_unsafe because we always check the error code first.
#define IMPL_RESULT(name, type)                                                  \
    IMPL_CLASS(name##_result, simdjson_result<type>)                             \
    int name##_result_error(const name##_result *r)                              \
    {                                                                            \
        auto code = reinterpret_cast<const simdjson_result<type> *>(r)->error(); \
        return static_cast<int>(code);                                           \
    }                                                                            \
    name *name##_result_value(name##_result *r)                                  \
    {                                                                            \
        auto result = reinterpret_cast<simdjson_result<type> *>(r);              \
        return object_to_pointer<name>(std::move(*result).value_unsafe());       \
    }

#define IMPL_PRIMITIVE_RESULT(name)                                              \
    IMPL_CLASS(name##_result, simdjson_result<name>)                             \
    int name##_result_error(const name##_result *r)                              \
    {                                                                            \
        auto code = reinterpret_cast<const simdjson_result<name> *>(r)->error(); \
        return static_cast<int>(code);                                           \
    }                                                                            \
    name name##_result_value(name##_result *r)                                   \
    {                                                                            \
        auto result = reinterpret_cast<simdjson_result<name> *>(r);              \
        return std::move(*result).value_unsafe();                                \
    }

IMPL_CLASS(SJ_padded_string, padded_string)
IMPL_RESULT(SJ_padded_string, padded_string)
IMPL_CLASS(SJ_OD_parser, ondemand::parser)
IMPL_CLASS(SJ_OD_document, ondemand::document)
IMPL_RESULT(SJ_OD_document, ondemand::document)

IMPL_PRIMITIVE_RESULT(uint64_t)
IMPL_PRIMITIVE_RESULT(int64_t)
IMPL_PRIMITIVE_RESULT(double)
IMPL_PRIMITIVE_RESULT(bool)
IMPL_PRIMITIVE_RESULT(SJ_string_view);

SJ_padded_string *SJ_padded_string_new(const char *s, size_t len)
{
    // return reinterpret_cast<SJ_padded_string *>(new simdjson::padded_string(s, len));
    return object_to_pointer<SJ_padded_string>(padded_string(s, len));
}
SJ_padded_string_result *SJ_padded_string_load(const char *path)
{
    // return reinterpret_cast<SJ_padded_string_result *>(new simdjson_result<padded_string>(padded_string::load(path)));
    return object_to_pointer<SJ_padded_string_result>(padded_string::load(path));
}

SJ_OD_parser *SJ_OD_parser_new(size_t max_capacity)
{
    // return reinterpret_cast<SJ_OD_parser *>(new ondemand::parser(max_capacity));
    return object_to_pointer<SJ_OD_parser>(ondemand::parser(max_capacity));
}

SJ_OD_document_result *SJ_OD_parser_iterate_padded_string(SJ_OD_parser *parser, const SJ_padded_string *s)
{
    auto doc = reinterpret_cast<ondemand::parser *>(parser)->iterate(*reinterpret_cast<const padded_string *>(s));
    // return reinterpret_cast<SJ_OD_document_result *>(new simdjson_result<ondemand::document>(std::move(doc)));
    return object_to_pointer<SJ_OD_document_result>(std::move(doc));
}

SJ_OD_document_result *SJ_OD_parser_iterate_padded_string_view(SJ_OD_parser *parser, const char *json, size_t len, size_t allocated)
{
    auto doc = reinterpret_cast<ondemand::parser *>(parser)->iterate(padded_string_view(json, len, allocated));
    return object_to_pointer<SJ_OD_document_result>(std::move(doc));
}

SJ_OD_value_result *SJ_OD_document_get_value(SJ_OD_document *doc)
{
    auto value = reinterpret_cast<ondemand::document *>(doc)->get_value();
    return object_to_pointer<SJ_OD_value_result>(std::move(value));
}
