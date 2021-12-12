#![no_main]
#![no_std]
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::{
    pac::{self, interrupt},
    prelude::*,
    timer::{default_ms_irq_handler, set_up_ms_timer, Delay},
};
use vorago_reb1::temp_sensor::Adt75TempSensor;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago Temperature Sensor and I2C Example --");
    let mut dp = pac::Peripherals::take().unwrap();
    let tim0 = set_up_ms_timer(
        &mut dp.SYSCONFIG,
        &mut dp.IRQSEL,
        50.mhz().into(),
        dp.TIM0,
        interrupt::OC0,
    );
    let mut delay = Delay::new(tim0);
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC0);
    }

    let mut temp_sensor = Adt75TempSensor::new(dp.I2CA, 50.mhz(), Some(&mut dp.SYSCONFIG))
        .expect("Creating temperature sensor struct failed");
    loop {
        let temp = temp_sensor
            .read_temperature()
            .expect("Failed reading temperature");
        rprintln!("Temperature in Celcius: {}", temp);
        delay.delay_ms(500);
    }
}

#[interrupt]
fn OC0() {
    default_ms_irq_handler();
}
