extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    if cfg!(linux) {
        println!("cargo:rustc-link-lib=bsd");
    }
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate readpassphrase bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings for readpassphrase!");
}

