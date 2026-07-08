#include "simdjson_dom_bridge.h"

#include "simdjson/dom/element.h"
#include "simdjson/dom/parser.h"

namespace simdjson_sys::dom {

namespace {

constexpr uint64_t JSON_VALUE_MASK = 0x00FFFFFFFFFFFFFF;

size_t roundup_n(size_t value, size_t n) { return (value + n - 1) & ~(n - 1); }

size_t string_buf_capacity(size_t document_capacity) {
  return roundup_n(5 * document_capacity / 3 + simdjson::SIMDJSON_PADDING, 64);
}

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

  const size_t string_len = string_buf_capacity(doc.capacity());
  return TapeView{
      rust::Slice<const uint64_t>(doc.tape.get(), tape_len),
      rust::Slice<const uint8_t>(doc.string_buf.get(), string_len),
  };
}

} // namespace simdjson_sys::dom
