#pragma once

#include "simdjson.h"

namespace simdjson
{
    namespace ffi
    {

        using simdjson::dom::parser;
        void hello();

        std::unique_ptr<parser> parser_new(size_t max_capacity);
    }
} // namespace simdjson