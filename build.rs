use cmake::Config;

fn main() {
    let _build = cxx_build::bridge("src/bridge.rs");

    let mut config = Config::new(".");
    let profile = config.get_profile();

    if profile == "Debug" {
        config.define("SIMDJSON_DEVELOPER_MODE", "ON");
    }

    let dst = Config::new(".").build().join("lib");

    println!("cargo:rerun-if-changed=include/simdjson_cxx.h");
    println!("cargo:rerun-if-changed=src/simdjson_cxx.cc");
    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=simdjson_cxx");
}
