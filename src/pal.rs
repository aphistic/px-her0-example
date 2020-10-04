/*
See:
http://blog.japaric.io/brave-new-io/
https://pramode.in/2018/02/24/an-introduction-to-writing-embedded-hal-based-drivers-in-rust/

https://github.com/piconomix/piconomix-fwlib/blob/master/boards/arm/stm32/px_hero/src/px_board.c
https://github.com/piconomix/piconomix-fwlib/blob/master/boards/arm/stm32/px_hero/inc/px_board_gpio.h
https://github.com/piconomix/piconomix-fwlib/blob/master/boards/arm/stm32/px_hero/inc/px_board.h
*/

use stm32l0xx_hal::{prelude::*, pac, rcc::Config};

use crate::button;
use crate::led;

pub struct PAL {
    pub led_usr: led::Led<led::LEDUSR>,
    pub led_lcd: led::Led<led::LEDLCD>,
    
    pub btn1_left: button::Button<button::BTN1>,
    pub btn2_right: button::Button<button::BTN2>,
    pub btn3_up: button::Button<button::BTN3>,
    pub btn4_down: button::Button<button::BTN4>,
    pub btn5_yes: button::Button<button::BTN5>,
    pub btn6_no: button::Button<button::BTN6>,
}

impl PAL {
    pub fn new() -> Self {
        let dp = pac::Peripherals::take().unwrap();

        // No idea what this does or why it does it yet!
        let mut rcc = dp.RCC.freeze(Config::hsi16());

        let gpioa = dp.GPIOA.split(&mut rcc);
        let gpiob = dp.GPIOB.split(&mut rcc);
        let gpioc = dp.GPIOC.split(&mut rcc);
        let gpioh = dp.GPIOH.split(&mut rcc);

        PAL{
            led_usr: gpioh.ph0.into_push_pull_output().into(),
            led_lcd: gpiob.pb12.into_push_pull_output().into(),

            btn1_left: gpioc.pc0.into_pull_up_input().into(),
            btn2_right: gpioh.ph1.into_pull_up_input().into(),
            btn3_up: gpioc.pc13.into_pull_up_input().into(),
            btn4_down: gpioc.pc12.into_pull_up_input().into(),
            btn5_yes: gpioa.pa15.into_pull_up_input().into(),
            btn6_no: gpioc.pc9.into_pull_up_input().into(),
        }
    }
}