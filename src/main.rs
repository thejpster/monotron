//! # Monotron
//!
//! > A simple 1980's home computer style application for the Tiva-C Launchpad
//!
//! See README.md for more details.
//!
//! ## SPI pins
//!
//! This chip has 4 SPI devices. They are on the following pins:
//!
//! 1.02         PB5     SSI2Fss
//! 1.07         PB4     SSI2Clk
//! 1.08         PA5     SSI0Tx
//! 2.04         PF0     SSI1Rx
//! 2.04         PF0     SSI1Rx / User Switch 2
//! 2.06/3.04    PD1/PB7 SSI2Tx / SSI3Fss / SSI1Fss
//! 2.07/3.03    PB6/PD0 SSI2Rx / SSI3Clk / SSI1Clk
//! 2.08         PA4     SSI0Rx
//! 2.09         PA3     SSI0Fss
//! 2.10         PA2     SSI0Clk
//! 3.05         PD2     SSI3Rx / SSI1Rx
//! 3.06         PD3     SSI3Tx / SSI1Tx
//! 3.10         PF1     SSI1Tx
//! 4.02         PF3     SSI1Fss
//!
//! Note that there are 0-ohm links between 2.07 and 3.03 and between 2.06 and
//! 3.04 for MSP430 compatibility reasons. This limits the pins we can use for
//! SPI.
//!
//! We use:
//! * SSI1Tx for Red on PF1 / 3.10
//! * SSI2Tx for Green on PB7 / 3.04=2.06
//! * SSI3Tx for Blue on PD3 / 3.06
//! * Timer1 Channel A PB4 is H-Sync
//! * GPIO PB5 is V-Sync
//! * M0PWM4 for Audio on PE4 / 1.05.
//! * PC6/PC7/PD6/PD7/PF4 for an Atari 9-pin Joystick.
//! * SSI0 (PA2/PA3/PA4/PA5) for an SD/MMC Interface
//!
//! Reserved for future use:
//! * SSI0, for interfacing with an SD/MMC card.
//! * PB0/PB1/PC4/PC5 for a 5-wire 3.3v UART.
//! * PB2/PE0 for PS/2 +CLK and +DATA.

#![no_main]
#![no_std]
#![feature(asm)]

mod api;
mod ui;

extern crate panic_halt;

use core::fmt::Write;
use cortex_m_rt::{entry, exception};
use fb::AsciiConsole;
use monotron_synth::*;
use tm4c123x_hal as hal;
use vga_framebuffer as fb;

use self::cpu::{interrupt, Interrupt};
use self::hal::bb;
use self::hal::prelude::*;
use self::hal::serial::{NewlineMode, Serial};
use self::hal::sysctl;
use self::hal::tm4c123x as cpu;

const ISR_LATENCY: u32 = 94;
const TOTAL_RAM_LEN: usize = 32768;
const OS_RAM_LEN: usize = 8192;
const APPLICATION_START_ADDR: *mut u8 = (0x20000000 + OS_RAM_LEN) as *mut u8;
const APPLICATION_LEN: usize = TOTAL_RAM_LEN - OS_RAM_LEN;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static GIT_DESCRIBE: &'static str = env!("GIT_DESCRIBE");
static mut G_SYNTH: Synth = Synth::new(80_000_000 / 2112);
// static FRAMEBUFFER: fb::FrameBuffer<VideoHardware> = fb::FrameBuffer::new();

struct VideoHardware {
    h_timer: cpu::TIMER1,
    red_ch: cpu::SSI1,
    green_ch: cpu::SSI2,
    blue_ch: cpu::SSI3,
}

struct Joystick {
    up: hal::gpio::gpioe::PE2<hal::gpio::Input<hal::gpio::PullUp>>,
    down: hal::gpio::gpioe::PE3<hal::gpio::Input<hal::gpio::PullUp>>,
    left: hal::gpio::gpiod::PD6<hal::gpio::Input<hal::gpio::PullUp>>,
    right: hal::gpio::gpiod::PD7<hal::gpio::Input<hal::gpio::PullUp>>,
    fire: hal::gpio::gpiof::PF4<hal::gpio::Input<hal::gpio::PullUp>>,
}

