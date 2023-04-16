#![deny(warnings)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32h7xx_hal::{pac, prelude::*};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        // hprintln!("PANIC - {:?}", info).unwrap();
    }
}

fn _pwm_d9() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    // hprintln!("Setup PWR...                  ").unwrap();
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.smps().freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);
    // Configuring pwm
    let tim4 = dp.TIM4;
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    let pb7 = gpiob.pb7.into_alternate();

    // Configure PWM at 10kHz
    let mut pwm = tim4.pwm(pb7, 10.kHz(), ccdr.peripheral.TIM4, &ccdr.clocks);

    // Output PWM on PB7
    let max_duty = pwm.get_max_duty();
    let min_duty = max_duty / 100;
    pwm.set_duty(max_duty);
    pwm.enable();
    hprintln!("PWM SET").unwrap();
    loop {
        delay.delay_ms(1000_u16);
        pwm.set_duty(max_duty);
        delay.delay_ms(1000_u16);
        pwm.set_duty(min_duty);
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
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    // let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    // let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // let stmod = gpioe.pe9.into_pull_up_input();
    // Configure PE1 as output.
    let mut led = gpioc.pc3.into_push_pull_output();
    // let d1_port = gpiob.pb15.into_pull_up_input();

    loop {
        // let result = d1_port.is_high();
        // hprintln!("Result - {}", result).unwrap();
        // let result = stmod.is_high();
        // hprintln!("Result - {}", result).unwrap();
        led.set_high();
        delay.delay_ms(1000_u16);

        led.set_low();
        delay.delay_ms(1000_u16);
    }
}
