#![no_main]
#![no_std]
use cortex_m_rt::entry;
use embedded_hal::blocking::i2c::SevenBitAddress;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::{
    i2c::{I2cMaster, I2cSpeed, MasterConfig},
    pac::{self, interrupt, I2CA},
    prelude::*,
    timer::{default_ms_irq_handler, set_up_ms_timer, Delay},
};

#[allow(dead_code)]
enum RegAddresses {
    Temperature = 0x00,
    Configuration = 0x01,
    THystSetpoint = 0x02,
    TOsSetPoint = 0x03,
    OneShot = 0x04,
}

const ADT75_I2C_ADDR: u8 = 0b1001000;

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

    let ms_cfg = MasterConfig::default();
    let mut i2ca: I2cMaster<I2CA, SevenBitAddress> = I2cMaster::i2ca(
        dp.I2CA,
        ms_cfg,
        50.mhz(),
        I2cSpeed::Regular100khz,
        Some(&mut dp.SYSCONFIG),
    );

    let mut cmd: [u8; 4] = [0; 4];
    cmd[0] = RegAddresses::Temperature as u8;
    let mut reply: [u8; 4] = [0; 4];
    i2ca.write(ADT75_I2C_ADDR, &cmd[0..1])
        .expect("I2C send transfer failed");
    loop {
        i2ca.read(ADT75_I2C_ADDR, &mut reply[0..2])
            .expect("I2C read transfer failed");
        let adc_code = (((reply[0] as u16) << 8) | reply[1] as u16) >> 4;
        let temp_celcius: f32;
        if ((adc_code >> 11) & 0x01) == 0 {
            // Sign bit not set, positiv value
            // Divide ADC code by 16 according to datasheet
            temp_celcius = adc_code as f32 / 16.0;
        } else {
            // Calculation for negative values, assuming all 12 bits are used
            temp_celcius = (adc_code - 4096) as f32 / 16.0;
        }
        rprintln!("Temperature in Celcius: {}", temp_celcius);
        delay.delay_ms(500);
    }
}

#[interrupt]
fn OC0() {
    default_ms_irq_handler();
}
