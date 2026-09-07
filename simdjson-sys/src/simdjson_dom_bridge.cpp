#include "simdjson_dom_bridge.h"

#include <algorithm>
#include <cstring>

namespace simdjson_sys::dom {

namespace {

// Keep Rust error translations in src/error.rs synchronized with upstream.
static_assert(simdjson::SUCCESS == 0, "simdjson error codes changed");
static_assert(simdjson::CAPACITY == 1, "simdjson error codes changed");
static_assert(simdjson::MEMALLOC == 2, "simdjson error codes changed");
static_assert(simdjson::TAPE_ERROR == 3, "simdjson error codes changed");
static_assert(simdjson::DEPTH_ERROR == 4, "simdjson error codes changed");
static_assert(simdjson::STRING_ERROR == 5, "simdjson error codes changed");
static_assert(simdjson::T_ATOM_ERROR == 6, "simdjson error codes changed");
static_assert(simdjson::F_ATOM_ERROR == 7, "simdjson error codes changed");
static_assert(simdjson::N_ATOM_ERROR == 8, "simdjson error codes changed");
static_assert(simdjson::NUMBER_ERROR == 9, "simdjson error codes changed");
static_assert(simdjson::BIGINT_ERROR == 10, "simdjson error codes changed");
static_assert(simdjson::UTF8_ERROR == 11, "simdjson error codes changed");
static_assert(simdjson::UNINITIALIZED == 12, "simdjson error codes changed");
static_assert(simdjson::EMPTY == 13, "simdjson error codes changed");
static_assert(simdjson::UNESCAPED_CHARS == 14, "simdjson error codes changed");
static_assert(simdjson::UNCLOSED_STRING == 15, "simdjson error codes changed");
static_assert(simdjson::UNSUPPORTED_ARCHITECTURE == 16, "simdjson error codes changed");
static_assert(simdjson::INCORRECT_TYPE == 17, "simdjson error codes changed");
static_assert(simdjson::NUMBER_OUT_OF_RANGE == 18, "simdjson error codes changed");
static_assert(simdjson::INDEX_OUT_OF_BOUNDS == 19, "simdjson error codes changed");
static_assert(simdjson::NO_SUCH_FIELD == 20, "simdjson error codes changed");
static_assert(simdjson::IO_ERROR == 21, "simdjson error codes changed");
static_assert(simdjson::INVALID_JSON_POINTER == 22, "simdjson error codes changed");
static_assert(simdjson::INVALID_URI_FRAGMENT == 23, "simdjson error codes changed");
static_assert(simdjson::UNEXPECTED_ERROR == 24, "simdjson error codes changed");
static_assert(simdjson::PARSER_IN_USE == 25, "simdjson error codes changed");
static_assert(simdjson::OUT_OF_ORDER_ITERATION == 26, "simdjson error codes changed");
static_assert(simdjson::INSUFFICIENT_PADDING == 27, "simdjson error codes changed");
static_assert(simdjson::INCOMPLETE_ARRAY_OR_OBJECT == 28, "simdjson error codes changed");
static_assert(simdjson::SCALAR_DOCUMENT_AS_VALUE == 29, "simdjson error codes changed");
static_assert(simdjson::OUT_OF_BOUNDS == 30, "simdjson error codes changed");
static_assert(simdjson::TRAILING_CONTENT == 31, "simdjson error codes changed");
static_assert(simdjson::OUT_OF_CAPACITY == 32, "simdjson error codes changed");
static_assert(simdjson::NUM_ERROR_CODES == 33, "simdjson error codes changed");

constexpr uint64_t JSON_VALUE_MASK = 0x00FFFFFFFFFFFFFF;

} // namespace

std::unique_ptr<simdjson::dom::parser> parser_new(std::size_t max_capacity) {
  return std::make_unique<simdjson::dom::parser>(max_capacity);
}

int32_t parser_parse(simdjson::dom::parser &parser,
                     rust::Slice<const uint8_t> json,
                     bool realloc_if_needed) {
  simdjson::dom::element element;
  const auto error = parser.parse(json.data(), json.size(), realloc_if_needed)
                         .get(element);
  return static_cast<int32_t>(error);
}

TapeView parser_get_tape_view(const simdjson::dom::parser &parser) {
  const auto &doc = parser.doc;
  if (!doc.tape || !doc.string_buf) {
    return TapeView{
        rust::Slice<const uint64_t>(),
        rust::Slice<const uint8_t>(),
    };
  }

  const size_t tape_len = static_cast<size_t>(doc.tape[0] & JSON_VALUE_MASK);
  if (tape_len == 0) {
    return TapeView{
        rust::Slice<const uint64_t>(),
        rust::Slice<const uint8_t>(),
    };
  }

  // Only expose initialized string bytes, never unused allocation capacity.
  // Numeric payload words are arbitrary bits and must not be read as tags.
  size_t string_len = 0;
  for (size_t i = 1; i < tape_len; ++i) {
    const auto tag = static_cast<uint8_t>(doc.tape[i] >> 56);
    if (tag == '"') {
      const size_t offset = static_cast<size_t>(doc.tape[i] & JSON_VALUE_MASK);
      uint32_t length;
      std::memcpy(&length, doc.string_buf.get() + offset, sizeof(length));
      string_len = std::max(string_len, offset + sizeof(length) + length + 1);
    } else if (tag == 'l' || tag == 'u' || tag == 'd') {
      ++i;
    }
  }
  return TapeView{
      rust::Slice<const uint64_t>(doc.tape.get(), tape_len),
      rust::Slice<const uint8_t>(doc.string_buf.get(), string_len),
  };
}

} // namespace simdjson_sys::dom

namespace simdjson_sys {
int32_t minify(rust::Slice<const uint8_t> json, rust::Slice<uint8_t> output,
               size_t &written) {
  written = 0;
  if (output.size() < json.size()) {
    return static_cast<int32_t>(simdjson::CAPACITY);
  }
  // Avoid passing empty Rust slices' dangling pointers to native code.
  if (json.empty()) {
    return static_cast<int32_t>(simdjson::SUCCESS);
  }
  size_t length = 0;
  const auto error = simdjson::minify(
      reinterpret_cast<const char *>(json.data()), json.size(),
      reinterpret_cast<char *>(output.data()), length);
  if (error == simdjson::SUCCESS) {
    written = length;
  }
  return static_cast<int32_t>(error);
}
} // namespace simdjson_sys
