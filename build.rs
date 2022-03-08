extern crate shlex;

use std::env;

fn main() {
    let cflags = env::var("DEP_RIOT_SYS_CFLAGS")
        .expect("DEP_RIOT_SYS_CFLAGS is not set, check whether riot-sys exports it.");
    let cflags = shlex::split(&cflags).expect("Odd shell escaping in CFLAGS");

    println!("cargo:rerun-if-env-changed=DEP_RIOT_SYS_CFLAGS");

    for flag in cflags.iter() {
        if flag.starts_with("-DMODULE_") {
            // Some modules like cmsis-dsp_StatisticsFunctions have funny characters
            println!(
                "cargo:rustc-cfg=riot_module_{}",
                flag[9..].to_lowercase().replace("-", "_")
            );
        }

        if flag == "-DDEVELHELP" {
            println!("cargo:rustc-cfg=riot_develhelp");
        }
    }

    for (key, _) in env::vars() {
        if let Some(marker) = key.strip_prefix("DEP_RIOT_SYS_MARKER_") {
            println!("cargo:rerun-if-env-changed={}", key);
            // It appears that they get uppercased in Cargo -- but should be lower-case as in the
            // original riot-sys build.rs, especially to not make the cfg statements look weird.
            println!("cargo:rustc-cfg=marker_{}", marker.to_lowercase());
        }
    }
}
