extern crate shlex;

use std::env;

fn main() {
    let cflags = env::var("DEP_RIOT_SYS_CFLAGS")
        .expect("DEP_RIOT_SYS_CFLAGS is not set, check whether riot-sys exports it.");
    let cflags = shlex::split(&cflags).expect("Odd shell escaping in CFLAGS");

    println!("cargo:rerun-if-env-changed=DEP_RIOT_SYS_CFLAGS");

//     let mut riot_version_count = 0;

    for flag in cflags.iter() {
        if flag.starts_with("-DMODULE_") {
            // Some modules like cmsis-dsp_StatisticsFunctions have funny characters
            println!("cargo:rustc-cfg=riot_module_{}", flag[9..].to_lowercase().replace("-", "_"));
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

//         if flag.starts_with("-DRIOT_VERSION=") {
//             let tail = &flag[15..];
//             let uptodash = tail
//                 .split(|x| x == '-')
//                 .next()
//                 .expect("Failed to parse RIOT_VERSION"); // Ignoring anything behind the dash
//             let numeric: Vec<u32> = uptodash
//                 .split(|x| x == '.')
//                 .map(|x| x.parse())
//                 .collect::<Result<_, _>>()
//                 .expect("Failed to parse RIOT_VERSION");
//             if numeric < vec![2019, 10] {
//                 println!("cargo:rustc-cfg=riot_version_pre2019_10");
//             }
//             riot_version_count += 1;
//         }
    }

//     if riot_version_count != 1 {
//         panic!("RIOT_VERSION missing from the defines.");
//     }
}
