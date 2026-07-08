

copy-compile-commands:
    fd -IH compile_commands target/ -x cp {} simdjson-sys/

# Run the parser bench tuned for the local CPU.
# Requires `.cargo/config.toml` (copy from .cargo/config.toml.example).
bench:
    cargo bench --bench parser_bench --features native
