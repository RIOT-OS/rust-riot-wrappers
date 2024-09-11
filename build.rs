extern crate shlex;

use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=RIOTBUILD_CONFIG_HEADER_C");
    let riotbuildh = env::var("RIOTBUILD_CONFIG_HEADER_C")
        .expect("riotbuild.h is expected to be indicated in the RIOTBUILD_CONFIG_HEADER_C environment variable, or another source of enabled modules provided.");
    println!("cargo:rerun-if-changed={riotbuildh}");

    let mut defines = std::collections::HashMap::new();

    use std::io::BufRead;
    for line in std::io::BufReader::new(
        std::fs::File::open(riotbuildh)
            .expect("Failed to read riotbuild.h (RIOTBUILD_CONFIG_HEADER_C)"),
    )
    .lines()
    {
        let line = line.expect("Error reading line from riotbuild.h (RIOTBUILD_CONFIG_HEADER_C)");
        if let Some(name) = line.strip_prefix("#undef ") {
            defines.remove(name.trim());
        }
        if let Some((name, val)) = line
            .strip_prefix("#define ")
            .and_then(|nv| nv.split_once(" "))
        {
            defines.insert(name.trim().to_owned(), val.trim().to_owned());
        }
    }

    const BOOLEAN_FLAGS: &[&str] = &[
        // This decides whether or not some fields are populated ... and unlike with other
        // structs, the zeroed default is not a good solution here. (It'd kind of work, but
        // it'd produce incorrect debug output).
        "CONFIG_AUTO_INIT_ENABLE_DEBUG",
    ];

    for marker in BOOLEAN_FLAGS {
        println!(
            "cargo::rustc-check-cfg=cfg(marker_{})",
            marker.to_lowercase()
        );
    }

    for (def, val) in defines {
        if val != "1" {
            // So far, only processing boolean flags
            continue;
        }
        if let Some(module) = def.strip_prefix("MODULE_") {
            // Some modules like cmsis-dsp_StatisticsFunctions have funny characters
            println!(
                "cargo:rustc-cfg=riot_module_{}",
                module.to_lowercase().replace("-", "_")
            );
        }
        if def == "DEVELHELP" {
            println!("cargo:rustc-cfg=riot_develhelp");
        }
        if BOOLEAN_FLAGS.contains(&def.as_str()) {
            println!("cargo:rustc-cfg=marker_{}", def.to_lowercase());
        }
    }

    println!("cargo::rustc-check-cfg=cfg(riot_develhelp)");

    // FIXME: This list is currently maintained manually;
    let known_modules = &[
        "auto_init",
        "auto_init_random",
        "bluetil_ad",
        "core_msg",
        "gcoap",
        "gnrc",
        "gnrc_icmpv6",
        "gnrc_ipv6_nib",
        "gnrc_netapi_callbacks",
        "gnrc_nettype_ccn",
        "gnrc_nettype_custom",
        "gnrc_nettype_gomach",
        "gnrc_nettype_icmpv6",
        "gnrc_nettype_ipv6",
        "gnrc_nettype_ipv6_ext",
        "gnrc_nettype_lwmac",
        "gnrc_nettype_ndn",
        "gnrc_nettype_sixlowpan",
        "gnrc_nettype_tcp",
        "gnrc_nettype_udp",
        "gnrc_pktbuf",
        "gnrc_udp",
        "ipv6",
        "microbit",
        "nimble_host",
        "periph_adc",
        "periph_dac",
        "periph_gpio",
        "periph_i2c",
        "periph_spi",
        "prng_shaxprng",
        "pthread",
        "random",
        "saul",
        "shell",
        "sock",
        "sock_aux_local",
        "sock_tcp",
        "sock_udp",
        "udp",
        "vfs",
        "ws281x",
        "ztimer",
        "ztimer_msec",
        "ztimer_periodic",
        "ztimer_sec",
        "ztimer_usec",
    ];

    for module in known_modules {
        println!("cargo::rustc-check-cfg=cfg(riot_module_{})", module);
    }
}
