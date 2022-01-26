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

        if flag.starts_with("-DRIOT_BOARD=") {
            println!(
                "cargo:rustc-cfg=riot_board=\"{}\"",
                flag[13..].to_lowercase()
            );
        }

        if flag.starts_with("-DRIOT_CPU=") {
            println!("cargo:rustc-cfg=riot_cpu=\"{}\"", flag[11..].to_lowercase());
        }

        if flag == "-DDEVELHELP" {
            println!("cargo:rustc-cfg=riot_develhelp");
        }
    }

    for (key, _) in env::vars() {
        if let Some(marker) = key.strip_prefix("DEP_RIOT_SYS_MARKER_") {
            println!("cargo:rerun-if-env-changed={}", key);
            println!("cargo:rustc-cfg=marker_{}", marker);
        }
    }
}
