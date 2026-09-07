# simdjson-sys

Low-level CXX bindings to the simdjson v4.6.4 DOM parser. Version
0.4.0-alpha.2 accompanies simdjson-rust 0.4.0-alpha.2.

Requires Rust 1.88+, CMake 3.15+, and a C++17 compiler (C++20 for MSVC targets).
The published crate bundles unmodified upstream singleheader sources; the C++
build does not need network access. See `vendor/simdjson/README.md` for the
upstream revision and `vendor/simdjson/LICENSE` for its Apache-2.0 license.
These Rust bindings are also Apache-2.0; see `LICENSE.txt`.

## Safety

`parser_parse` is unsafe. When `realloc_if_needed` is false, the allocation
must provide at least 64 initialized readable bytes beyond the input slice.
With true, simdjson copies the input into its own padded buffer.

`parser_get_tape_view` is unsafe and requires a successful last parse, without
subsequent parser mutation. The returned slices borrow the parser and expose
only initialized tape and string-buffer contents. Use `simdjson-rust::dom::Parser`
for a safe API that tracks parser validity and handles padding.

The optional `native` feature enables `-march=native` for the library and bridge
on non-MSVC targets. Leave it disabled when distributing portable binaries.

## Minification

`minify_ffi::minify(json, output, written)` is a safe low-level binding to
simdjson's text minifier. It requires no input padding and checks that the
output slice has at least the input length, returning `CAPACITY` otherwise.
On success, `written` is the output length; on error it is zero and output
contents are unspecified. It does not validate JSON syntax or UTF-8.
