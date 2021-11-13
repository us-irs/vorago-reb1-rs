//! Blinky examples using the PAC directly, the HAL, or the BSP
//!
//! Additional note on LEDs:
//! Be not afraid: Pulling the GPIOs low makes the LEDs blink. See REB1
//! schematic for more details.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::ToggleableOutputPin;
use panic_halt as _;
use va108xx_hal::{gpio::pins::PinsA, pac, prelude::*};
use vorago_reb1::leds::Leds;

// REB LED pin definitions. All on port A
const LED_D2: u32 = 1 << 10;
const LED_D3: u32 = 1 << 7;
const LED_D4: u32 = 1 << 6;

#[allow(dead_code)]
enum LibType {
    Pac,
    Hal,
    Bsp,
}

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();

    let lib_type = LibType::Bsp;

    match lib_type {
        LibType::Pac => {
            // Enable all peripheral clocks
            dp.SYSCONFIG
                .peripheral_clk_enable
                .modify(|_, w| unsafe { w.bits(0xffffffff) });
            dp.PORTA
                .dir()
                .modify(|_, w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
            dp.PORTA
                .datamask()
                .modify(|_, w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
            for _ in 0..10 {
                dp.PORTA
                    .clrout()
                    .write(|w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
                cortex_m::asm::delay(5_000_000);
                dp.PORTA
                    .setout()
                    .write(|w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
                cortex_m::asm::delay(5_000_000);
            }
            loop {
                dp.PORTA
                    .togout()
                    .write(|w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
                cortex_m::asm::delay(25_000_000);
            }
        }
        LibType::Hal => {
            let pins = PinsA::new(&mut dp.SYSCONFIG, Some(dp.IOCONFIG), dp.PORTA);
            let mut led1 = pins.pa10.into_push_pull_output();
            let mut led2 = pins.pa7.into_push_pull_output();
            let mut led3 = pins.pa6.into_push_pull_output();
            for _ in 0..10 {
                led1.set_low().ok();
                led2.set_low().ok();
                led3.set_low().ok();
                cortex_m::asm::delay(5_000_000);
                led1.set_high().ok();
                led2.set_high().ok();
                led3.set_high().ok();
                cortex_m::asm::delay(5_000_000);
            }
            loop {
                led1.toggle().ok();
                cortex_m::asm::delay(5_000_000);
                led2.toggle().ok();
                cortex_m::asm::delay(5_000_000);
                led3.toggle().ok();
                cortex_m::asm::delay(5_000_000);
            }
        }
        LibType::Bsp => {
            let pinsa = PinsA::new(&mut dp.SYSCONFIG, Some(dp.IOCONFIG), dp.PORTA);
            let mut leds = Leds::new(
                pinsa.pa10.into_push_pull_output(),
                pinsa.pa7.into_push_pull_output(),
                pinsa.pa6.into_push_pull_output(),
            );
            loop {
                for _ in 0..10 {
                    // Blink all LEDs quickly
                    for led in leds.iter_mut() {
                        led.toggle();
                    }
                    cortex_m::asm::delay(5_000_000);
                }
                // Now use a wave pattern
                loop {
                    leds[0].toggle();
                    cortex_m::asm::delay(5_000_000);
                    leds[1].toggle();
                    cortex_m::asm::delay(5_000_000);
                    leds[2].toggle();
                    cortex_m::asm::delay(5_000_000);
                }
            }
        }
    }
}
