

copy-compile-commands:
    fd -IH compile_commands target/ -x cp {} simdjson-sys/

# Run the parser bench tuned for the local CPU.
# Requires `.cargo/config.toml` (copy from .cargo/config.toml.example).
bench:
    cargo bench --bench parser_bench --features native

# Enable local commit message validation after installing the mise tools.
setup-hooks:
    cog install-hook commit-msg

# Validate only commits made after Conventional Commits was introduced.
check-commits:
    cog check conventional-commits-start..HEAD
