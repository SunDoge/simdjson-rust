#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>

#include "simdjson-sys/src/lib.rs.h"

namespace simdjson_sys::dom {

std::unique_ptr<simdjson::dom::parser> parser_new(std::size_t max_capacity);
int32_t parser_parse(simdjson::dom::parser &parser,
                     rust::Slice<const uint8_t> json, bool realloc_if_needed);
TapeView parser_get_tape_view(const simdjson::dom::parser &parser);

} // namespace simdjson_sys::dom
