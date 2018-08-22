//! # Monotron
//!
//! > A simple 1980's home computer style application for the Stellaris Launchpad
//!
//! See README.md for more details.
//!
//! ## SPI pins
//!
//! This chip has 4 SPI devices. They are on the following pins:
//!
//! 1.08         PA5     SSI0Tx
//! 2.06/3.04    PD1/PB7 SSI2Tx
//! 3.06         PD3     SSI3Tx / SSI1Tx
//!
//! 2.08         PA4     SSI0Rx
//! 2.04         PF0     SSI1Rx / User Switch 2
//! 2.07/3.03    PB6/PD0 SSI2Rx / SSI3Clk / SSI1Clk
//! 3.05         PD2     SSI3Rx / SSI1Rx
//!
//! 2.10         PA2     SSI0Clk
//! 2.07/3.03    PB6/PD0 SSI2Rx / SSI3Clk / SSI1Clk
//! 1.07         PB4     SSI2Clk
//!
//! Note that there are 0-ohm links between 2.07 and 3.03 and between 2.06 and
//! 3.04 for MSP430 compatibility reasons. This limits the pins we can use for
//! SPI.
//!
//! We use:
//! * SSI0Tx for Red on PA5 / 1.08
//! * SSI1Tx for Blue on PD3 / 3.06
//! * SSI2Tx for Green on PB7 / 3.04 / 2.06
//! * SSI3Rx for Keyboard Data on PD2 / 3.05
//! * SSI3Clk for Keyboard Clock on PD0 / 2.07 / 3.03
//! * Timer1 Channel A PB4 is H-Sync
//! * GPIO PC4 is V-Sync

#![no_main]
#![feature(asm)]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate panic_semihosting;
extern crate cortex_m_semihosting;
extern crate menu;
extern crate pc_keyboard;
#[macro_use]
extern crate tm4c123x_hal;
extern crate vga_framebuffer as fb;

mod ui;

use core::fmt::Write;
use cortex_m::asm;
use tm4c123x_hal::prelude::*;
use tm4c123x_hal::bb;
use tm4c123x_hal::serial::{NewlineMode, Serial};
use tm4c123x_hal::sysctl;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const GIT_DESCRIBE: &'static str = env!("GIT_DESCRIBE");
const ISR_LATENCY: u32 = 94;

static mut FRAMEBUFFER: fb::FrameBuffer<VideoHardware> = fb::FrameBuffer::new();

static mut KEYBOARD_REGISTER: u16 = 0;
static mut KEYBOARD_BITS: usize = 0;
static mut KEYBOARD_LAST_BIT_AT: u64 = 0;
static mut KEYBOARD_WORDS: [u16; 8] = [0u16; 8];
static mut KEYBOARD_WRITE_INDEX: usize = 0;
static mut KEYBOARD_READ_INDEX: usize = 0;

static mut APPLICATION_RAM: [u8; 24*1024] = [0u8; 24*1024];

/// Max video lines (@ 26.5us per line) in-between keyboard bits. Keyboard must be no slower
/// than 10kHz. Set to 9kHz to be safe => 8,888 clocks.
const MAX_KEYBOARD_BIT_LINES: u32 = 4;

struct VideoHardware {
    h_timer: tm4c123x_hal::tm4c123x::TIMER1,
    red_ch: tm4c123x_hal::tm4c123x::SSI0,
    blue_ch: tm4c123x_hal::tm4c123x::SSI1,
    green_ch: tm4c123x_hal::tm4c123x::SSI2,
}

struct Context {
    pub value: u32,
    rx: tm4c123x_hal::serial::Rx<
        tm4c123x_hal::serial::UART0,
        tm4c123x_hal::gpio::gpioa::PA0<
            tm4c123x_hal::gpio::AlternateFunction<
                tm4c123x_hal::gpio::AF1,
                tm4c123x_hal::gpio::PushPull,
            >,
        >,
        (),
    >,
    keyboard: pc_keyboard::Keyboard<pc_keyboard::layouts::Uk105Key>,
    // spi: tm4c123x_hal::tm4c123x::SSI3
}

