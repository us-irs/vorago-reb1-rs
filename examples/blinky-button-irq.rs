//! Minimal blinky for the REB1 board using only PAC features
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::pac::interrupt;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago Button Blinky --");
    loop {}
}

#[interrupt]
fn OC15() {}