struct DummyTimeSource;

impl embedded_sdmmc::TimeSource for DummyTimeSource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

struct Context {
    pub value: u32,
    usb_uart: hal::serial::Serial<
        hal::serial::UART0,
        hal::gpio::gpioa::PA1<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        hal::gpio::gpioa::PA0<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        (),
        (),
    >,
    avr_uart: hal::serial::Serial<
        hal::serial::UART7,
        hal::gpio::gpioe::PE1<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        hal::gpio::gpioe::PE0<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        (),
        (),
    >,
    #[allow(dead_code)] // we'll get on to this later
    midi_uart: hal::serial::Serial<
        hal::serial::UART3,
        hal::gpio::gpioc::PC7<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        hal::gpio::gpioc::PC6<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        (),
        (),
    >,
    #[allow(dead_code)] // we'll get on to this later
    rs232_uart: hal::serial::Serial<
        hal::serial::UART1,
        hal::gpio::gpiob::PB1<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        hal::gpio::gpiob::PB0<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        hal::gpio::gpioc::PC4<hal::gpio::AlternateFunction<hal::gpio::AF8, hal::gpio::PushPull>>,
        hal::gpio::gpioc::PC5<hal::gpio::AlternateFunction<hal::gpio::AF8, hal::gpio::PushPull>>,
    >,
    keyboard: pc_keyboard::Keyboard<pc_keyboard::layouts::Uk105Key, pc_keyboard::ScancodeSet2>,
    buffered_char: Option<Input>,
    joystick: Joystick,
    cont: embedded_sdmmc::Controller<
        embedded_sdmmc::SdMmcSpi<
            hal::spi::Spi<
                cpu::SSI0,
                (
                    hal::gpio::gpioa::PA2<
                        hal::gpio::AlternateFunction<hal::gpio::AF2, hal::gpio::PushPull>,
                    >,
                    hal::gpio::gpioa::PA4<
                        hal::gpio::AlternateFunction<hal::gpio::AF2, hal::gpio::PushPull>,
                    >,
                    hal::gpio::gpioa::PA5<
                        hal::gpio::AlternateFunction<hal::gpio::AF2, hal::gpio::PushPull>,
                    >,
                ),
            >,
            hal::gpio::gpioa::PA3<hal::gpio::Output<hal::gpio::PushPull>>,
        >,
        DummyTimeSource,
    >,
    clocks: hal::sysctl::Clocks,
    seen_keypress: bool,
    console: fb::Console,
}

enum Input {
    Special(pc_keyboard::KeyCode),
    Cp850(u8),
}

#[derive(Copy, Clone, Debug)]
pub struct JoystickState(u8);

impl JoystickState {
    const UP: u8 = 0b10000;
    const DOWN: u8 = 0b01000;
    const LEFT: u8 = 0b00100;
    const RIGHT: u8 = 0b00010;
    const FIRE: u8 = 0b00001;

    fn new(up: bool, down: bool, left: bool, right: bool, fire: bool) -> Self {
        let mut b = 0;
        if up {
            b |= Self::UP;
        }
        if down {
            b |= Self::DOWN;
        }
        if left {
            b |= Self::LEFT;
        }
        if right {
            b |= Self::RIGHT;
        }
        if fire {
            b |= Self::FIRE;
        }
        JoystickState(b)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn is_up(&self) -> bool {
        (self.0 & Self::UP) != 0
    }

    pub fn is_down(&self) -> bool {
        (self.0 & Self::DOWN) != 0
    }

    pub fn is_left(&self) -> bool {
        (self.0 & Self::LEFT) != 0
    }

    pub fn is_right(&self) -> bool {
        (self.0 & Self::RIGHT) != 0
    }

    pub fn fire_pressed(&self) -> bool {
        (self.0 & Self::FIRE) != 0
    }
}

impl Joystick {
    fn get_state(&self) -> JoystickState {
        let is_up = self.up.is_low();
        let is_down = self.down.is_low();
        let is_left = self.left.is_low();
        let is_right = self.right.is_low();
        let is_fire = self.fire.is_low();
        JoystickState::new(is_up, is_down, is_left, is_right, is_fire)
    }
}

impl Context {
    fn has_char(&mut self) -> bool {
        let attempt = self.read();
        if attempt.is_some() {
            self.buffered_char = attempt;
            true
        } else {
            false
        }
    }

