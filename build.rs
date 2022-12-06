use cmake::Config;
use cxx_build::CFG;
use std::env;
use std::path::Path;

fn main() {
    // let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    // let simdjson_include_dir = Path::new(&manifest_dir).join("csrc").join("simdjson");
    // CFG.exported_header_dirs.push(&simdjson_include_dir);

    // cxx_build::bridge("src/libsimdjson.rs") // returns a cc::Build
    //     .file("csrc/wrapper.cpp")
    //     .file("csrc/simdjson/simdjson.cpp")
    //     .flag_if_supported("-std=c++20")
    //     .flag_if_supported("/std=c++latest")
    //     .flag_if_supported("-pthread")
    //     .flag_if_supported("-O3")
    //     .compile("simdjson-sys");

    // println!("cargo:rerun-if-changed=csrc/wrapper.cpp");
    // println!("cargo:rerun-if-changed=csrc/wrapper.h");

    let _build = cxx_build::bridge("src/bridge.rs");

    let dst = Config::new(".").build().join("lib");

    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=simdjson_cxx");
}
