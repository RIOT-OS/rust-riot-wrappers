#![no_std]

use embedded_hal::Pwm;
use riot_wrappers::pwm::{HertzU32, PWMDevice, PWMMode};
use riot_wrappers::riot_main;

riot_main!(main);

fn main() {
    let mut pwm = PWMDevice::<0>::new(0, PWMMode::Left, HertzU32::Hz(10), 100).unwrap();
    let channel_0 = pwm.get_channel(0).unwrap();
    pwm.set_duty(channel_0, 50); // 50% duty_cycle
}
