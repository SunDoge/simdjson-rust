#include "include/simdjson_cxx.h"

namespace ffi
{
    int get_int()
    {
        return 1;
    }

    std::unique_ptr<OndemandParser> new_ondemand_parser(size_t max_capacity)
    {
        return std::make_unique<OndemandParser>(max_capacity);
    }
} // namespace ffi
