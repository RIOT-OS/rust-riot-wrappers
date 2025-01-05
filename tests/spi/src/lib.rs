//! This is a primitive SPI test.
//!
//! It performs different reads and writes in different modes. The precise results depend on
//! the voltage on the MISO pin, which is usually indeterminate.
#![no_std]

use embedded_hal::spi::SpiDevice;

use riot_wrappers::println;
use riot_wrappers::riot_main;

riot_main!(main);

fn main() {
    // SPI device and CS pin that the test may drive.
    //
    // Boards should only be added if the attached peripherals are safe to use, no matter what gets
    // written there.
    let (spi_num, cs_num) = match riot_wrappers::BOARD {
        "particle-xenon" => (0, (0, 31)),
        _ => panic!("No "),
    };

    let cs = riot_wrappers::gpio::GPIO::from_port_and_pin(cs_num.0, cs_num.1).unwrap();
    let spi =
        riot_wrappers::spi::for_embedded_hal_1::SPIBus::from_number(spi_num)
            // arbitrary parameters
            .with_speed_1mhz()
            .with_mode(embedded_hal::spi::MODE_2);

    println!("Testing with hardware CS");
    // It is not guaranteed that this is really hardware CS; could just as well be performed by
    // RIOT internally.
    let mut spi_with_hard_cs = spi
            .with_cs(cs)
            .unwrap();
    test_on_device(&mut spi_with_hard_cs);

    println!("Tests done.");
}

fn test_on_device<D: SpiDevice<Error = core::convert::Infallible>>(spi: &mut D) {
    println!("Plain transfer in place:");
    let mut buf = [0, 0, 0x12, 0x34];
    println!("Writing {:?}, …", buf);
    let Ok(()) = spi.transfer_in_place(&mut buf);
    println!("read {:?}.", buf);

    println!("Write from flash:");
    // Writing from flash makes a difference eg. on nrf52: That peripheral is DMA only and can not
    // read flash.
    let buf = [0, 0, 0x12, 0x34];
    println!("Writing {:?}.", buf);
    let Ok(()) = spi.write(&buf);

    println!("Performing complex sequence:");
    let writebuf = [0; 300];
    let mut readbuf1 = [0x12, 0x34, 0x56, 0x78];
    let mut readbuf2 = [0x12, 0x34, 0x56, 0x78];
    use embedded_hal::spi::Operation;
    let mut operations = [
        Operation::Write(&writebuf),
        Operation::DelayNs(123),
        Operation::Read(&mut readbuf1),
        Operation::Read(&mut readbuf2),
    ];
    let Ok(()) = spi.transaction(&mut operations);
    println!(
        "Wrote [0; 300], read into {:?} and {:?}",
        readbuf1, readbuf2
    );

    println!("Plain transfer in place:");
    let writebuf = [0, 0];
    let mut readbuf = [0xff; 10];
    let Ok(()) = spi.transfer(&mut readbuf, &writebuf);
    println!(
        "In mixed transfer, wrote [0; 2], and continued reading into {:?}.",
        readbuf
    );
}
