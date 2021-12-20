//! MAX11619 ADC example applikcation
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{blocking::delay::DelayUs, spi};
use max116xx_10bit::VoltageRefMode;
use max116xx_10bit::{AveragingConversions, AveragingResults};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::timer::CountDownTimer;
use va108xx_hal::{
    gpio::PinsA,
    pac::{self, interrupt, SPIB},
    prelude::*,
    spi::{Spi, SpiBase, SpiConfig, TransferConfig},
    timer::{default_ms_irq_handler, set_up_ms_timer, Delay, IrqCfg},
    utility::{port_mux, Funsel, PortSel},
};
use vorago_reb1::max11619::{
    max11619_externally_clocked_no_wakeup, max11619_externally_clocked_with_wakeup,
    max11619_internally_clocked, EocPin, AN2_CHANNEL, POTENTIOMETER_CHANNEL,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExampleMode {
    UsingEoc,
    NotUsingEoc,
    NotUsingEocWithDelay,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ReadMode {
    Single,
    Multiple,
    MultipleNToHighest,
    AverageN,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MuxMode {
    None,
    PortB19to17,
}

const EXAMPLE_MODE: ExampleMode = ExampleMode::NotUsingEoc;
const READ_MODE: ReadMode = ReadMode::Multiple;
const MUX_MODE: MuxMode = MuxMode::None;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago ADC Example --");

    let mut dp = pac::Peripherals::take().unwrap();
    let tim0 = set_up_ms_timer(
        IrqCfg::new(pac::Interrupt::OC0, true, true),
        &mut dp.SYSCONFIG,
        Some(&mut dp.IRQSEL),
        50.mhz(),
        dp.TIM0,
    );
    let delay = Delay::new(tim0);
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC0);
    }

    let pinsa = PinsA::new(&mut dp.SYSCONFIG, None, dp.PORTA);
    let spi_cfg = SpiConfig::default();
    let (sck, mosi, miso) = (
        pinsa.pa20.into_funsel_2(),
        pinsa.pa19.into_funsel_2(),
        pinsa.pa18.into_funsel_2(),
    );

    if MUX_MODE == MuxMode::PortB19to17 {
        port_mux(&mut dp.IOCONFIG, PortSel::PortB, 19, Funsel::Funsel1).ok();
        port_mux(&mut dp.IOCONFIG, PortSel::PortB, 18, Funsel::Funsel1).ok();
        port_mux(&mut dp.IOCONFIG, PortSel::PortB, 17, Funsel::Funsel1).ok();
        port_mux(&mut dp.IOCONFIG, PortSel::PortB, 16, Funsel::Funsel1).ok();
    }
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
        ExampleMode::NotUsingEocWithDelay => {
            let delay_us = CountDownTimer::new(&mut dp.SYSCONFIG, 50.mhz(), dp.TIM2);
            spi_example_externally_clocked_with_delay(spi, delay, delay_us);
        }
    }
}

#[interrupt]
fn OC0() {
    default_ms_irq_handler();
}

/// Use the SPI clock as the conversion clock
fn spi_example_externally_clocked(spi: SpiBase<SPIB>, mut delay: Delay) -> ! {
    let mut adc = max11619_externally_clocked_no_wakeup(spi)
        .expect("Creating externally clocked MAX11619 device failed");
    if READ_MODE == ReadMode::AverageN {
        adc.averaging(
            AveragingConversions::FourConversions,
            AveragingResults::FourResults,
        )
        .expect("Error setting up averaging register");
    }
    let mut cmd_buf: [u8; 32] = [0; 32];
    let mut counter = 0;
    loop {
        rprintln!("-- Measurement {} --", counter);

        match READ_MODE {
            ReadMode::Single => {
                rprintln!("Reading single potentiometer channel");
                let pot_val = adc
                    .read_single_channel(&mut cmd_buf, POTENTIOMETER_CHANNEL)
                    .expect("Creating externally clocked MAX11619 ADC failed");
                rprintln!("Single channel read:");
                rprintln!("\tPotentiometer value: {}", pot_val);
            }
            ReadMode::Multiple => {
                let mut res_buf: [u16; 4] = [0; 4];
                adc.read_multiple_channels_0_to_n(
                    &mut cmd_buf,
                    &mut res_buf.iter_mut(),
                    POTENTIOMETER_CHANNEL,
                )
                .expect("Multi-Channel read failed");
                print_res_buf(&res_buf);
            }
            ReadMode::MultipleNToHighest => {
                let mut res_buf: [u16; 2] = [0; 2];
                adc.read_multiple_channels_n_to_highest(
                    &mut cmd_buf,
                    &mut res_buf.iter_mut(),
                    AN2_CHANNEL,
                )
                .expect("Multi-Channel read failed");
                rprintln!("Multi channel read from 2 to 3:");
                rprintln!("\tAN2 value: {}", res_buf[0]);
                rprintln!("\tAN3 / Potentiometer value: {}", res_buf[1]);
            }
            ReadMode::AverageN => {
                rprintln!("Scanning and averaging not possible for externally clocked mode");
            }
        }
        counter += 1;
        delay.delay_ms(500);
    }
}

