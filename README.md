# simdjson-rust

[![Github Actions](https://img.shields.io/github/actions/workflow/status/SunDoge/simdjson-rust/CI.yml?branch=master&style=for-the-badge)](https://github.com/SunDoge/simdjson-rust/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/simdjson-rust?style=for-the-badge)](https://crates.io/crates/simdjson-rust)
[![docs.rs](https://img.shields.io/docsrs/simdjson-rust/latest?style=for-the-badge)](https://docs.rs/simdjson-rust)

Rust bindings for the [simdjson][simdjson] C++ library (currently **v4.6.4**),
exposing its DOM parser as a zero-copy tape and integrating with `serde` for
direct deserialization into Rust structs.

## What this crate does

simdjson parses a JSON document in two stages: **stage1** scans the input with
SIMD to find structural characters, **stage2** builds a flat _tape_ — an array
of 64-bit words encoding every value with its type tag and payload, plus a
side string buffer holding all (already-unescaped) string bytes.

This crate hands that tape to Rust as two borrowed slices (`&[u64]` tape +
`&[u8]` string buffer) and lets you walk it with zero copy:

- `dom::Parser` — parse a JSON string/bytes into a `TapeRef` cursor or a
  `tape::Value` DOM tree.
- `tape::Value` — a zero-copy `enum` (`Null`/`Bool`/`Int64`/`Uint64`/`Double`/
  `String(&str)`/`Array`/`Object`) borrowing the parser's memory.
- `serde::from_str` / `from_tape` — deserialize any `serde::Deserialize` type
  directly off the tape, with zero-copy `&'de str` for string fields.

## Quick start

```toml
[dependencies]
simdjson-rust = "0.4.0-alpha.1"
```

```rust
use simdjson_rust::dom::Parser;

let mut parser = Parser::default();
let value = parser.parse_to_value(r#"{"hello": "world", "n": 42}"#).unwrap();
assert_eq!(value.get("n").unwrap().as_i64(), Some(42));
```

With the default `serde` feature, deserialize straight into your own types:

```rust
use serde::Deserialize;
use simdjson_rust::serde::from_str;

#[derive(Deserialize, Debug, PartialEq)]
struct Point { x: f64, y: f64 }

let p: Point = from_str(r#"{"x": 1.0, "y": 2.0}"#).unwrap();
assert_eq!(p, Point { x: 1.0, y: 2.0 });
```

For high-throughput work, reuse the `Parser` across documents and deserialize
from the tape cursor — this avoids re-allocating the parser's internal buffers
on every call:

```rust
use simdjson_rust::dom::Parser;
use simdjson_rust::serde::from_tape;

let mut parser = Parser::default();
for json in inputs {
    let tape = parser.parse_str(json)?;
    let item: MyStruct = from_tape(tape)?;
    // ...
}
```

## Performance

Numbers below are from `cargo bench --bench parser_bench --features native` on
the author's machine (criterion, median). They compare three serde struct
deserialization paths — `serde_json`, the pure-Rust [`simd-json`][simd-json]
crate, and this crate — across three input sizes. Treat them as relative
indicators, not absolute claims; always benchmark on your own hardware and
your own data shape.

| Input           | `serde_json` | `simd-json` | `simdjson-rust` |
| --------------- | ------------ | ----------- | --------------- |
| Small (~100 B)  | 84 ns        | 286 ns      | **110 ns**      |
| Medium (~2 KB)  | ~810 ns      | 1210 ns     | **638 ns**      |
| Large (~100 KB) | 187 µs       | 176 µs      | **137 µs**      |

Where `simdjson-rust` wins:

- **Large inputs**: the C++ stage1+stage2 pass is highly tuned, and the Rust
  tape-walk that follows is pure sequential `&[u64]` reads. On 100 KB the
  crate beats both `serde_json` and `simd-json`.
- **Repeated small parses**: `Parser` is reusable, so the per-call fixed cost
  is low; `serde_json` is still a touch faster on a single 100 B document, but
  `simdjson-rust` pulls ahead once the parser is reused and on medium inputs.
- **Zero-copy strings**: simdjson unescapes strings into `string_buf` during
  stage2, so `&'de str` fields borrow directly with no re-validation or
  allocation.

Where it does _not_ win:

- A single tiny JSON parsed once — `serde_json`'s scalar parser has the lowest
  fixed cost.
- Anything that needs the C++ toolchain at build time (see below).

### How the benchmark is run

The bench lives in `benches/parser_bench.rs` and uses [criterion][criterion].
Each of the three size groups deserializes the same struct shape via all three
parsers. To run it locally:

```sh
# Copy the local tuning config (tunes the Rust code for your CPU).
cp .cargo/config.toml.example .cargo/config.toml
# Run with the C++ side tuned for your CPU too.
cargo bench --bench parser_bench --features native
# Or, portable build (no native tuning):
cargo bench --bench parser_bench
```

Note: `simd-json`'s serde entry point rewrites its `&mut str` input in place,
so each iteration clones the input string for it — a constant tax specific to
that bench function, not the parser. Read its column as "parse + one
`String` clone".

## Build requirements

This crate builds simdjson from source via CMake, so the build host needs a
C++17 compiler and CMake. simdjson is fetched through CMake's `FetchContent`
on first build (no manual download), then cached. There is an opt-in `native`
cargo feature that compiles the C++ kernels with `-march=native` for the
building CPU; it is **off by default** so published binaries stay portable.
See `.cargo/config.toml.example` for the matching Rust `-C target-cpu=native`.

## Why only DOM, not ondemand?

simdjson offers two APIs: **DOM** (parse the whole document into a tape up
front) and **ondemand** (lazily parse only the fields you touch). This crate
currently binds only DOM.

Ondemand would be attractive for "large document, few fields" workloads, but
binding it cleanly is hard:

- simdjson's ondemand layer keeps its structural-index array and stage1 state
  behind `internal::` and private members; there is no stable public surface
  to hand that state to Rust without per-field FFI calls (which dominate the
  cost for small reads).
- Doing it efficiently would mean exposing simdjson's internal stage1
  structural indexes to Rust and re-implementing stage2 (scope walking,
  number parsing, string unescaping) in Rust — a large, internal-API-dependent
  undertaking that would carry a real maintenance burden across simdjson
  upgrades.

For full-document deserialization — the common case and what `serde` is built
for — DOM is both simpler and faster (one C++ pass, then a pure-Rust tape
walk). Ondemand only wins when you deliberately skip most of a large document,
which is a narrower use case.

This is not a closed door. If ondemand support would unblock a real workload
of yours, please open an issue describing your access pattern (document size,
how many fields you read). Concrete demand will re-prioritize the work, and
the likely path is a small patch to simdjson that exposes a stage1-only entry
point — feasible, but not worth building speculatively.

## License

Apache-2.0, same as [simdjson][simdjson].

[simdjson]: https://github.com/simdjson/simdjson
[simd-json]: https://github.com/simd-lite/simd-json
[criterion]: https://github.com/bheisler/criterion.rs
