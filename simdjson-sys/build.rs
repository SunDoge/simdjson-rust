use bindgen;
use cmake::Config;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    if !Path::new("simdjson/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status()
            .unwrap();
    }

    // out/build/libsimdjson.so
    let mut config = Config::new("simdjson");

    let mut dst = if cfg!(windows) {
        config.define("CMAKE_GENERATOR_PLATFORM", "x64").build()
    } else {
        config.build()
    };

    dst.push("build");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=simdjson");
    println!("cargo:rustc-link-lib=stdc++");

    let mut header_path = PathBuf::from("simdjson");
    header_path.push("singleheader");
    header_path.push("simdjson.h");
    // header_path.push("include");
    // header_path.push("simdjson");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header_path.to_string_lossy())
        // .header_contents("wrapper.h","#include<jsonparser.h>")
        // .clang_arg(format!("-L{}", header_path.display()))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        // .clang_arg("-fno-inline-functions")
        .clang_arg("-fkeep-inline-functions")
        // .clang_arg("-lc++abi")
        // .clang_arg("-lstdc++")
        // .clang_arg("-static")
        // .clang_arg("-stdlib=libc++")
        // .whitelist_var("json_parse_ptr")

        .whitelist_function("simdjson::json_parse")
        .whitelist_function("simdjson::build_parsed_json")
        .whitelist_function("simdjson::allocate_padded_buffer")
        // .whitelist_type("*Iterator")

        // .whitelist_function("aligned_free")
        .generate_inline_functions(true)
        .derive_default(true)
        .opaque_type("std::.*")
        // .whitelist_type("ParsedJson")
        // .opaque_type("ParsedJson")
        // .blacklist_type("std::.*")
        // .blacklist_function("dump")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
