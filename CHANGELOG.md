# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### 0.4.0-alpha.1 (release candidate, not yet published)

- Update the CXX DOM bindings to simdjson v4.6.4, exposing a borrowed tape,
  `Value` tree, and optional Serde deserialization. Both crates now use the
  same version. This prerelease includes breaking API changes.
- Require Rust 1.88 or newer and verify it in CI.
- Bundle fixed upstream C++ sources and license in `simdjson-sys`; remove
  build-time downloads and the obsolete C API/submodule.
- **Breaking:** mark `parse_bytes_with_padding` and low-level parse/tape
  accessors unsafe, documenting their contracts. Safe parser methods handle
  initialized padding and invalidate the tape after a failed parse.
- Expose only initialized string-buffer bytes; validate UTF-8 and bounds
  when reading strings from user-constructed tapes.
- Correct simdjson error-code offsets and cover all v4.6.4 error codes,
  including `BIGINT_ERROR` and codes following `UNEXPECTED_ERROR`.
- Replace generic tape errors with typed variants carrying positions and
  lengths. Serde propagates these through `serde::Error::Tape`.
- Gate Serde-dependent examples and benchmarks so no-default-feature builds
  work. Test default, all-feature and no-default-feature configurations.
- Keep CPU-specific tuning opt-in through `native` (non-MSVC C++ targets).
  Release and benchmark profiles use fat LTO and one codegen unit.
- Adopt mise tasks and Conventional Commits; run macOS CI only on master
  pushes and manual dispatches.

## [0.1.0] - 2019-03-13
- Bindings for [simdjson] v0.0.1


[Unreleased]: https://github.com/SunDoge/simdjson-rust 
[0.1.0]: https://github.com/SunDoge/simdjson-rust/releases/tag/v0.1.0

[simdjson]: https://github.com/simdjson/simdjson
[cxx]: https://github.com/dtolnay/cxx
[bindgen]: https://github.com/rust-lang/rust-bindgen