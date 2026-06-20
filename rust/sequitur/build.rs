extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let numpy_include_path = String::from_utf8(
        Command::new("python")
            .arg("-c")
            .arg("import numpy; print(numpy.get_include())")
            .output()
            .expect("Failed to get numpy include path")
            .stdout,
    )
    .unwrap();

    cc::Build::new()
        .cpp(true)
        .file("../../Assertions.cc")
        .file("../../Types.cc")
        .file("../../Utility.cc")
        .file("../../Graph.cc")
        .file("../../Multigram.cc")
        .file("../../SequenceModel.cc")
        .file("../../EditDistance.cc")
        .file("../../Estimation.cc")
        .file("../../Translation.cc")
        .file("src/capi.cc")
        .include("../..")
        .include(numpy_include_path.trim())
        .compile("libsequitur.a");

    println!("cargo:rustc-link-lib=static=sequitur");
    println!("cargo:rustc-link-lib=stdc++");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg(format!("-I{}", numpy_include_path.trim()))
        .clang_arg("-I../..")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
