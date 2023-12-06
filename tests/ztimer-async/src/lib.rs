#![no_std]
#![feature(type_alias_impl_trait)] // for the embassy-executor/nightly ::task macro

use riot_wrappers::println;
use riot_wrappers::riot_main;

riot_main!(main);

fn main() -> ! {
    // Could also use static-on-stack crate instead
    static EXECUTOR: static_cell::StaticCell<embassy_executor_riot::Executor> =
        static_cell::StaticCell::new();
    let executor: &'static mut _ = EXECUTOR.init(embassy_executor_riot::Executor::new());
    executor.run(|spawner| {
        spawner.spawn(amain(spawner));
    })
}

#[embassy_executor::task]
async fn amain(spawner: embassy_executor::Spawner) {
    use riot_wrappers::ztimer::*;

    println!("Waiting 500 ticks on the msec timer before doing anything else");
    Clock::msec().sleep_async(Ticks(500)).await;
    println!("And now for something more complex...");

    spawner.spawn(ten_tenths());
    spawner.spawn(five_fifths());
}

#[embassy_executor::task]
async fn ten_tenths() {
    use riot_wrappers::ztimer::*;

    println!("A: Will wake up 10x in 1s");
    for _ in 0..10 {
        Clock::msec().sleep_async(Ticks(100)).await;
        println!("A: tick");
    }
    println!("A: Done");
}

#[embassy_executor::task]
async fn five_fifths() {
    use riot_wrappers::ztimer::*;

    println!("B: Will wake up 5x in 1s");
    for _ in 0..5 {
        Clock::msec().sleep_async(Ticks(200)).await;
        println!("B: tick");
    }
    println!("B: Done");
}
