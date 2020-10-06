#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;

pub extern crate stm32l0xx_hal as hal;

mod button;
mod led;
mod pal;

#[entry]
fn main() -> ! {
    // No idea what this part really does aside from set up a
    // timer we use for slowing things down. 
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000);
    syst.enable_counter();

    let mut p = pal::Pal::new();

    loop {
        while !syst.has_wrapped() {}

        if p.btn1_left.is_pressed() {
            p.led_usr.on();
        } else {
            p.led_usr.off();
        }
    }
}