    fn read(&mut self) -> Option<Input> {
        if self.buffered_char.is_some() {
            let mut x = None;
            core::mem::swap(&mut self.buffered_char, &mut x);
            return x;
        }
        if let Ok(ch) = self.usb_uart.read() {
            // Got some serial input
            // Backspace key in screen seems to generate 0x7F (delete).
            // Map it to backspace (0x08)
            if ch == 0x7F {
                Some(Input::Cp850(0x08))
            } else {
                // Just bodge the UTF-8 input into CP850 and hope for the best
                Some(Input::Cp850(ch))
            }
        } else {
            let key = if let Ok(ch) = self.avr_uart.read() {
                // Got something in the buffer from the AVR
                match self.keyboard.add_byte(ch) {
                    Ok(Some(event)) => {
                        self.seen_keypress = true;
                        self.keyboard.process_keyevent(event)
                    }
                    Ok(None) => None,
                    Err(e) if self.seen_keypress => {
                        writeln!(self, "Bad key input! {:?} (0x{:02x})", e, ch).unwrap();
                        None
                    }
                    Err(_e) => {
                        // Squash any random errors on start-up
                        None
                    }
                }
            } else {
                None
            };

            match key {
                None => None,
                Some(pc_keyboard::DecodedKey::Unicode(c)) => {
                    if c == '\n' {
                        // Return generates \n but menu wants \r
                        Some(Input::Cp850(b'\r'))
                    } else {
                        // Er, do a better Unicode to CP850 translation here!
                        let byte = fb::Char::map_char(c) as u8;
                        Some(Input::Cp850(byte))
                    }
                }
                Some(pc_keyboard::DecodedKey::RawKey(code)) => {
                    // Handle raw keypress that can't be represented in Unicode
                    // here (e.g. Insert, Page Down, etc)
                    Some(Input::Special(code))
                }
            }
        }
    }

    /// Write an 8-bit ASCII character to the screen.
    fn write_u8(&mut self, ch: u8) {
        self.console.write_character(ch).unwrap();
    }
}

impl core::fmt::Write for Context {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.console.write_str(s)
    }
}

