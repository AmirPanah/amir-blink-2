#![no_std]
#![no_main]

// Amir
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
    let mut led = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);


    loop {
        led.toggle().ok();
        cortex_m::asm::delay(clocks.sysclk().0 / 2);
    }
}
