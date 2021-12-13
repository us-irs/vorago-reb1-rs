//! This module provides a thin REB1 specific layer on top of the `max116xx_10bit` driver crate
//!
//! ## Examples
//!
//! - [ADC example](https://egit.irs.uni-stuttgart.de/rust/vorago-reb1/src/branch/main/examples/max11619-adc.rs)
use core::convert::Infallible;
use dummy_pin::DummyPin;
use embedded_hal::{blocking::spi::Transfer, spi::FullDuplex};
use max116xx_10bit::{
    Error, ExternallyClocked, InternallyClockedInternallyTimedSerialInterface, Max116xx10Bit,
    Max116xx10BitEocExt, VoltageRefMode, WithWakeupDelay, WithoutWakeupDelay,
};
use va108xx_hal::gpio::{Floating, Input, Pin, PA14};

pub type Max11619ExternallyClockedNoWakeup<SPI> =
    Max116xx10Bit<SPI, DummyPin, ExternallyClocked, WithoutWakeupDelay>;
pub type Max11619ExternallyClockedWithWakeup<SPI> =
    Max116xx10Bit<SPI, DummyPin, ExternallyClocked, WithWakeupDelay>;
pub type Max11619InternallyClocked<SPI, EOC> =
    Max116xx10BitEocExt<SPI, DummyPin, EOC, InternallyClockedInternallyTimedSerialInterface>;
pub type EocPin = Pin<PA14, Input<Floating>>;

pub const AN0_CHANNEL: u8 = 0;
pub const AN1_CHANNEL: u8 = 1;
pub const AN2_CHANNEL: u8 = 2;
pub const POTENTIOMETER_CHANNEL: u8 = 3;

pub fn max11619_externally_clocked_no_wakeup<SpiE, SPI>(
    spi: SPI,
) -> Result<Max11619ExternallyClockedNoWakeup<SPI>, Error<SpiE, Infallible>>
where
    SPI: Transfer<u8, Error = SpiE> + FullDuplex<u8, Error = SpiE>,
{
    let mut adc = Max116xx10Bit::max11619(spi, DummyPin::new_low())?;
    adc.reset(false)?;
    adc.setup()?;
    Ok(adc)
}

pub fn max11619_externally_clocked_with_wakeup<SpiE, SPI>(
    spi: SPI,
) -> Result<Max11619ExternallyClockedWithWakeup<SPI>, Error<SpiE, Infallible>>
where
    SPI: Transfer<u8, Error = SpiE> + FullDuplex<u8, Error = SpiE>,
{
    let mut adc = Max116xx10Bit::max11619(spi, DummyPin::new_low())?
        .into_ext_clkd_with_int_ref_wakeup_delay();
    adc.reset(false)?;
    adc.setup()?;
    Ok(adc)
}

pub fn max11619_internally_clocked<SpiE, SPI>(
    spi: SPI,
    eoc: EocPin,
    v_ref: VoltageRefMode,
) -> Result<Max11619InternallyClocked<SPI, EocPin>, Error<SpiE, Infallible>>
where
    SPI: Transfer<u8, Error = SpiE> + FullDuplex<u8, Error = SpiE>,
{
    let mut adc = Max116xx10Bit::max11619(spi, DummyPin::new_low())?
        .into_int_clkd_int_timed_through_ser_if_without_wakeup(v_ref, eoc)?;
    adc.reset(false)?;
    adc.setup()?;
    Ok(adc)
}
