#![no_std]
#![no_main]


use panic_halt as _;
use cortex_m_rt::entry;
use stm32f3xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut led4 = gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led3 = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led5 = gpioe.pe10.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led7 = gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led9 = gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led10 = gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led8 = gpioe.pe14.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led6 = gpioe.pe15.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let delay = clocks.sysclk().0 / 6;

    loop {
        led4.set_high().ok();  cortex_m::asm::delay(delay); led4.set_low().ok();
        led3.set_high().ok();  cortex_m::asm::delay(delay); led3.set_low().ok();
        led5.set_high().ok();  cortex_m::asm::delay(delay); led5.set_low().ok();
        led7.set_high().ok();  cortex_m::asm::delay(delay); led7.set_low().ok();
        led9.set_high().ok();  cortex_m::asm::delay(delay); led9.set_low().ok();
        led10.set_high().ok(); cortex_m::asm::delay(delay); led10.set_low().ok();
        led8.set_high().ok();  cortex_m::asm::delay(delay); led8.set_low().ok();
        led6.set_high().ok();  cortex_m::asm::delay(delay); led6.set_low().ok();
    }
}

