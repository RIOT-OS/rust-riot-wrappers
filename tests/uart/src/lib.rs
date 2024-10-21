#![no_std]
use embedded_io::{Error, Write};
use riot_wrappers::println;
use riot_wrappers::riot_main;
use riot_wrappers::uart::UARTDevice;
use static_cell::StaticCell;

riot_main!(main);

fn main() {
    let mut uart = UARTDevice::from_port(0);
    fn echo_writer(byte: u8) {
        static mut COUNT: u8 = 0;
        let mut _count = unsafe { COUNT };
        let character = byte as char;
        if character.is_ascii_lowercase() {
            println!("{:}:{:}", _count, character.to_ascii_uppercase());
        } else if character.is_ascii_uppercase() {
            println!("{:}:{:}", _count, character.to_ascii_lowercase());
        } else {
            println!("{:}:{:}", _count, character);
        }
        _count += 1;
        unsafe { COUNT = _count };
    }
    static CB: StaticCell<fn(u8)> = StaticCell::new();
    let cb = CB.init(echo_writer);
    let res = uart.init_with_fn(115200, cb);

    // Alternatively, you can use a closure:
    // let res = uart.init_with_closure(115200, |mem: u8| {
    //     println!("Hello from closure!");
    //     echo_writer(mem);
    // });
    match res {
        Ok(_) => {
            let _ = uart.write(b"Uart initialised, type something in!\n");
        }
        Err(err) => println!("Error: {:?}", err.kind()),
    }
}
