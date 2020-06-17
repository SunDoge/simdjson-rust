use std::env;
use std::fs;
use std::path::PathBuf;

// fn copy_header(header: &str, out_dir: &PathBuf) {
//     let mut header_out_path = out_dir.clone();
//     header_out_path.push(header);
//     fs::copy(format!("csrc/{}", &header), &header_out_path).unwrap();
// }

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut out_path = PathBuf::from(&out_dir);
    out_path.push("simdjson-rust");
    out_path.push("src");
    out_path.push("csrc");

    let mut include_path = PathBuf::from(&out_dir);
    include_path.push("src");

    let mut lib_path = PathBuf::from(&out_dir);
    lib_path.push("simdjson-rust");
    lib_path.push("src");

    cxx_build::bridge("src/libsimdjson.rs") // returns a cc::Build
        .include(&include_path)
        // when using as a lib, the path change. out/src => out/simdjson-rust/src
        .include(&lib_path)
        .include("csrc")
        .file("csrc/wrapper.cpp")
        .file("csrc/simdjson.cpp")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++latest") // For windows
        .flag_if_supported("-pthread")
        .flag_if_supported("-O 3")
        .compile("simdjson-sys");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=csrc/wrapper.cpp");
    println!("cargo:rerun-if-changed=csrc/wrapper.h");
}
