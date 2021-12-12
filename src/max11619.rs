use max116xx_10bit::{
    Error, ExternallyClocked, InternallyClockedInternallyTimedSerialInterface, Max11619,
    Max116xx10Bit, RefMode, WithoutWakeupDelay,
};
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::spi::FullDuplex;
use va108xx_hal::gpio::{Floating, Input, Pin, PA14};

pub type Max11619ExternallyClocked<SPI> =
    Max116xx10Bit<SPI, Max11619, ExternallyClocked, WithoutWakeupDelay>;
pub type Max11619InternallyClocked<SPI> = Max116xx10Bit<
    SPI,
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
) -> Result<Max11619ExternallyClocked<SPI>, Error<SpiE>>
where
    SPI: Transfer<u8, Error = SpiE> + FullDuplex<u8, Error = SpiE>,
{
    let adc: Max116xx10Bit<SPI, Max11619, ExternallyClocked, WithoutWakeupDelay> =
        Max116xx10Bit::new(spi, RefMode::ExternalSingleEndedNoWakeupDelay)?;
    Ok(adc)
}

pub fn max11619_internally_clocked<SpiE, SPI>(
    spi: SPI,
) -> Result<Max11619InternallyClocked<SPI>, Error<SpiE>>
where
    SPI: Transfer<u8, Error = SpiE> + FullDuplex<u8, Error = SpiE>,
{
    let adc: Max116xx10Bit<
        SPI,
        Max11619,
        InternallyClockedInternallyTimedSerialInterface,
        WithoutWakeupDelay,
    > = Max116xx10Bit::new(spi, RefMode::ExternalSingleEndedNoWakeupDelay)?;
    Ok(adc)
}
