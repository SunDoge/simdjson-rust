#pragma once

#include "rust/cxx.h"
#include "simdjson.h"

namespace ffi
{
    using OndemandParser = simdjson::ondemand::parser;

    int get_int();

    std::unique_ptr<OndemandParser> new_ondemand_parser(size_t max_capacity);

} // namespace ffi
