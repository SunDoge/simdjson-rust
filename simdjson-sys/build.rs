use std::{env, path::PathBuf};

fn main() {
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++20") // error C7555: use of designated initializers requires at least '/std:c++20'
        .flag_if_supported("-pthread")
        .flag_if_supported("-O3")
        .flag_if_supported("/O2")
        .flag_if_supported("-DNDEBUG")
        .flag_if_supported("/DNDEBUG")
        .include("simdjson/singleheader")
        .file("src/simdjson_c_api.cpp")
        .file("simdjson/singleheader/simdjson.cpp")
        .cargo_metadata(true)
        .compile("simdjson_c_api");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/simdjson_c_api.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    println!("cargo:rerun-if-changed=src/simdjson_c_api.h");
    println!("cargo:rerun-if-changed=src/simdjson_c_api.cpp");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
