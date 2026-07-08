fn main() {
    let dst = cmake::Config::new(".").build();
    let include_dir = dst.join("include");
    let lib_dir = dst.join("lib");

    cxx_build::bridge("src/lib.rs")
        .include("src")
        .include(&include_dir)
        .file("src/simdjson_dom_bridge.cpp")
        .std("c++17")
        .flag_if_supported("/std:c++20") // simdjson uses designated initializers on MSVC.
        .compile("simdjson_sys_bridge");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=simdjson");
    println!("cargo:rerun-if-changed=src/simdjson_dom_bridge.h");
    println!("cargo:rerun-if-changed=src/simdjson_dom_bridge.cpp");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
}
