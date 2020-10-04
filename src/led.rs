use hal::prelude::*;

use hal::gpio;
use hal::gpio::gpiob::PB12;
use hal::gpio::gpioh::PH0;

pub type LEDLCD = PB12<gpio::Output<gpio::PushPull>>;
pub type LEDUSR = PH0<gpio::Output<gpio::PushPull>>;

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
into_led!(LEDLCD, LEDUSR);

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
