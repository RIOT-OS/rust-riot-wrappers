#![no_std]

use riot_wrappers::println;
use riot_wrappers::riot_main;
use riot_wrappers::uart::UartDevice;

riot_main!(main);

fn main() {
    let mut cb = |new_data| {
        //do something here with the received data
    };
    let mut scoped_main = |self_: &mut UartDevice| loop {
        self_.write(b"Hello from UART")
    };
    let mut uart = UartDevice::new_scoped(0, 115200, &mut cb, scoped_main)
        .unwrap_or_else(|e| panic!("Error initializing UART: {e:?}"));
}
