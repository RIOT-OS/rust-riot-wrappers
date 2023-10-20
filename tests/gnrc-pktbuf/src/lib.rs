#![no_std]

use riot_wrappers::println;
use riot_wrappers::riot_main;

riot_main!(main);

#[track_caller]
fn check() {
    assert!(
        unsafe { riot_sys::gnrc_pktbuf_is_sane() },
        "GNRC packet buffer in unsound state"
    );
}

#[track_caller]
fn check_empty() {
    check();
    assert!(
        unsafe { riot_sys::gnrc_pktbuf_is_empty() },
        "GNRC packet buffer should be empty at this point"
    );
}

fn main() {
    check_empty();

    let snip1 = riot_wrappers::gnrc::pktbuf::Pktsnip::allocate(
        128,
        riot_sys::gnrc_nettype_t_GNRC_NETTYPE_UNDEF,
    )
    .unwrap();
    println!("Allocated pktsnip {:?}", snip1);

    check();

    let snip2 = riot_wrappers::gnrc::pktbuf::Pktsnip::allocate_from(
        &[1, 2, 3, 4],
        riot_sys::gnrc_nettype_t_GNRC_NETTYPE_UNDEF,
    )
    .unwrap();
    println!("Allocated another pktsnip {:?}", snip2);
    let snip2 = snip2
        .add(10, riot_sys::gnrc_nettype_t_GNRC_NETTYPE_UNDEF)
        .unwrap();
    println!("... and prefixed it with 10 bytes into {:?}", snip2);

    check();

    drop(snip1);
    drop(snip2);
    println!("Dropped it");

    check_empty();

    println!("Tests completed.");
}
