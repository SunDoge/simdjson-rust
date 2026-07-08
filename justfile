

copy-compile-commands:
    fd -IH compile_commands target/ -x cp {} simdjson-sys/
