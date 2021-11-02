#![no_main]
#![no_std]

use cortex_m_rt::entry;
use va108xx;

#[entry]
fn main() -> ! {
    let mut dp = va108xx::Peripherals::take();
}
