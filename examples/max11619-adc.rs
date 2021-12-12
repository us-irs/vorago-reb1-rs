//! MAX11619 ADC example applikcation
#![no_main]
#![no_std]

use core::panic;

use cortex_m_rt::entry;
use embedded_hal::spi;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::{
    gpio::PinsA,
    pac::{self, interrupt, SPIB},
    prelude::*,
    spi::{Spi, SpiBase, SpiConfig, TransferConfig},
    timer::{default_ms_irq_handler, set_up_ms_timer, Delay},
    utility::*,
};
use vorago_reb1::max11619::{
    max11619_externally_clocked, max11619_internally_clocked, EocPin, AN2_CHANNEL,
    POTENTIOMETER_CHANNEL,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExampleMode {
    UsingEoc,
    NotUsingEoc,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ReadMode {
    Single,
    Multiple,
    MultipleNToHighest,
}

const EXAMPLE_MODE: ExampleMode = ExampleMode::NotUsingEoc;
const READ_MODE: ReadMode = ReadMode::Multiple;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago ADC Example --");

    let mut dp = pac::Peripherals::take().unwrap();
    let tim0 = set_up_ms_timer(
        &mut dp.SYSCONFIG,
        &mut dp.IRQSEL,
        50.mhz().into(),
        dp.TIM0,
        interrupt::OC0,
    );
    let delay = Delay::new(tim0);
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC0);
    }

    let pinsa = PinsA::new(&mut dp.SYSCONFIG, None, dp.PORTA);
    let mut spi_cfg = SpiConfig::default();
    spi_cfg.scrdv = 0x07;
    let (sck, mosi, miso) = (
        pinsa.pa20.into_funsel_2(),
        pinsa.pa19.into_funsel_2(),
        pinsa.pa18.into_funsel_2(),
    );

    port_mux(&mut dp.IOCONFIG, PortSel::PortB, 16, Funsel::Funsel1).ok();
    // port_mux(&mut dp.IOCONFIG, PortSel::PortB, 17, Funsel::Funsel1).ok();
    port_mux(&mut dp.IOCONFIG, PortSel::PortB, 18, Funsel::Funsel1).ok();
    port_mux(&mut dp.IOCONFIG, PortSel::PortB, 19, Funsel::Funsel1).ok();
    // Set the accelerometer chip select low in case the board slot is populated
    let mut accel_cs = pinsa.pa16.into_push_pull_output();
    accel_cs
        .set_high()
        .expect("Setting accelerometer chip select high failed");

    let transfer_cfg = TransferConfig::new(
        3.mhz(),
        spi::MODE_0,
        Some(pinsa.pa17.into_funsel_2()),
        true,
        false,
    );
    let spi = Spi::spib(
        dp.SPIB,
        (sck, miso, mosi),
        50.mhz(),
        spi_cfg,
        Some(&mut dp.SYSCONFIG),
        Some(&transfer_cfg.downgrade()),
    )
    .downgrade();
    match EXAMPLE_MODE {
        ExampleMode::NotUsingEoc => spi_example_externally_clocked(spi, delay),
        ExampleMode::UsingEoc => {
            spi_example_internally_clocked(spi, delay, pinsa.pa14.into_floating_input());
        }
    }
}

#[interrupt]
fn OC0() {
    default_ms_irq_handler();
}

fn spi_example_externally_clocked(spi: SpiBase<SPIB>, mut delay: Delay) -> ! {
    let mut adc = max11619_externally_clocked(spi)
        .expect("Creating externally clocked MAX11619 device failed");
    let mut cmd_buf: [u8; 32] = [0; 32];
    let mut counter = 0;
    loop {
        rprintln!("-- Measurement {} --", counter);

        match READ_MODE {
            ReadMode::Single => {
                rprintln!("Reading single potentiometer channel");
                let pot_val = match adc.read_single_channel(&mut cmd_buf, POTENTIOMETER_CHANNEL) {
                    Ok(pot_val) => pot_val,
                    _ => {
                        panic!("Creating externally clocked MAX11619 ADC failed");
                    }
                };
                rprintln!("Single channel read:");
                rprintln!("\tPotentiometer value: {}", pot_val);
            }
            ReadMode::Multiple => {
                let mut res_buf: [u16; 4] = [0; 4];
                match adc.read_multiple_channels_0_to_n(
                    &mut cmd_buf,
                    &mut res_buf.iter_mut(),
                    POTENTIOMETER_CHANNEL,
                ) {
                    Ok(_) => {
                        rprintln!("Multi channel read from 0 to 3:");
                        rprintln!("\tAN0 value: {}", res_buf[0]);
                        rprintln!("\tAN1 value: {}", res_buf[1]);
                        rprintln!("\tAN2 value: {}", res_buf[2]);
                        rprintln!("\tAN3 / Potentiometer value: {}", res_buf[3]);
                    }
                    _ => {
                        panic!("Multi-Channel read failed");
                    }
                }
            }
            ReadMode::MultipleNToHighest => {
                let mut res_buf: [u16; 2] = [0; 2];
                match adc.read_multiple_channels_n_to_highest(
                    &mut cmd_buf,
                    &mut res_buf.iter_mut(),
                    AN2_CHANNEL,
                ) {
                    Ok(_) => {
                        rprintln!("Multi channel read from 2 to 3:");
                        rprintln!("\tAN2 value: {}", res_buf[0]);
                        rprintln!("\tAN3 / Potentiometer value: {}", res_buf[1]);
                    }
                    _ => {
                        panic!("Multi-Channel read failed");
                    }
                }
            }
        }
        counter += 1;
        delay.delay_ms(500);
    }
}

fn spi_example_internally_clocked(spi: SpiBase<SPIB>, mut delay: Delay, mut eoc_pin: EocPin) -> ! {
    let mut adc = max11619_internally_clocked(spi).expect("Creaintg MAX116xx device failed");
    let mut counter = 0;
    loop {
        rprintln!("-- Measurement {} --", counter);
        match adc.request_single_channel(POTENTIOMETER_CHANNEL) {
            Ok(_) => (),
            _ => panic!("Requesting single channel value  failed"),
        };

        let pot_val = match nb::block!(adc.get_single_channel(&mut eoc_pin)) {
            Ok(pot_val) => pot_val,
            _ => panic!("Reading single channel value  failed"),
        };
        rprintln!("\tPotentiometer value: {}", pot_val);
        counter += 1;
        delay.delay_ms(500);
    }
}
