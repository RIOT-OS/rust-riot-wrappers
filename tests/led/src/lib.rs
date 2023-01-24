#![no_std]

use riot_wrappers::led;
use riot_wrappers::riot_main;

riot_main!(main);

fn main() {
    let mut led0 = riot_wrappers::led::LED::<0>::new();
    let mut led1 = riot_wrappers::led::LED::<1>::new();
    let mut led2 = riot_wrappers::led::LED::<2>::new();
    let mut led3 = riot_wrappers::led::LED::<3>::new();
    let mut led4 = riot_wrappers::led::LED::<4>::new();
    let mut led5 = riot_wrappers::led::LED::<5>::new();
    let mut led6 = riot_wrappers::led::LED::<6>::new();
    let mut led7 = riot_wrappers::led::LED::<7>::new();
    let mut leds: [&mut dyn switch_hal::ToggleableOutputSwitch<Error = _>; 8] = [
        &mut led0, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6, &mut led7,
    ];
    use switch_hal::ToggleableOutputSwitch;
    loop {
        for i in 0..=255 {
            for j in 0..8 {
                if (i ^ (i - 1)) & (1 << j) != 0 {
                    leds[j].toggle();
                }
            }
        }

        // The LSB blinking is probably way too fast; after a cycle, shift so we're seeing other
        // significances in the implemented LEDs. It's still *way* too fast for the naked eye.
        leds.rotate_left(1);
    }
}
