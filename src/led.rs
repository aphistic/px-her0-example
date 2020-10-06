use hal::prelude::*;

use hal::gpio;
use hal::gpio::gpiob::PB12;
use hal::gpio::gpioh::PH0;

// Define some types for our peripherals so we can refer to them
// with a useful name instead of just GPIO pins.
pub type LEDLCD = PB12<gpio::Output<gpio::PushPull>>;
pub type LEDUSR = PH0<gpio::Output<gpio::PushPull>>;

// Define a macro we can use to create all the Into impls we need
macro_rules! into_led {
    ($($pin:ident),+) => {
        $(
            impl<PIN: StatefulOutputPin + From<$pin>> Into<Led<PIN>> for $pin {
                fn into(self) -> Led<PIN> {
                    Led {
                        p: self.into(),
                    }
                }
            }
        )+
    }
}
// Define the Into impls for LEDLCD and LEDUSR
into_led!(LEDLCD, LEDUSR);

// Define the actual Led struct! This seems deceptively straightforward
// compared to the other code.
pub struct Led<PIN: StatefulOutputPin> {
    p: PIN,
}

impl<PIN: StatefulOutputPin> Led<PIN> {
    pub fn on(&mut self) {
        match self.p.set_high() {
            _ => {}
        }
    }

    pub fn off(&mut self) {
        match self.p.set_low() {
            _ => {}
        }
    }

    pub fn is_on(&self) -> bool {
        match self.p.is_set_high() {
            Ok(v) => v,
            _ => false,
        }
    }

    pub fn toggle(&mut self) {
        match self.is_on() {
            true => self.off(),
            false => self.on(),
        }
    }
}
