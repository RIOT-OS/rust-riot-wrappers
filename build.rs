extern crate bindgen;

use bindgen::builder;
use std::env;
use std::path::PathBuf;

fn main() {

    let sourcefile = env::var("RIOT_EXPANDED_HEADER")
        .expect("Please set RIOT_EXPANDED_HEADER, see README for details.");

    println!("cargo:rerun-if-env-changed=RIOT_EXPANDED_HEADER");
    println!("cargo:rerun-if-changed={}", sourcefile);

    let bindings = builder()
        .header(sourcefile)
        .use_core()
        .ctypes_prefix("libc")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
