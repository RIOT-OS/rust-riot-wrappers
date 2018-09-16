extern crate bindgen;

use bindgen::builder;
use bindgen::callbacks::{ParseCallbacks, IntKind};
use std::env;

#[derive(Debug)]
struct ReportRiotDefinesAsCfg();

impl ParseCallbacks for ReportRiotDefinesAsCfg {
    /// Derive riot_MODULE_X configuration options from MODULE_X C define headers
    fn int_macro(&self, name: &str, value: i64) -> Option<IntKind> {
        if name.starts_with("MODULE_") && value != 0 {
            println!("cargo:rustc-cfg=riot_{}", name.to_lowercase())
        }
        None
    }

    fn str_macro(&self, name: &str, value: &[u8]) {
        if name == "RIOT_BOARD" {
            let boardname = std::str::from_utf8(value).expect("Board name is not UTF8");
            println!("cargo:rustc-cfg=riot_board=\"{}\"", boardname)
        }
    }
}

fn main() {

    let sourcefile = env::var("RIOT_EXPANDED_HEADER")
        .expect("Please set RIOT_EXPANDED_HEADER, see README of the riot-sys crate for details.");

    println!("cargo:rerun-if-env-changed=RIOT_EXPANDED_HEADER");
    println!("cargo:rerun-if-changed={}", sourcefile);

    builder()
        .header(sourcefile)
        .parse_callbacks(Box::new(ReportRiotDefinesAsCfg()))
        .generate()
        .expect("Unable to generate bindings");
}
