use va108xx_hal::{
    gpio::dynpins::DynPin,
    gpio::pins::{Pin, PinsA, PushPullOutput, PA10, PA6, PA7},
    prelude::*,
};

pub type LD2 = Pin<PA10, PushPullOutput>;
pub type LD3 = Pin<PA7, PushPullOutput>;
pub type LD4 = Pin<PA6, PushPullOutput>;

pub struct Leds {
    leds: [Led; 3],
}

impl Leds {
    pub fn new(led_parts: PinsA) -> Self {
        let led2 = led_parts.pa10.into_push_pull_output();
        let led3 = led_parts.pa7.into_push_pull_output();
        let led4 = led_parts.pa6.into_push_pull_output();
        Leds {
            leds: [led2.into(), led3.into(), led4.into()],
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
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pin.set_low().ok();
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pin.set_high().ok();
    }

    /// Toggles the LED
    pub fn toggle(&mut self) {
        self.pin.toggle().ok();
    }
}
