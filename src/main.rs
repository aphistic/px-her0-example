#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;

pub extern crate stm32l0xx_hal as hal;

use rtt_target::{rtt_init_print, rprintln};

mod button;
mod led;
mod pal;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // No idea what this part really does aside from set up a
    // timer we use for slowing things down. 
    let clock_cycles = 8_000;
    rprintln!("Setting SYST clock to {}", clock_cycles);
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(clock_cycles);
    syst.enable_counter();

    let mut p = pal::Pal::new();

    rprintln!("Starting loop");
    loop {
        while !syst.has_wrapped() {}

        if p.btn1_left.is_pressed() {
            if !p.led_usr.is_on() {
                rprintln!("Turning on user led");
                p.led_usr.on();
            }
        } else {
            if p.led_usr.is_on() {
                rprintln!("Turning off user led");
                p.led_usr.off();
            }
        }
    }
}
