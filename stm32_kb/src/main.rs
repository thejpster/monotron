//! # Monotron Keyboard Controller
//!
//! A PS/2 keyboard or mouse sends a synchronous signal at 10 kHz. Because
//! Monotron is very busy calculating pixels to send to the VGA port, it can't
//! handle the interrupts (and there's no SPI peripherals left to handle it in
//! hardware).
//!
//! Instead, we use this separate little STM32 chip.

#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;
use cortex_m_rt::entry;
use hal::{prelude::*, serial::Serial, stm32 as pac};
use stm32f0xx_hal as hal;

static GIT_DESCRIBE: &'static str = env!("GIT_DESCRIBE");

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut flash = p.FLASH;
    let mut rcc = p.RCC.configure().hsi48().freeze(&mut flash);
    let gpioa = p.GPIOA.split(&mut rcc);
    let (tx, rx) = cortex_m::interrupt::free(move |cs| {
        (
            gpioa.pa2.into_alternate_af1(cs),
            gpioa.pa3.into_alternate_af1(cs),
        )
    });

    let mut uart = Serial::usart2(p.USART2, (tx, rx), 115200.bps(), &mut rcc);

    write!(uart, "Booted version: {}", GIT_DESCRIBE).unwrap();
    loop {
        uart.write_str("Hello, world!\r\n").unwrap();
    }
}
