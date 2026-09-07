fn main() {
    let native = std::env::var_os("CARGO_FEATURE_NATIVE").is_some();
    let msvc = std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    // Keep the library and bridge on the same standard. MSVC needs C++20
    // for the designated initializers in simdjson's headers.
    let cxx_standard = if msvc { "20" } else { "17" };

    let mut cmake_cfg = cmake::Config::new(".");
    cmake_cfg
        .define("CMAKE_CXX_STANDARD", cxx_standard)
        .define("CMAKE_CXX_STANDARD_REQUIRED", "ON");
    if msvc {
        // Rust and cc use the non-debug CRT even for debug builds. CMake's
        // default debug CRT would cause runtime/iterator ABI mismatches when
        // linking test executables against the bridge and simdjson.
        let static_crt = std::env::var("CARGO_CFG_TARGET_FEATURE")
            .unwrap_or_default()
            .split(',')
            .any(|feature| feature == "crt-static");
        cmake_cfg.define(
            "CMAKE_MSVC_RUNTIME_LIBRARY",
            if static_crt {
                "MultiThreaded"
            } else {
                "MultiThreadedDLL"
            },
        );
    }
    if native && !msvc {
        // Tune the C++ kernels for the building CPU. Off by default so that
        // published builds remain portable across machines.
        cmake_cfg.cxxflag("-march=native");
    }
    let dst = cmake_cfg.build();
    let include_dir = dst.join("include");
    let lib_dir = dst.join("lib");

    let mut bridge = cxx_build::bridge("src/lib.rs");
    if native && !msvc {
        bridge.flag("-march=native");
    }
    bridge
        .include("src")
        .include(&include_dir)
        .file("src/simdjson_dom_bridge.cpp")
        .std(&format!("c++{cxx_standard}"))
        .compile("simdjson_sys_bridge");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=simdjson");
    println!("cargo:rerun-if-changed=src/simdjson_dom_bridge.h");
    println!("cargo:rerun-if-changed=src/simdjson_dom_bridge.cpp");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=vendor/simdjson");
}
