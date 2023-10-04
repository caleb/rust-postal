extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=dylib=postal");

    let library = pkg_config::probe_library("libpostal").unwrap();

    let bindings = bindgen::Builder::default()
        .clang_args(library.include_paths.iter().map(|path| format!("-I{}", path.to_string_lossy())))
        .formatter(bindgen::Formatter::Rustfmt)
        .header("wrapper.h")
        .derive_debug(true)
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
