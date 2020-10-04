use hal::prelude::*;

use hal::gpio;
use hal::gpio::gpioa::{PA15};
use hal::gpio::gpioc::{PC0, PC9, PC12, PC13};
use hal::gpio::gpioh::{PH1};

pub type BTN1 = PC0<gpio::Input<gpio::PullUp>>;
pub type BTN2 = PH1<gpio::Input<gpio::PullUp>>;
pub type BTN3 = PC13<gpio::Input<gpio::PullUp>>;
pub type BTN4 = PC12<gpio::Input<gpio::PullUp>>;
pub type BTN5 = PA15<gpio::Input<gpio::PullUp>>;
pub type BTN6 = PC9<gpio::Input<gpio::PullUp>>;

pub struct Button<BTN: InputPin> {
    p: BTN,
}

macro_rules! into_btn {
    ($($pin:ident),+) => {
        $(
            impl<BTN: InputPin + From<$pin>> Into<Button<BTN>> for $pin {
                fn into(self) -> Button<BTN> {
                    Button {
                        p: self.into(),
                    }
                }
            }
        )+
    }
}
into_btn!(BTN1, BTN2, BTN3, BTN4, BTN5, BTN6); 

impl<BTN: InputPin> Button<BTN> {
    pub fn is_pressed(&self) -> bool {
        match self.p.is_low() {
            Ok(v) => v,
            _ => false,
        }
    }
}