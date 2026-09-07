# Releasing

The next candidate is `0.4.0-alpha.3` for both crates. No release has been
published by preparing this branch.

## One-time crates.io setup

For **both** `simdjson-sys` and `simdjson-rust`, open the crate's settings on
crates.io and add a GitHub Actions Trusted Publisher with these exact values:

| Setting | Value |
| --- | --- |
| Repository owner | `SunDoge` |
| Repository name | `simdjson-rust` |
| Workflow filename | `publish.yml` |
| Environment | Leave empty (the job does not use a GitHub Environment) |

Both crates already exist on crates.io. Each needs its own publisher entry;
configuring only the high-level crate will not authorize the dependency upload.
No `CARGO_REGISTRY_TOKEN` GitHub secret is required. The official
[`rust-lang/crates-io-auth-action`](https://github.com/rust-lang/crates-io-auth-action)
exchanges the job's OIDC identity for a temporary token and revokes it at job
completion. See the [crates.io setup guide](https://crates.io/docs/trusted-publishing).

## Release procedure

1. Bump both crates and their internal dependency together:

   ```sh
   cargo release version alpha --workspace
   cargo release version alpha --workspace --execute
   ```

   The first command previews the change. The `version` subcommand does not
   commit, publish, tag or push. Update README, this document, and CHANGELOG
   (including the release date) before making the release commit.
2. Run `mise run release-check` with current stable Cargo. Also run
   `cargo +1.88.0 check --workspace --all-targets --all-features` to verify MSRV.
3. Merge the reviewed PR and require Linux, Windows, macOS and MSRV CI to pass
   on master. macOS can also be checked using a manual workflow dispatch.
4. On the approved master commit, push the version tag, for example:

   ```sh
   git tag v0.4.0-alpha.3
   git push origin v0.4.0-alpha.3
   ```

   **Pushing this tag starts a real crates.io publication.** The
   `.github/workflows/publish.yml` workflow runs one Linux job, verifies that
   the commit belongs to master and the tag matches both crate versions, then
   authenticates and runs `cargo publish --workspace`. Cargo verifies the
   packages and publishes the dependency before the high-level crate.
   Branch pushes and pull requests do not trigger this workflow.
5. After the publishing job succeeds, create the GitHub release for that tag
   with migration notes and check docs.rs for both crates. GitHub release
   creation remains a separate step.

If an upload is interrupted, inspect crates.io before retrying: a timeout may
occur after the server accepts an upload. If only `simdjson-sys` was published,
recover from the same tagged checkout using `cargo publish -p simdjson-rust`
with local publisher credentials. If both versions exist, do not republish.
Never move a tag that has already published a crate version.

For an upstream simdjson update, replace both files in
`simdjson-sys/vendor/simdjson` from the same official release, retain its
license, and update the recorded revision. Review the error enum and tape
layout against that revision, then rerun safety regression tests and CI.

Offline verification requires cached Cargo dependencies. After `cargo fetch`,
`cargo test --workspace --offline` builds the vendored C++ source without a
network download. A fresh CMake build also works with
`-DFETCHCONTENT_FULLY_DISCONNECTED=ON`.
