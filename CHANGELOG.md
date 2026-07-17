# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Bindings for [simdjson] v0.3.1.
- Use [cxx] instead of [bindgen] because of too many generics.
- **Breaking:** restructured `tape::Error` and `serde::Error` into typed
  variants carrying positions/lengths (`OutOfBounds`, `UnknownTapeType`,
  `UnexpectedTapeType`, `StringOutOfBounds`, `MissingValueWord`,
  `InvalidUtf8`) instead of a single `TapeCorrupted { message: String }`.
  `serde::Error::TapeCorrupted` is removed; tape failures now surface as
  `serde::Error::Tape { source: tape::Error }` (transparent).
- Build tuning: `lto = "fat"` + `codegen-units = 1` on release/bench
  profiles. New opt-in `native` feature compiles the bundled simdjson C++
  with `-march=native`; copy `.cargo/config.toml.example` to
  `.cargo/config.toml` for the matching Rust `-C target-cpu=native`.


## [0.1.0] - 2019-03-13
- Bindings for [simdjson] v0.0.1


[Unreleased]: https://github.com/SunDoge/simdjson-rust 
[0.1.0]: https://github.com/SunDoge/simdjson-rust/releases/tag/v0.1.0

[simdjson]: https://github.com/simdjson/simdjson
[cxx]: https://github.com/dtolnay/cxx
[bindgen]: https://github.com/rust-lang/rust-bindgen