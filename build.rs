fn main() {
    cxx_build::bridge("src/libsimdjson.rs") // returns a cc::Build
        .file("csrc/wrapper.cpp")
        .file("csrc/simdjson.cpp")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++latest") // For windows
        .flag_if_supported("-pthread")
        .compile("simdjson-sys");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=csrc/wrapper.cpp");
    println!("cargo:rerun-if-changed=csrc/wrapper.h");
}
