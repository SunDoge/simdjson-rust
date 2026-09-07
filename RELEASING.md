# Releasing

The next candidate is `0.4.0-alpha.1` for both crates. No release has been
published by preparing this branch.

1. Keep `[workspace.package].version` and the workspace `simdjson-sys`
   dependency version in sync. Update README and CHANGELOG, including the
   intended release date, before making the final release commit.
2. Run `mise run release-check` with the current stable Cargo. Workspace
   publishing stages the local dependency so the unpublished pair can be
   verified together. Also run `cargo +1.88.0 check --workspace --all-targets
   --all-features` to verify the minimum Rust version.
3. Merge the reviewed PR and require Linux, Windows, macOS and MSRV CI to pass
   on master. macOS can also be checked using a manual workflow dispatch.
4. From a clean checkout of the approved commit, run
   `cargo publish --dry-run --workspace` again. Inspect both `.crate` archives
   in `target/package/`, including source provenance and licenses.
5. When explicitly ready to publish, run `cargo publish --workspace`. Cargo
   publishes `simdjson-sys` before `simdjson-rust`. If interrupted after the
   first upload, check crates.io and publish only the missing crate; versions
   cannot be overwritten.
6. Tag the published commit `v0.4.0-alpha.1` and create its GitHub release with
   migration notes. Check docs.rs for both crates.

For an upstream simdjson update, replace both files in
`simdjson-sys/vendor/simdjson` from the same official release, retain its
license, and update the recorded revision. Review the error enum and tape
layout against that revision, then rerun safety regression tests and CI.

Offline verification requires cached Cargo dependencies. After `cargo fetch`,
`cargo test --workspace --offline` builds the vendored C++ source without a
network download. A fresh CMake build also works with
`-DFETCHCONTENT_FULLY_DISCONNECTED=ON`.
