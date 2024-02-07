//! Blinks an LED

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_probe as _;
use defmt_brtt as _;

use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    defmt::info!("Blinking!");

    loop {
        defmt::info!("LED on");
        for _ in 0..100_000 {
            led.set_high();
        }
        defmt::info!("LED off");
        for _ in 0..100_000 {
            led.set_low();
        }
    }
}