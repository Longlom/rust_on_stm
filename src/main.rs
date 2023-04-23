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

fn _pwm_stmod() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.smps().freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);
    // Configuring pwm
    let tim4 = dp.TIM4;
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);

    let pd14 = gpiod.pd14.into_alternate();

    // Configure PWM at 10kHz
    let mut pwm = tim4.pwm(pd14, 50.Hz(), ccdr.peripheral.TIM4, &ccdr.clocks);

    // Output PWM on pd14 (on fanout board)
    // let max_duty = pwm.get_max_duty() / 2;
    let middle = 4915;
    let max = 6553;
    let min = 1;
    // pwm.set_duty(max_duty);
    pwm.enable();
    hprintln!("PWM for fanout SET").unwrap();
    loop {
        pwm.set_duty(middle);

        hprintln!("Set middle").unwrap();
        // Wait for a few seconds
        delay.delay_ms(3000_u16);

        pwm.set_duty(max);
        hprintln!("Set max").unwrap();
        delay.delay_ms(3000_u16);

        pwm.set_duty(min);
        hprintln!("Set min").unwrap();
        delay.delay_ms(3000_u16);

    }
}

fn _blink_ld2() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.smps().freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let mut led = gpioc.pc3.into_push_pull_output();

    loop {
        led.set_high();
        delay.delay_ms(1000_u16);

        led.set_low();
        delay.delay_ms(1000_u16);
    }
}



fn hello_world() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.smps().freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // Get the delay provider.
    let mut _delay = cp.SYST.delay(ccdr.clocks);

hprintln!("HELLO WORLD!").unwrap();

    loop {

    }
}

#[entry]
fn main() -> ! {
    hello_world()
    // pwm_stmod();
    //    blink_ld2();
    //    pwm_d9()

    //  loop { }
}