enum Input {
    Unicode(char),
    Special(pc_keyboard::KeyCode),
    Utf8(u8)
}


impl Context {
    fn keyboard_read(&mut self) -> Option<u16> {
        // Check unsafe global state mutated by ISR
        unsafe {
            if KEYBOARD_WRITE_INDEX != KEYBOARD_READ_INDEX {
                let data = KEYBOARD_WORDS[KEYBOARD_READ_INDEX % KEYBOARD_WORDS.len()];
                KEYBOARD_READ_INDEX = KEYBOARD_READ_INDEX.wrapping_add(1);
                Some(data)
            } else {
                None
            }
        }
    }

    fn read(&mut self) -> Option<Input> {
        if let Ok(ch) = self.rx.read() {
            // Got some serial input
            Some(Input::Utf8(ch))
        } else {
            let key = if let Some(word) = self.keyboard_read() {
                // writeln!(self, "Got 0x{:04} from kb {}/{}\n", word, unsafe { KEYBOARD_READ_INDEX }, unsafe { KEYBOARD_WRITE_INDEX }).unwrap();
                // Got something in the keyboard buffer
                match self.keyboard.add_word(word) {
                    Ok(Some(event)) => self.keyboard.process_keyevent(event),
                    Ok(None) => None,
                    Err(e) => {
                        writeln!(self, "Bad key input! {:?}", e).unwrap();
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
                        Some(Input::Unicode('\r'))
                    } else {
                        Some(Input::Unicode(c))
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
}

impl core::fmt::Write for Context {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { FRAMEBUFFER.write_str(s) }
    }
}

fn enable(p: sysctl::Domain, sc: &mut tm4c123x_hal::sysctl::PowerControl) {
    sysctl::control_power(sc, p, sysctl::RunMode::Run, sysctl::PowerState::On);
    sysctl::control_power(sc, p, sysctl::RunMode::Sleep, sysctl::PowerState::On);
    sysctl::reset(sc, p);
}

entry!(main);

fn main() -> ! {
    let p = tm4c123x_hal::Peripherals::take().unwrap();
    let cp = tm4c123x_hal::CorePeripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = sysctl::Oscillator::Main(
        sysctl::CrystalFrequency::_16mhz,
        sysctl::SystemClock::UsePll(sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sc.clock_setup.freeze();

    let mut nvic = cp.NVIC;
    nvic.enable(tm4c123x_hal::Interrupt::TIMER1A);
    nvic.enable(tm4c123x_hal::Interrupt::TIMER1B);
    nvic.enable(tm4c123x_hal::Interrupt::GPIOD);
    // Make Timer1A (start of line) lower priority than Timer1B (clocking out
    // data) so that it can be interrupted.
    // Make GPIOD (the keyboard) between the two. We might corrupt
    // a bit while scheduling the line start, but that's probably better
    // than getting a wonky video signal?
    // Priorities go from 0*16 (most urgent) to 15*16 (least urgent)
    unsafe {
        nvic.set_priority(tm4c123x_hal::Interrupt::TIMER1A, 15*16);
        nvic.set_priority(tm4c123x_hal::Interrupt::TIMER1B, 0*16);
        nvic.set_priority(tm4c123x_hal::Interrupt::GPIOD, 8*16);
    }

    enable(sysctl::Domain::Timer1, &mut sc.power_control);
    enable(sysctl::Domain::Ssi0, &mut sc.power_control);
    enable(sysctl::Domain::Ssi1, &mut sc.power_control);
    enable(sysctl::Domain::Ssi2, &mut sc.power_control);
    enable(sysctl::Domain::Ssi3, &mut sc.power_control);

    let mut porta = p.GPIO_PORTA.split(&sc.power_control);
    let mut portb = p.GPIO_PORTB.split(&sc.power_control);
    let portc = p.GPIO_PORTC.split(&sc.power_control);
    let mut portd = p.GPIO_PORTD.split(&sc.power_control);
    let portf = p.GPIO_PORTF.split(&sc.power_control);

    let mut red_led = portf.pf1.into_push_pull_output();
    red_led.set_low();

    // T0CCP0
    let _h_sync = portb
        .pb4
        .into_af_push_pull::<tm4c123x_hal::gpio::AF7>(&mut portb.control);
    // GPIO controlled V-Sync
    let _v_sync = portc.pc4.into_push_pull_output();
    // Ssi0Tx
    let _red_data = porta
        .pa5
        .into_af_push_pull::<tm4c123x_hal::gpio::AF2>(&mut porta.control);
    // Ssi1Tx
    let _blue_data = portd
        .pd3
        .into_af_push_pull::<tm4c123x_hal::gpio::AF2>(&mut portd.control);
    // Ssi2Tx
    let _green_data = portb
        .pb7
        .into_af_push_pull::<tm4c123x_hal::gpio::AF2>(&mut portb.control);

    // Keyboard produces 5V so set pins to open-drain to make them 5V tolerant
    // We set them floating as we have external pull-ups to 5V

    // Keyboard Clock
    let mut keyboard_clock = portd
        .pd0
        .into_pull_down_input();
    //        .into_af_open_drain::<tm4c123x_hal::gpio::AF1, tm4c123x_hal::gpio::Floating>(&mut portd.control);
    // EEE PD0 is the red wire, which goes to Channel B on the scope (red trace)
    // It comes out of the green wire on the PS/2 socket

    // Keyboard Data
    let _keyboard_data = portd
        .pd2
        .into_pull_down_input();
    //        .into_af_open_drain::<tm4c123x_hal::gpio::AF1, tm4c123x_hal::gpio::Floating>(&mut portd.control);
    // EEE PD2 is the yellow wire, which goes to Channel A on the scope (blue trace)
    // It comes out of the white wire on the PS/2 socket

    // Interrupt on clock input. We sample data pin on falling edge.
    keyboard_clock.set_interrupt_mode(tm4c123x_hal::gpio::InterruptMode::EdgeFalling);

    // After this, we expect GPIOD (0x4000_7000) to be:
    // DIR (+0x400) = 0x00
    // IS  (+0x404) = 0x00
    // IBE (+0x408) = 0x00
    // IEV (+0x40C) = 0x00
    // IME (+0x410) = 0x01
    // RIS (+0x414)
    // MIS (+0x418)

    // // Need to configure SSI3 as a slave
    // p.SSI3.cr1.write(|w| w.sse().clear_bit());
    // // Slave mode
    // p.SSI3.cr1.modify(|_, w| w.ms().set_bit());
    // // SSIClk = SysClk / (CPSDVSR * (1 + SCR))
    // // So CPSDVSR = 160, SCR = 0 gives us 500 kHz which is fine
    // // >> "For slave mode, the system clock or the PIOSC must be at least 12
    // // >> times faster than the SSInClk, with the restriction that SSInClk
    // // >> cannot be faster than 6.67 MHz."
    // // Typical keyboard clock is 10 to 20 kHz
    // // Host must sample data after falling clock edge
    // p.SSI3.cpsr.write(|w| unsafe { w.cpsdvsr().bits(160) });
    // // Configure to receive 11 bits, Freescale (moto) mode SPO=0, SPH=1
    // p.SSI3.cr0.write(|w| {
    //     w.dss()._11();
    //     w.frf().moto();
    //     w.spo().set_bit();
    //     w.sph().set_bit();
    //     w
    // });
    // // Set clock source to sysclk
    // p.SSI3.cc.modify(|_, w| w.cs().syspll());
    // // Enable
    // p.SSI3.cr1.modify(|_, w| w.sse().set_bit());

    unsafe {
        let hw = VideoHardware {
            h_timer: p.TIMER1,
            red_ch: p.SSI0,
            blue_ch: p.SSI1,
            green_ch: p.SSI2
        };
        FRAMEBUFFER.init(hw);
    }

    // Activate UART
    let uart = Serial::uart0(
        p.UART0,
        porta
            .pa1
            .into_af_push_pull::<tm4c123x_hal::gpio::AF1>(&mut porta.control),
        porta
            .pa0
            .into_af_push_pull::<tm4c123x_hal::gpio::AF1>(&mut porta.control),
        (),
        (),
        115200_u32.bps(),
        NewlineMode::SwapLFtoCRLF,
        &clocks,
        &sc.power_control,
    );
    let (mut _tx, rx) = uart.split();

    let keyboard = pc_keyboard::Keyboard::new(pc_keyboard::layouts::Uk105Key);
    let mut c = Context { value: 0, rx, keyboard };

    unsafe {
        FRAMEBUFFER.set_attr(fb::Attr::new(fb::Colour::White, fb::Colour::Black));
        FRAMEBUFFER.clear();
        // Prevent block being removed by the linker
        core::ptr::write_volatile(&mut APPLICATION_RAM[0], 0xAA);
    }

    write!(c, "\u{001b}Z\u{001b}W\u{001b}k╔══════════════════════════════════════════════╗").unwrap();
    write!(c, "║\u{001b}R█████\u{001b}K \u{001b}R\u{001b}y█████\u{001b}K\u{001b}k \u{001b}Y██  █\u{001b}K \u{001b}G█████\u{001b}K \u{001b}G\u{001b}y█\u{001b}k█\u{001b}y█\u{001b}k██\u{001b}K \u{001b}B████\u{001b}K \u{001b}B█████\u{001b}K \u{001b}M██  █\u{001b}W║").unwrap();
    write!(c, "║\u{001b}R▓\u{001b}K \u{001b}R▓\u{001b}K \u{001b}R▓\u{001b}K \u{001b}R\u{001b}y▓\u{001b}K\u{001b}k   \u{001b}R\u{001b}y▓\u{001b}K\u{001b}k \u{001b}Y▓\u{001b}K \u{001b}Y▓ ▓\u{001b}K \u{001b}G▓\u{001b}K   \u{001b}G▓\u{001b}K \u{001b}G \u{001b}K \u{001b}G\u{001b}y▓\u{001b}K\u{001b}k \u{001b}G \u{001b}K \u{001b}B\u{001b}g▓\u{001b}K\u{001b}k  \u{001b}B\u{001b}g▓\u{001b}K\u{001b}k \u{001b}B▓\u{001b}K   \u{001b}B▓\u{001b}K \u{001b}M▓\u{001b}K \u{001b}M▓ ▓\u{001b}W║").unwrap();
    write!(c, "║\u{001b}R▒\u{001b}K \u{001b}R▒\u{001b}K \u{001b}R▒\u{001b}K \u{001b}R\u{001b}y▒\u{001b}K\u{001b}k   \u{001b}R\u{001b}y▒\u{001b}K\u{001b}k \u{001b}Y▒\u{001b}K  \u{001b}Y▒▒\u{001b}K \u{001b}G▒\u{001b}K   \u{001b}G▒\u{001b}K \u{001b}G \u{001b}K \u{001b}G\u{001b}y▒\u{001b}K\u{001b}k \u{001b}G \u{001b}K \u{001b}B\u{001b}g▒\u{001b}K\u{001b}k \u{001b}B\u{001b}g▒\u{001b}k \u{001b}K \u{001b}B▒\u{001b}K   \u{001b}B▒\u{001b}K \u{001b}M▒\u{001b}K \u{001b}M ▒▒\u{001b}W║").unwrap();
    write!(c, "║\u{001b}R░ ░\u{001b}K \u{001b}R░\u{001b}K \u{001b}R\u{001b}y░░░░░\u{001b}K\u{001b}k \u{001b}Y░   ░\u{001b}K \u{001b}G░░░░░\u{001b}K \u{001b}G  \u{001b}y░\u{001b}k  \u{001b}K \u{001b}B\u{001b}g░\u{001b}k  \u{001b}g░\u{001b}K\u{001b}k \u{001b}B░░░░░\u{001b}K \u{001b}M░   ░\u{001b}W║").unwrap();
    write!(c, "╚══════════════════════════════════════════════╝").unwrap();
    writeln!(c, "Monotron v{} ({})", VERSION, GIT_DESCRIBE).unwrap();
    writeln!(c, "Copyright © theJPster 2018").unwrap();

    let (stack_space, data_space) = unsafe {
        extern "C" {
            static __ebss: u32;
            static __sdata: u32;
        }
        let ebss = &__ebss as *const u32 as usize;
        let start = &__sdata as *const u32 as usize;
        let total = ebss - start;
        (32768 - total, APPLICATION_RAM.len())
    };
    writeln!(c, "{} bytes stack, {} bytes free.", stack_space, data_space).unwrap();

    let mut buffer = [0u8; 64];
    let mut r = menu::Runner::new(&ui::ROOT_MENU, &mut buffer, &mut c);

    loop {
        // Wait for new UTF-8 input
        match r.context.read() {
            Some(Input::Unicode(ch)) => {
              let mut char_as_bytes: [u8; 4] = [0u8; 4];
                // Our menu takes UTF-8 chars for serial compatibility,
                // so convert our Unicode to UTF8 bytes
                for octet in ch.encode_utf8(&mut char_as_bytes).bytes() {
                    r.input_byte(octet);
                }
            }
            Some(Input::Utf8(octet)) => {
                // Backspace key in screen seems to generate 0x7F (delete).
                // Map it to backspace (0x08)
                if octet == 0x7F {
                    r.input_byte(0x08);
                } else {
                    r.input_byte(octet);
                }
            }
            Some(Input::Special(code)) => {
                // Can't handle special chars yet
                writeln!(r.context, "Special char {:?}", code).unwrap();
            }
            None => {},
        }
    }
}

impl fb::Hardware for VideoHardware {
    fn configure(&mut self, width: u32, sync_end: u32, line_start: u32, clock_rate: u32) {
        // Configure SPI
        // Need to configure SSI0, SSI1 and SSI2 at 20 MHz
        self.red_ch.cr1.modify(|_, w| w.sse().clear_bit());
        self.blue_ch.cr1.modify(|_, w| w.sse().clear_bit());
        self.green_ch.cr1.modify(|_, w| w.sse().clear_bit());
        // SSIClk = SysClk / (CPSDVSR * (1 + SCR))
        // e.g. 20 MHz = 80 MHz / (4 * (1 + 0))
        // CPSDVSR = 4 -------^
        // SCR = 0 --------------------^
        let ratio = 80_000_000 / clock_rate;
        // For all sensible divisors of 80 MHz, we want SCR = 0.
        self.red_ch.cpsr.write(|w| unsafe { w.cpsdvsr().bits(ratio as u8) });
        self.blue_ch.cpsr.write(|w| unsafe { w.cpsdvsr().bits(ratio as u8) });
        self.green_ch.cpsr.write(|w| unsafe { w.cpsdvsr().bits(ratio as u8) });
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
        self.h_timer
            .tbmatchr
            .modify(|_, w| unsafe { w.bits((multiplier * (width - line_start)) + ISR_LATENCY - 1) });
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
        let gpio = unsafe { &*tm4c123x_hal::tm4c123x::GPIO_PORTC::ptr() };
        unsafe { bb::change_bit(&gpio.data, 4, true) };
    }

    /// Called when V-Sync needs to be low.
    fn vsync_off(&mut self) {
        let gpio = unsafe { &*tm4c123x_hal::tm4c123x::GPIO_PORTC::ptr() };
        unsafe { bb::change_bit(&gpio.data, 4, false) };
    }

    /// Write pixels straight to FIFOs
    fn write_pixels(&mut self, red: u32, green: u32, blue: u32) {
        let ssi_r = unsafe { &*tm4c123x_hal::tm4c123x::SSI0::ptr() };
        let ssi_g = unsafe { &*tm4c123x_hal::tm4c123x::SSI2::ptr() };
        let ssi_b = unsafe { &*tm4c123x_hal::tm4c123x::SSI1::ptr() };
        while (ssi_r.sr.read().bits() & 0x02) == 0 {
        }
        ssi_r.dr.write(|w| unsafe { w.bits(red) });
        ssi_g.dr.write(|w| unsafe { w.bits(green) });
        ssi_b.dr.write(|w| unsafe { w.bits(blue) });
    }
}

interrupt!(GPIOD, keyboard_interrupt);
fn keyboard_interrupt() {
    let gpio = unsafe { &*tm4c123x_hal::tm4c123x::GPIO_PORTD::ptr() };
    // Read PD2
    let bit = bb::read_bit(&gpio.data, 2);
    unsafe {
        let now = FRAMEBUFFER.total_line();
        let gap = (now - KEYBOARD_LAST_BIT_AT) as u32;
        KEYBOARD_LAST_BIT_AT = now;
        if gap > MAX_KEYBOARD_BIT_LINES {
            // Flush buffer
            KEYBOARD_REGISTER = 0;
            KEYBOARD_BITS = 0;
        }
        if bit {
            KEYBOARD_REGISTER |= 1 << KEYBOARD_BITS;
        }
        KEYBOARD_BITS += 1;
        if KEYBOARD_BITS == 11 {
            KEYBOARD_WORDS[KEYBOARD_WRITE_INDEX % KEYBOARD_WORDS.len()] = KEYBOARD_REGISTER;
            KEYBOARD_REGISTER = 0;
            KEYBOARD_WRITE_INDEX = KEYBOARD_WRITE_INDEX.wrapping_add(1);
            KEYBOARD_BITS = 0;
        }
    }
    gpio.icr.write(|w| unsafe { w.gpio().bits(0xFF) });
}

interrupt!(TIMER1A, timer1a);

/// Called on start of sync pulse (end of front porch)
fn timer1a() {
    let ssi_r = unsafe { &*tm4c123x_hal::tm4c123x::SSI0::ptr() };
    let ssi_b = unsafe { &*tm4c123x_hal::tm4c123x::SSI1::ptr() };
    let ssi_g = unsafe { &*tm4c123x_hal::tm4c123x::SSI2::ptr() };
    // Disable SSI0/1/2 as we don't want pixels yet
    ssi_r.cr1.modify(|_, w| w.sse().clear_bit());
    ssi_g.cr1.modify(|_, w| w.sse().clear_bit());
    ssi_b.cr1.modify(|_, w| w.sse().clear_bit());
    // Pre-load red with 2 bytes and green 1 with (they start early so we can line them up)
    ssi_r.dr.write(|w| unsafe { w.data().bits(0) });
    ssi_r.dr.write(|w| unsafe { w.data().bits(0) });
    ssi_g.dr.write(|w| unsafe { w.data().bits(0) });
    // Run the draw routine
    unsafe { FRAMEBUFFER.isr_sol() };
    // Clear timer A interrupt
    let timer = unsafe { &*tm4c123x_hal::tm4c123x::TIMER1::ptr() };
    timer.icr.write(|w| w.caecint().set_bit());
}

interrupt!(TIMER1B, timer1b);

/// Called on start of pixel data (end of back porch)
fn timer1b() {
    unsafe {
    /// Activate the three FIFOs exactly 32 clock cycles (or 8 pixels) apart This
    /// gets the colour video lined up, as we preload the red channel with 0x00
    /// 0x00 and the green channel with 0x00.
        asm!(
            "movs    r0, #132;
            movs    r1, #1;
            movt    r0, #16912;
            mov.w   r2, #262144;
            mov.w   r3, #131072;
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
            : "r0" "r1" "r2"
            : "volatile");
    }
    // Clear timer B interrupt
    let timer = unsafe { &*tm4c123x_hal::tm4c123x::TIMER1::ptr() };
    timer.icr.write(|w| w.cbecint().set_bit());
}


// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

// End of file
