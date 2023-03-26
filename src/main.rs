//! Basic example that produces a 1Hz square-wave on Pin PE1

#![deny(warnings)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
// use cortex_m_semihosting::hprintln;
use stm32h7xx_hal::{pac, prelude::*};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        // hprintln!("PANIC - {:?}", info).unwrap();
    }
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    // hprintln!("Setup PWR...                  ").unwrap();
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.smps().freeze();
    // Constrain and Freeze clock
    // hprintln!("Setup RCC...                  ").unwrap();
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // hprintln!("").unwrap();
    // hprintln!("stm32h7xx-hal example - Blinky").unwrap();
    // hprintln!("").unwrap();

    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // Configure PE1 as output.
    let mut led = gpioc.pc3.into_push_pull_output();
    let d1_port = gpiob.pb15.into_pull_up_input();


    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        let result = d1_port.is_high();
        hprintln!("Result - {}", result).unwrap();

        led.set_high();
        delay.delay_ms(1000_u16);

        led.set_low();
        delay.delay_ms(1000_u16);
    }
}
