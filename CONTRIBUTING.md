# Contributing

New commits must follow [Conventional Commits](https://www.conventionalcommits.org/):

```text
<type>[optional scope][!]: <description>
```

Use `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`,
`chore`, or `revert`. For example:

```text
fix(serde): handle empty arrays
ci: update the Windows runner
feat!: change the parser API
```

Install Cocogitto and enable the local commit message hook:

```sh
mise install cocogitto
mise exec -- cog install-hook commit-msg
```

Normal `git commit` commands are then checked before the commit is created.
Alternatively, use `cog commit` to compose a conventional message.

To run the same history check as CI:

```sh
mise exec -- cog check conventional-commits-start..HEAD
```

The fixed `conventional-commits-start` tag marks the commit introducing this
policy. Earlier history is excluded; old messages do not need to be rewritten.
Keep this tag when cloning or fetching,
and do not move it. It is a migration marker, not a release tag.

CI checks all new non-merge commits on pushes and pull requests. Git-generated
merge commits are ignored. When squash-merging, use a conventional PR title as
the resulting commit message. Hooks are local to each clone, so contributors
must install them once; CI also checks commits made without the hook.

## CI coverage

Linux (GCC and LLVM) and Windows (MSVC and LLVM) run on branch pushes and pull
requests. macOS runs only on pushes to `master` or when CI is manually triggered
from the Actions page. New pushes cancel outdated runs for the same branch or PR.
Rust dependencies and build outputs are cached by
`actions-rust-lang/setup-rust-toolchain`, with separate keys for C++ compilers.
