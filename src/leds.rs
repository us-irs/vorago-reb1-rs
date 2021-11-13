//! # API for using the REB1 LEDs
//!
//! ## Examples
//!
//! - [LED example](https://github.com/robamu-org/vorago-reb1-rs/blob/main/examples/blinky-leds.rs)
use va108xx_hal::{
    gpio::dynpins::DynPin,
    gpio::pins::{Pin, PushPullOutput, PA10, PA6, PA7},
    prelude::*,
};

pub type LD2 = Pin<PA10, PushPullOutput>;
pub type LD3 = Pin<PA7, PushPullOutput>;
pub type LD4 = Pin<PA6, PushPullOutput>;

pub struct Leds {
    leds: [Led; 3],
}

impl Leds {
    pub fn new(led_pin1: LD2, led_pin2: LD3, led_pin3: LD4) -> Leds {
        Leds {
            leds: [led_pin1.into(), led_pin2.into(), led_pin3.into()],
        }
    }
}

impl core::ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl core::ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl core::ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl core::ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

pub struct Led {
    pin: DynPin,
}

macro_rules! ctor {
	($($ldx:ident),+) => {
		$(
			impl From<$ldx> for Led {
				fn from(led: $ldx) -> Self {
					Led {
						pin: led.into()
					}
				}
			}
		)+
	}
}

ctor!(LD2, LD3, LD4);

impl Led {
    /// Turns the LED off. Setting the pin high actually turns the LED off
    pub fn off(&mut self) {
        self.pin.set_high().ok();
    }

    /// Turns the LED on. Setting the pin low actually turns the LED on
    pub fn on(&mut self) {
        self.pin.set_low().ok();
    }

    /// Toggles the LED
    pub fn toggle(&mut self) {
        self.pin.toggle().ok();
    }
}