fn spi_example_externally_clocked_with_delay(
    spi: SpiBase<SPIB>,
    mut delay: Delay,
    mut delay_us: impl DelayUs<u8>,
) -> ! {
    let mut adc =
        max11619_externally_clocked_with_wakeup(spi).expect("Creating MAX116xx device failed");
    let mut cmd_buf: [u8; 32] = [0; 32];
    let mut counter = 0;
    loop {
        rprintln!("-- Measurement {} --", counter);

        match READ_MODE {
            ReadMode::Single => {
                rprintln!("Reading single potentiometer channel");
                let pot_val = adc
                    .read_single_channel(&mut cmd_buf, POTENTIOMETER_CHANNEL, &mut delay_us)
                    .expect("Creating externally clocked MAX11619 ADC failed");
                rprintln!("Single channel read:");
                rprintln!("\tPotentiometer value: {}", pot_val);
            }
            ReadMode::Multiple => {
                let mut res_buf: [u16; 4] = [0; 4];
                adc.read_multiple_channels_0_to_n(
                    &mut cmd_buf,
                    &mut res_buf.iter_mut(),
                    POTENTIOMETER_CHANNEL,
                    &mut delay_us,
                )
                .expect("Multi-Channel read failed");
                print_res_buf(&res_buf);
            }
            ReadMode::MultipleNToHighest => {
                let mut res_buf: [u16; 2] = [0; 2];
                adc.read_multiple_channels_n_to_highest(
                    &mut cmd_buf,
                    &mut res_buf.iter_mut(),
                    AN2_CHANNEL,
                    &mut delay_us,
                )
                .expect("Multi-Channel read failed");
                rprintln!("Multi channel read from 2 to 3:");
                rprintln!("\tAN2 value: {}", res_buf[0]);
                rprintln!("\tAN3 / Potentiometer value: {}", res_buf[1]);
            }
            ReadMode::AverageN => {
                rprintln!("Scanning and averaging not possible for externally clocked mode");
            }
        }
        counter += 1;
        delay.delay_ms(500);
    }
}

/// This function uses the EOC pin to determine whether the conversion finished
fn spi_example_internally_clocked(spi: SpiBase<SPIB>, mut delay: Delay, eoc_pin: EocPin) -> ! {
    let mut adc = max11619_internally_clocked(
        spi,
        eoc_pin,
        VoltageRefMode::ExternalSingleEndedNoWakeupDelay,
    )
    .expect("Creating MAX116xx device failed");
    let mut counter = 0;
    loop {
        rprintln!("-- Measurement {} --", counter);

        match READ_MODE {
            ReadMode::Single => {
                adc.request_single_channel(POTENTIOMETER_CHANNEL)
                    .expect("Requesting single channel value  failed");

                let pot_val = nb::block!(adc.get_single_channel())
                    .expect("Reading single channel value  failed");
                rprintln!("\tPotentiometer value: {}", pot_val);
            }
            ReadMode::Multiple => {
                adc.request_multiple_channels_0_to_n(POTENTIOMETER_CHANNEL)
                    .expect("Requesting single channel value  failed");
                let mut res_buf: [u16; 4] = [0; 4];
                nb::block!(adc.get_multi_channel(&mut res_buf.iter_mut()))
                    .expect("Requesting multiple channel values failed");
                print_res_buf(&res_buf);
            }
            ReadMode::MultipleNToHighest => {
                adc.request_multiple_channels_n_to_highest(AN2_CHANNEL)
                    .expect("Requesting single channel value  failed");
                let mut res_buf: [u16; 4] = [0; 4];
                nb::block!(adc.get_multi_channel(&mut res_buf.iter_mut()))
                    .expect("Requesting multiple channel values failed");
                rprintln!("Multi channel read from 2 to 3:");
                rprintln!("\tAN2 value: {}", res_buf[0]);
                rprintln!("\tAN3 / Potentiometer value: {}", res_buf[1]);
            }
            ReadMode::AverageN => {
                adc.request_channel_n_repeatedly(POTENTIOMETER_CHANNEL)
                    .expect("Reading channel multiple times failed");
                let mut res_buf: [u16; 16] = [0; 16];
                nb::block!(adc.get_multi_channel(&mut res_buf.iter_mut()))
                    .expect("Requesting multiple channel values failed");
                rprintln!("Reading potentiometer 4 times");
                rprintln!("\tValue 0: {}", res_buf[0]);
                rprintln!("\tValue 1: {}", res_buf[1]);
                rprintln!("\tValue 2: {}", res_buf[2]);
                rprintln!("\tValue 3: {}", res_buf[3]);
            }
        }

        counter += 1;
        delay.delay_ms(500);
    }
}

fn print_res_buf(buf: &[u16; 4]) {
    rprintln!("Multi channel read from 0 to 3:");
    rprintln!("\tAN0 value: {}", buf[0]);
    rprintln!("\tAN1 value: {}", buf[1]);
    rprintln!("\tAN2 value: {}", buf[2]);
    rprintln!("\tAN3 / Potentiometer value: {}", buf[3]);
}
