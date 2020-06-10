#include "wrapper.h"
#include <iostream>

namespace simdjson
{
    namespace ffi
    {
        using simdjson::dom::parser;
        void hello()
        {
            std::cout << "fuck" << std::endl;
        }

        std::unique_ptr<parser> parser_new(size_t max_capacity) {
            return std::make_unique<parser>(max_capacity);
        }

    } // namespace ffi
} // namespace demo