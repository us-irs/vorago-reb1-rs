//! Blinky button application for the REB1 board
#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::{
    clock::{set_clk_div_register, FilterClkSel},
    gpio::{FilterType, InterruptEdge, PinsA},
    pac::{self, interrupt},
    prelude::*,
    time::Hertz,
    timer::{default_ms_irq_handler, set_up_ms_timer, IrqCfg},
};
use vorago_reb1::button::Button;
use vorago_reb1::leds::Leds;

static LEDS: Mutex<RefCell<Option<Leds>>> = Mutex::new(RefCell::new(None));
static BUTTON: Mutex<RefCell<Option<Button>>> = Mutex::new(RefCell::new(None));

#[derive(Debug, PartialEq)]
pub enum PressMode {
    Toggle,
    Keep,
}

// You can change the press mode here
const PRESS_MODE: PressMode = PressMode::Keep;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago Button IRQ Example --");
    let mut dp = pac::Peripherals::take().unwrap();
    let pinsa = PinsA::new(&mut dp.SYSCONFIG, Some(dp.IOCONFIG), dp.PORTA);
    let edge_irq = match PRESS_MODE {
        PressMode::Toggle => InterruptEdge::HighToLow,
        PressMode::Keep => InterruptEdge::BothEdges,
    };

    // Configure an edge interrupt on the button and route it to interrupt vector 15
    let mut button = Button::new(pinsa.pa11.into_floating_input()).edge_irq(
        edge_irq,
        IrqCfg::new(pac::interrupt::OC15, true, true),
        Some(&mut dp.SYSCONFIG),
        Some(&mut dp.IRQSEL),
    );

    if PRESS_MODE == PressMode::Toggle {
        // This filter debounces the switch for edge based interrupts
        button = button.filter_type(FilterType::FilterFourClockCycles, FilterClkSel::Clk1);
        set_clk_div_register(
            &mut dp.SYSCONFIG,
            FilterClkSel::Clk1,
            Hertz::from(50.khz()).0,
        );
    }

    set_up_ms_timer(
        IrqCfg::new(pac::Interrupt::OC0, true, true),
        &mut dp.SYSCONFIG,
        Some(&mut dp.IRQSEL),
        50.mhz(),
        dp.TIM0,
    );
    let mut leds = Leds::new(
        pinsa.pa10.into_push_pull_output(),
        pinsa.pa7.into_push_pull_output(),
        pinsa.pa6.into_push_pull_output(),
    );
    for led in leds.iter_mut() {
        led.off();
    }
    // Make both button and LEDs accessible from the IRQ handler as well
    cortex_m::interrupt::free(|cs| {
        LEDS.borrow(cs).replace(Some(leds));
        BUTTON.borrow(cs).replace(Some(button));
    });
    loop {}
}

#[interrupt]
fn OC0() {
    default_ms_irq_handler();
}

#[interrupt]
fn OC15() {
    cortex_m::interrupt::free(|cs| {
        if PRESS_MODE == PressMode::Toggle {
            if let Some(ref mut leds) = LEDS.borrow(cs).borrow_mut().as_deref_mut() {
                leds[0].toggle();
            }
        } else {
            if let (Some(ref mut leds), Some(ref button)) = (
                LEDS.borrow(cs).borrow_mut().as_deref_mut(),
                BUTTON.borrow(cs).borrow().as_ref(),
            ) {
                if button.released() {
                    leds[0].off();
                } else {
                    leds[0].on();
                }
            }
        }
    });
}