fn enable(p: sysctl::Domain, sc: &mut hal::sysctl::PowerControl) {
    sysctl::control_power(sc, p, sysctl::RunMode::Run, sysctl::PowerState::On);
    sysctl::control_power(sc, p, sysctl::RunMode::Sleep, sysctl::PowerState::On);
    sysctl::reset(sc, p);
}

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
    let cp = hal::CorePeripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = sysctl::Oscillator::Main(
        sysctl::CrystalFrequency::_16mhz,
        sysctl::SystemClock::UsePll(sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sc.clock_setup.freeze();

    let mut nvic = cp.NVIC;
    nvic.enable(Interrupt::TIMER1A);
    nvic.enable(Interrupt::TIMER1B);
    // nvic.enable(Interrupt::GPIOD);
    // Make Timer1A (start of line) lower priority than Timer1B (clocking out
    // data) so that it can be interrupted.
    // Make GPIOD (the keyboard) between the two. We might corrupt
    // a bit while scheduling the line start, but that's probably better
    // than getting a wonky video signal?
    // Priorities go from 0*16 (most urgent) to 15*16 (least urgent)
    // EEE trying with keyboard higher than video
    unsafe {
        nvic.set_priority(Interrupt::TIMER1A, 8 * 16);
        nvic.set_priority(Interrupt::TIMER1B, 4 * 16);
        // nvic.set_priority(Interrupt::GPIOD, 3*16);
    }

    enable(sysctl::Domain::Timer1, &mut sc.power_control);
    enable(sysctl::Domain::Ssi0, &mut sc.power_control);
    enable(sysctl::Domain::Ssi1, &mut sc.power_control);
    enable(sysctl::Domain::Ssi2, &mut sc.power_control);
    enable(sysctl::Domain::Ssi3, &mut sc.power_control);
    enable(sysctl::Domain::Pwm0, &mut sc.power_control);

    let mut porta = p.GPIO_PORTA.split(&sc.power_control);
    let mut portb = p.GPIO_PORTB.split(&sc.power_control);
    let mut portc = p.GPIO_PORTC.split(&sc.power_control);
    let mut portd = p.GPIO_PORTD.split(&sc.power_control);
    let mut porte = p.GPIO_PORTE.split(&sc.power_control);
    let mut portf = p.GPIO_PORTF.split(&sc.power_control);

    // T0CCP0
    let _h_sync = portb
        .pb4
        .into_af_push_pull::<hal::gpio::AF7>(&mut portb.control);
    // GPIO controlled V-Sync
    let _v_sync = portb.pb5.into_push_pull_output();
    // Ssi1Tx
    let _red_data = portf
        .pf1
        .into_af_push_pull::<hal::gpio::AF2>(&mut portf.control);
    // Ssi2Tx
    let _green_data = portb
        .pb7
        .into_af_push_pull::<hal::gpio::AF2>(&mut portb.control);
    // Ssi3Tx
    let _blue_data = portd
        .pd3
        .into_af_push_pull::<hal::gpio::AF1>(&mut portd.control);
    // Audio PWM output
    let _audio_pin = porte
        .pe4
        .into_af_push_pull::<hal::gpio::AF4>(&mut porte.control);

    // Configure PWM peripheral. We use M0PWM4 on PE4. That's pwmA on the third
    // pair (the pairs are 0/1, 2/3, 4/5 and 6/7) of the first PWM peripheral.
    // We want up/down mode. We have a load value 256 and a comparator value
    // set according to the volume level for the current sample.

    let pwm = p.PWM0;
    pwm.ctl.write(|w| w.globalsync0().clear_bit());
    // Mode = 1 => Count up/down mode
    pwm._2_ctl.write(|w| w.enable().set_bit().mode().set_bit());
    pwm._2_gena.write(|w| w.actcmpau().zero().actcmpad().one());
    // 528 cycles (264 up and down) = 4 loops per video line (2112 cycles)
    pwm._2_load.write(|w| unsafe { w.load().bits(263) });
    pwm._2_cmpa.write(|w| unsafe { w.compa().bits(64) });
    pwm.enable.write(|w| w.pwm4en().set_bit());

    // SSI0 for SD/MMC access
    let sdmmc_clk = porta
        .pa2
        .into_af_push_pull::<hal::gpio::AF2>(&mut porta.control);
    let sdmmc_cs = porta.pa3.into_push_pull_output();
    let sdmmc_miso = porta
        .pa4
        .into_af_push_pull::<hal::gpio::AF2>(&mut porta.control);
    let sdmmc_mosi = porta
        .pa5
        .into_af_push_pull::<hal::gpio::AF2>(&mut porta.control);
    // Use the HAL driver for SPI
    let sdmmc_spi = hal::spi::Spi::spi0(
        p.SSI0,
        (sdmmc_clk, sdmmc_miso, sdmmc_mosi),
        embedded_hal::spi::MODE_0,
        250_000.hz(),
        &clocks,
        &sc.power_control,
    );

    // USB Serial UART
    let mut usb_uart = Serial::uart0(
        p.UART0,
        porta
            .pa1
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        porta
            .pa0
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        (),
        (),
        115200_u32.bps(),
        NewlineMode::SwapLFtoCRLF,
        &clocks,
        &sc.power_control,
    );

    usb_uart.write_all(b"This is a test\r\n");

    // MIDI UART
    let midi_uart = Serial::uart3(
        p.UART3,
        portc
            .pc7
            .into_af_push_pull::<hal::gpio::AF1>(&mut portc.control),
        portc
            .pc6
            .into_af_push_pull::<hal::gpio::AF1>(&mut portc.control),
        (),
        (),
        31250_u32.bps(),
        NewlineMode::Binary,
        &clocks,
        &sc.power_control,
    );

    // AVR UART
    let avr_uart = Serial::uart7(
        p.UART7,
        porte
            .pe1
            .into_af_push_pull::<hal::gpio::AF1>(&mut porte.control),
        porte
            .pe0
            .into_af_push_pull::<hal::gpio::AF1>(&mut porte.control),
        (),
        (),
        19200_u32.bps(),
        NewlineMode::Binary,
        &clocks,
        &sc.power_control,
    );

    // RS-232 UART
    let rs232_uart = Serial::uart1(
        p.UART1,
        portb
            .pb1
            .into_af_push_pull::<hal::gpio::AF1>(&mut portb.control),
        portb
            .pb0
            .into_af_push_pull::<hal::gpio::AF1>(&mut portb.control),
        portc
            .pc4
            .into_af_push_pull::<hal::gpio::AF8>(&mut portc.control),
        portc
            .pc5
            .into_af_push_pull::<hal::gpio::AF8>(&mut portc.control),
        115200_u32.bps(),
        NewlineMode::Binary,
        &clocks,
        &sc.power_control,
    );

    let hw = VideoHardware {
        h_timer: p.TIMER1,
        red_ch: p.SSI1,
        green_ch: p.SSI2,
        blue_ch: p.SSI3,
    };
    FRAMEBUFFER.init(hw);

    // struct LoggingSpi<T> where T: embedded_hal::spi::FullDuplex<u8> {
    //     spi: T,
    // }

    // use nb;
    // impl<T> embedded_hal::spi::FullDuplex<u8> for LoggingSpi<T> where T: embedded_hal::spi::FullDuplex<u8> {
    //     type Error = T::Error;

    //     fn read(&mut self) -> nb::Result<u8, T::Error> {
    //         let b = self.spi.read();
    //         if let Ok(data) = b {
    //             unsafe { writeln!(FRAMEBUFFER, "RX 0x{:02x}", data).unwrap() };
    //         }
    //         b
    //     }

    //     fn send(&mut self, byte: u8) -> nb::Result<(), T::Error> {
    //         unsafe { write!(FRAMEBUFFER, "TX 0x{:02x} ", byte).unwrap() };
    //         self.spi.send(byte)
    //     }
    // }

    // let sdmmc_spi = LoggingSpi { spi: sdmmc_spi };

    // TODO let users pick a keyboard layout, and store their choice in EEPROM somewhere
    let keyboard = pc_keyboard::Keyboard::new(
        pc_keyboard::layouts::Uk105Key,
        pc_keyboard::ScancodeSet2,
        pc_keyboard::HandleControl::MapLettersToUnicode,
    );

    let mut c = Context {
        value: 0,
        usb_uart,
        avr_uart,
        midi_uart,
        rs232_uart,
        keyboard,
        buffered_char: None,
        joystick: Joystick {
            up: porte.pe2.into_pull_up_input(),
            down: porte.pe3.into_pull_up_input(),
            left: portd.pd6.into_pull_up_input(),
            right: portd.pd7.unlock(&mut portd.control).into_pull_up_input(),
            fire: portf.pf4.into_pull_up_input(),
        },
        cont: embedded_sdmmc::Controller::new(
            embedded_sdmmc::SdMmcSpi::new(sdmmc_spi, sdmmc_cs),
            DummyTimeSource,
        ),
        clocks,
        seen_keypress: false,
    };

    while c.usb_uart.read().is_ok() {
        // Try again and empty the buffer
    }

    FRAMEBUFFER.set_attr(fb::Attr::new(fb::Colour::White, fb::Colour::Black));
    FRAMEBUFFER.clear();

    write!(c, "\u{001b}Z\u{001b}W\u{001b}k╔══════════════════════════════════════════════╗").unwrap();
    write!(c, "║\u{001b}R█████\u{001b}K \u{001b}R\u{001b}y█████\u{001b}K\u{001b}k \u{001b}Y██  █\u{001b}K \u{001b}G█████\u{001b}K \u{001b}G\u{001b}y█\u{001b}k█\u{001b}y█\u{001b}k██\u{001b}K \u{001b}B████\u{001b}K \u{001b}B█████\u{001b}K \u{001b}M██  █\u{001b}W║").unwrap();
    write!(c, "║\u{001b}R▓\u{001b}K \u{001b}R▓\u{001b}K \u{001b}R▓\u{001b}K \u{001b}R\u{001b}y▓\u{001b}K\u{001b}k   \u{001b}R\u{001b}y▓\u{001b}K\u{001b}k \u{001b}Y▓\u{001b}K \u{001b}Y▓ ▓\u{001b}K \u{001b}G▓\u{001b}K   \u{001b}G▓\u{001b}K \u{001b}G \u{001b}K \u{001b}G\u{001b}y▓\u{001b}K\u{001b}k \u{001b}G \u{001b}K \u{001b}B\u{001b}g▓\u{001b}K\u{001b}k  \u{001b}B\u{001b}g▓\u{001b}K\u{001b}k \u{001b}B▓\u{001b}K   \u{001b}B▓\u{001b}K \u{001b}M▓\u{001b}K \u{001b}M▓ ▓\u{001b}W║").unwrap();
    write!(c, "║\u{001b}R▒\u{001b}K \u{001b}R▒\u{001b}K \u{001b}R▒\u{001b}K \u{001b}R\u{001b}y▒\u{001b}K\u{001b}k   \u{001b}R\u{001b}y▒\u{001b}K\u{001b}k \u{001b}Y▒\u{001b}K  \u{001b}Y▒▒\u{001b}K \u{001b}G▒\u{001b}K   \u{001b}G▒\u{001b}K \u{001b}G \u{001b}K \u{001b}G\u{001b}y▒\u{001b}K\u{001b}k \u{001b}G \u{001b}K \u{001b}B\u{001b}g▒\u{001b}K\u{001b}k \u{001b}B\u{001b}g▒\u{001b}k \u{001b}K \u{001b}B▒\u{001b}K   \u{001b}B▒\u{001b}K \u{001b}M▒\u{001b}K \u{001b}M ▒▒\u{001b}W║").unwrap();
    write!(c, "║\u{001b}R░ ░\u{001b}K \u{001b}R░\u{001b}K \u{001b}R\u{001b}y░░░░░\u{001b}K\u{001b}k \u{001b}Y░   ░\u{001b}K \u{001b}G░░░░░\u{001b}K \u{001b}G  \u{001b}y░\u{001b}k  \u{001b}K \u{001b}B\u{001b}g░\u{001b}k  \u{001b}g░\u{001b}K\u{001b}k \u{001b}B░░░░░\u{001b}K \u{001b}M░   ░\u{001b}W║").unwrap();
    write!(c, "╚══════════════════════════════════════════════╝").unwrap();
    writeln!(c, "Monotron v{} ({})", VERSION, GIT_DESCRIBE).unwrap();
    writeln!(c, "Copyright © theJPster 2018").unwrap();

    let stack_space = unsafe {
        extern "C" {
            static __ebss: u32;
            static __sdata: u32;
        }
        let ebss = &__ebss as *const u32 as usize;
        let start = &__sdata as *const u32 as usize;
        let total = ebss - start;
        8192 - total
    };
    writeln!(
        c,
        "{} bytes stack, {} bytes free.",
        stack_space, APPLICATION_LEN
    )
    .unwrap();

    let mut buffer = [0u8; 64];
    let mut r = menu::Runner::new(&ui::ROOT_MENU, &mut buffer, &mut c);

    loop {
        api::wfvbi(r.context);
        // Wait for new UTF-8 input
        match r.context.read() {
            Some(Input::Cp850(octet)) => {
                r.input_byte(octet);
            }
            Some(Input::Special(code)) => {
                // Can't handle special chars yet
                writeln!(r.context, "Special char {:?}", code).unwrap();
            }
            None => {}
        }
    }
}

impl fb::Hardware for VideoHardware {
    fn configure(&mut self, width: u32, sync_end: u32, line_start: u32, clock_rate: u32) {
        // Configure SPI
        // Need to configure SSI1, SSI2 and SSI3 at 20 MHz
        self.red_ch.cr1.modify(|_, w| w.sse().clear_bit());
        self.blue_ch.cr1.modify(|_, w| w.sse().clear_bit());
        self.green_ch.cr1.modify(|_, w| w.sse().clear_bit());
        // SSIClk = SysClk / (CPSDVSR * (1 + SCR))
        // e.g. 20 MHz = 80 MHz / (4 * (1 + 0))
        // CPSDVSR = 4 -------^
        // SCR = 0 --------------------^
        let ratio = 80_000_000 / clock_rate;
        // For all sensible divisors of 80 MHz, we want SCR = 0.
        self.red_ch
            .cpsr
            .write(|w| unsafe { w.cpsdvsr().bits(ratio as u8) });
        self.blue_ch
            .cpsr
            .write(|w| unsafe { w.cpsdvsr().bits(ratio as u8) });
        self.green_ch
            .cpsr
            .write(|w| unsafe { w.cpsdvsr().bits(ratio as u8) });
        self.red_ch.cr0.write(|w| {
            w.dss()._8();
            w.frf().moto();
            w.spo().clear_bit();
            w.sph().set_bit();
            w
        });
        self.blue_ch.cr0.write(|w| {
            w.dss()._8();
            w.frf().moto();
            w.spo().clear_bit();
            w.sph().set_bit();
            w
        });
        self.green_ch.cr0.write(|w| {
            w.dss()._8();
            w.frf().moto();
            w.spo().clear_bit();
            w.sph().set_bit();
            w
        });
        // Set clock source to sysclk
        self.red_ch.cc.modify(|_, w| w.cs().syspll());
        self.blue_ch.cc.modify(|_, w| w.cs().syspll());
        self.green_ch.cc.modify(|_, w| w.cs().syspll());

        // Configure Timer1A for h-sync and Timer1B for line trigger
        self.h_timer.ctl.modify(|_, w| {
            w.taen().clear_bit();
            w.tben().clear_bit();
            w
        });
        self.h_timer.cfg.modify(|_, w| w.cfg()._16_bit());
        self.h_timer.tamr.modify(|_, w| {
            w.taams().set_bit();
            w.tacmr().clear_bit();
            w.tapwmie().set_bit();
            w.tamr().period();
            w
        });
        self.h_timer.tbmr.modify(|_, w| {
            w.tbams().set_bit();
            w.tbcmr().clear_bit();
            w.tbmr().period();
            w.tbpwmie().set_bit();
            w
        });
        self.h_timer.ctl.modify(|_, w| {
            // Trigger Timer A capture on rising edge (i.e. line start)
            w.tapwml().clear_bit();
            // Trigger Timer B capture on falling edge (i.e. data start)
            w.tbpwml().set_bit();
            w
        });
        // We're counting down in PWM mode, so start at the end
        // We start 16 pixels early
        let multiplier = 80_000_000 / clock_rate;
        self.h_timer
            .tailr
            .modify(|_, w| unsafe { w.bits(width * multiplier - 1) });
        self.h_timer
            .tbilr
            .modify(|_, w| unsafe { w.bits(width * multiplier - 1) });
        self.h_timer
            .tamatchr
            .modify(|_, w| unsafe { w.bits(multiplier * (width - sync_end) - 1) });
        self.h_timer.tbmatchr.modify(|_, w| unsafe {
            w.bits((multiplier * (width - line_start)) + ISR_LATENCY - 1)
        });
        self.h_timer.imr.modify(|_, w| {
            w.caeim().set_bit(); // Timer1A fires at start of line
            w.cbeim().set_bit(); // Timer1B fires at start of data
            w
        });

        // Clear interrupts
        self.h_timer.icr.write(|w| {
            w.tbmcint().set_bit();
            w.tbtocint().set_bit();
            w
        });

        self.h_timer.ctl.modify(|_, w| {
            w.taen().set_bit();
            w.tben().set_bit();
            w
        });
    }

    /// Called when V-Sync needs to be high.
    fn vsync_on(&mut self) {
        let gpio = unsafe { &*cpu::GPIO_PORTB::ptr() };
        unsafe { bb::change_bit(&gpio.data, 5, true) };
    }

    /// Called when V-Sync needs to be low.
    fn vsync_off(&mut self) {
        let gpio = unsafe { &*cpu::GPIO_PORTB::ptr() };
        unsafe { bb::change_bit(&gpio.data, 5, false) };
    }

    /// Write pixels straight to FIFOs
    fn write_pixels(&mut self, red: u32, green: u32, blue: u32) {
        let ssi_r = unsafe { &*cpu::SSI1::ptr() };
        let ssi_g = unsafe { &*cpu::SSI2::ptr() };
        let ssi_b = unsafe { &*cpu::SSI3::ptr() };
        while (ssi_r.sr.read().bits() & 0x02) == 0 {}
        ssi_r.dr.write(|w| unsafe { w.bits(red) });
        ssi_g.dr.write(|w| unsafe { w.bits(green) });
        ssi_b.dr.write(|w| unsafe { w.bits(blue) });
    }
}

/// Called on start of sync pulse (end of front porch)
interrupt!(TIMER1A, timer1a);
fn timer1a() {
    static FRAMEBUFFER: Framebuffer = Framebuffer::new();

    let pwm = unsafe { &*cpu::PWM0::ptr() };
    static mut NEXT_SAMPLE: u8 = 128;
    pwm._2_cmpa
        .write(|w| unsafe { w.compa().bits(NEXT_SAMPLE as u16) });
    let ssi_r = unsafe { &*cpu::SSI1::ptr() };
    let ssi_g = unsafe { &*cpu::SSI2::ptr() };
    let ssi_b = unsafe { &*cpu::SSI3::ptr() };
    // Disable the SPIs as we don't want pixels yet
    ssi_r.cr1.modify(|_, w| w.sse().clear_bit());
    ssi_g.cr1.modify(|_, w| w.sse().clear_bit());
    ssi_b.cr1.modify(|_, w| w.sse().clear_bit());
    // Pre-load red with 2 bytes and green 1 with (they start early so we can line them up)
    ssi_r.dr.write(|w| unsafe { w.data().bits(0) });
    ssi_r.dr.write(|w| unsafe { w.data().bits(0) });
    ssi_g.dr.write(|w| unsafe { w.data().bits(0) });
    // Run the draw routine
    FRAMEBUFFER.isr_sol();
    // Run the audio routine
    unsafe {
        NEXT_SAMPLE = G_SYNTH.next().into();
    }
    // Clear timer A interrupt
    let timer = unsafe { &*cpu::TIMER1::ptr() };
    timer.icr.write(|w| w.caecint().set_bit());
}

/// Called on start of pixel data (end of back porch)
interrupt!(TIMER1B, timer1b);
fn timer1b() {
    unsafe {
        /// Activate the three FIFOs exactly 32 clock cycles (or 8 pixels) apart This
        /// gets the colour video lined up, as we preload the red channel with 0x00
        /// 0x00 and the green channel with 0x00.
        asm!(
            "movs    r0, #132;
            movs    r1, #1;
            movt    r0, #16914;
            mov.w   r2, #131072;
            mov.w   r3, #262144;
            str r1, [r0, #0];
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            str r1, [r0, r2];
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            nop;
            str r1, [r0, r3];
            "
            :
            :
            : "r0" "r1" "r2" "r3"
            : "volatile");
    }
    // Clear timer B interrupt
    let timer = unsafe { &*cpu::TIMER1::ptr() };
    timer.icr.write(|w| w.cbecint().set_bit());
}

#[exception]
/// The hard fault handler
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
/// The default exception handler
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

// End of file
