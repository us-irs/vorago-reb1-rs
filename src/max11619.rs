//! This module provides a thin REB1 specific layer on top of the `max116xx_10bit` driver crate
//!
//! ## Examples
//!
//! - [ADC example](https://egit.irs.uni-stuttgart.de/rust/vorago-reb1/src/branch/main/examples/max11619-adc.rs)
use core::convert::Infallible;

use dummy_pin::DummyPin;
use embedded_hal::{blocking::spi::Transfer, spi::FullDuplex};
use max116xx_10bit::{
    Error, ExternallyClocked, InternallyClockedInternallyTimedSerialInterface, Max11619,
    Max116xx10Bit, RefMode, WithoutWakeupDelay,
};
use va108xx_hal::gpio::{Floating, Input, Pin, PA14};

pub type Max11619ExternallyClocked<SPI> =
    Max116xx10Bit<SPI, DummyPin, Max11619, ExternallyClocked, WithoutWakeupDelay>;
pub type Max11619InternallyClocked<SPI> = Max116xx10Bit<
    SPI,
    DummyPin,
    Max11619,
    InternallyClockedInternallyTimedSerialInterface,
    WithoutWakeupDelay,
>;
pub type EocPin = Pin<PA14, Input<Floating>>;

pub const AN0_CHANNEL: u8 = 0;
pub const AN1_CHANNEL: u8 = 1;
pub const AN2_CHANNEL: u8 = 2;
pub const POTENTIOMETER_CHANNEL: u8 = 3;

pub fn max11619_externally_clocked<SpiE, SPI>(
    spi: SPI,
) -> Result<Max11619ExternallyClocked<SPI>, Error<SpiE, Infallible>>
where
    SPI: Transfer<u8, Error = SpiE> + FullDuplex<u8, Error = SpiE>,
{
    let adc: Max116xx10Bit<SPI, DummyPin, Max11619, ExternallyClocked, WithoutWakeupDelay> =
        Max116xx10Bit::new(
            spi,
            DummyPin::new_low(),
            RefMode::ExternalSingleEndedNoWakeupDelay,
        )?;
    Ok(adc)
}

pub fn max11619_internally_clocked<SpiE, SPI>(
    spi: SPI,
) -> Result<Max11619InternallyClocked<SPI>, Error<SpiE, Infallible>>
where
    SPI: Transfer<u8, Error = SpiE> + FullDuplex<u8, Error = SpiE>,
{
    let adc: Max116xx10Bit<
        SPI,
        DummyPin,
        Max11619,
        InternallyClockedInternallyTimedSerialInterface,
        WithoutWakeupDelay,
    > = Max116xx10Bit::new(
        spi,
        DummyPin::new_low(),
        RefMode::ExternalSingleEndedNoWakeupDelay,
    )?;
    Ok(adc)
}
