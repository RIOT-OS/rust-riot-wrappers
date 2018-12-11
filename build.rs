extern crate shlex;

use std::env;

fn main() {
    let cflags = env::var("RIOT_CFLAGS")
        .expect("Please pass in RIOT_CFLAGS -- see README.md of the riot-sys crate");
    let cflags = shlex::split(&cflags).expect("Odd shell escaping in RIOT_CFLAGS");

    println!("cargo:rerun-if-env-changed=RIOT_CFLAGS");

    for flag in cflags.iter() {
        if flag.starts_with("-DMODULE_") {
            println!("cargo:rustc-cfg=riot_module_{}", flag[9..].to_lowercase());
        }

        if flag.starts_with("-DRIOT_BOARD=") {
            println!(
                "cargo:rustc-cfg=riot_board=\"{}\"",
                flag[13..].to_lowercase()
            );
        }
    }
}
