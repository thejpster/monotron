//! # Monotron
//!
//! > A simple 1980's home computer style application for the Stellaris Launchpad
//!
//! See README.md for more details.

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate embedded_hal;
extern crate menu;
extern crate tm4c123x_hal;
extern crate vga_framebuffer as fb;

use core::fmt::Write;
use cortex_m::asm;
use embedded_hal::prelude::*;
use tm4c123x_hal::bb;
use tm4c123x_hal::gpio::GpioExt;
use tm4c123x_hal::serial::{NewlineMode, Serial};
use tm4c123x_hal::sysctl::{self, SysctlExt};
use tm4c123x_hal::time::U32Ext;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const GIT_DESCRIBE: &'static str = env!("GIT_DESCRIBE");
const ISR_LATENCY: u32 = 35;

type Menu<'a> = menu::Menu<'a, Context>;
type Item<'a> = menu::Item<'a, Context>;

const TEST_ALPHABET: Item = Item {
    item_type: menu::ItemType::Callback(test_alphabet),
    command: "alphabet",
    help: Some("Scrolls some test text output forever (reboot to exit)."),
};

const TEST_ANIMATION: Item = Item {
    item_type: menu::ItemType::Callback(test_animation),
    command: "animate",
    help: Some("Bounces some text around (reboot to exit)."),
};

const ROOT_MENU: Menu = Menu {
    label: "root",
    items: &[&TEST_ALPHABET, &TEST_ANIMATION],
    entry: None,
    exit: None,
};

static mut FRAMEBUFFER: fb::FrameBuffer<&'static mut Hardware> = fb::FrameBuffer::new();

struct Hardware {
    h_timer: Option<tm4c123x_hal::tm4c123x::TIMER0>,
}

struct Context {
    pub value: u32,
}

impl core::fmt::Write for Context {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            FRAMEBUFFER.write_str(s)
        }
    }
}

static mut HARDWARE: Hardware = Hardware { h_timer: None };

fn enable(p: sysctl::Domain, sc: &mut tm4c123x_hal::sysctl::PowerControl) {
    sysctl::control_power(sc, p, sysctl::RunMode::Run, sysctl::PowerState::On);
    sysctl::control_power(sc, p, sysctl::RunMode::Sleep, sysctl::PowerState::On);
    sysctl::reset(sc, p);
}

fn main() {
    let p = tm4c123x_hal::Peripherals::take().unwrap();
    let cp = tm4c123x_hal::CorePeripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = sysctl::Oscillator::Main(
        sysctl::CrystalFrequency::_16mhz,
        sysctl::SystemClock::UsePll(sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sc.clock_setup.freeze();

    let mut nvic = cp.NVIC;
    nvic.enable(tm4c123x_hal::Interrupt::TIMER0A);
    nvic.enable(tm4c123x_hal::Interrupt::TIMER0B);
    // Make Timer0A (start of line) lower priority than Timer0B (clocking out
    // data) so that it can be interrupted.
    unsafe { nvic.set_priority(tm4c123x_hal::Interrupt::TIMER0A, 32); }

    enable(sysctl::Domain::Timer0, &mut sc.power_control);
    // enable(sysctl::Domain::MicroDma, &mut sc.power_control);
    enable(sysctl::Domain::Ssi2, &mut sc.power_control);

    let mut portb = p.GPIO_PORTB.split(&sc.power_control);
    let portc = p.GPIO_PORTC.split(&sc.power_control);
    // T0CCP0
    let _h_sync = portb.pb6.into_af7(&mut portb.control);
    // GPIO controlled V-Sync
    let _v_sync = portc.pc4.into_push_pull_output();
    // Ssi2Tx
    let _green_data = portb.pb7.into_af2(&mut portb.control);

    // Need to configure SSI2 at 20 MHz
    p.SSI2.cr1.modify(|_, w| w.sse().clear_bit());
    // SSIClk = SysClk / (CPSDVSR * (1 + SCR))
    // 20 MHz = 80 MHz / (4 * (1 + 0))
    // CPSDVSR = 4 -------^
    // SCR = 0 --------------------^
    p.SSI2.cpsr.write(|w| unsafe { w.cpsdvsr().bits(4) });
    p.SSI2.cr0.write(|w| {
        w.dss()._16();
        w.frf().moto();
        w.spo().clear_bit();
        w.sph().set_bit();
        w
    });
    // Enable TX DMA
    // p.SSI2.dmactl.write(|w| w.txdmae().set_bit());
    // Set clock source to sysclk
    p.SSI2.cc.modify(|_, w| w.cs().syspll());

    // Need to configure MicroDMA to feed SSI2 with data
    // That's Encoder 2, Channel 13
    // let dma = p.UDMA;
    // dma.cfg.write(|w| w.masten().set_bit());
    // dma.ctlbase
    //     .write(|w| unsafe { w.addr().bits(&mut DMA_CONTROL_TABLE as *mut DmaInfo as u32) });

    unsafe {
        HARDWARE.h_timer = Some(p.TIMER0);
        FRAMEBUFFER.init(&mut HARDWARE);
    }

    let mut c = Context {
        value: 0,
    };

    unsafe { FRAMEBUFFER.clear(); }
    writeln!(c, "Monotron v{} ({})", VERSION, GIT_DESCRIBE).unwrap();

    let mut porta = p.GPIO_PORTA.split(&sc.power_control);

    // Activate UART
    let uart = Serial::uart0(
        p.UART0,
        porta.pa1.into_af1(&mut porta.control),
        porta.pa0.into_af1(&mut porta.control),
        (),
        (),
        115200_u32.bps(),
        NewlineMode::SwapLFtoCRLF,
        &clocks,
        &sc.power_control,
    );
    let (mut _tx, mut rx) = uart.split();

    let mut buffer = [0u8; 64];
    let mut r = menu::Runner::new(&ROOT_MENU, &mut buffer, &mut c);

    loop {
        // Wait for char
        if let Ok(ch) = rx.read() {
            r.output.write_char(ch as char).unwrap();
            // Feed char to runner
            r.input_byte(ch);
        }
    }
}

impl fb::Hardware for &'static mut Hardware {
    fn configure(&mut self, width: u32, sync_end: u32, line_start: u32, _clock_rate: u32) {
        if let Some(ref h_timer) = self.h_timer {
            // Configure Timer0A for h-sync and Timer0B for line trigger
            h_timer.ctl.modify(|_, w| {
                w.taen().clear_bit();
                w.tben().clear_bit();
                w
            });
            h_timer.cfg.modify(|_, w| w.cfg()._16_bit());
            h_timer.tamr.modify(|_, w| {
                w.taams().set_bit();
                w.tacmr().clear_bit();
                w.tapwmie().set_bit();
                w.tamr().period();
                w
            });
            h_timer.tbmr.modify(|_, w| {
                w.tbams().set_bit();
                w.tbcmr().clear_bit();
                w.tbmr().period();
                w.tbpwmie().set_bit();
                w
            });
            h_timer.ctl.modify(|_, w| {
                // Trigger Timer A capture on rising edge (i.e. line start)
                w.tapwml().clear_bit();
                // Trigger Timer B capture on falling edge (i.e. data start)
                w.tbpwml().set_bit();
                w
            });
            // We're counting down in PWM mode, so start at the end
            h_timer
                .tailr
                .modify(|_, w| unsafe { w.bits(width * 2 - 1) });
            h_timer
                .tbilr
                .modify(|_, w| unsafe { w.bits(width * 2 - 1) });
            h_timer
                .tamatchr
                .modify(|_, w| unsafe { w.bits(2 * (width - sync_end) - 1) });
            h_timer
                .tbmatchr
                .modify(|_, w| unsafe { w.bits(2 * (width - line_start) + ISR_LATENCY - 1) });
            h_timer.imr.modify(|_, w| {
                w.caeim().set_bit(); // Timer0A fires at start of line
                w.cbeim().set_bit(); // Timer0B fires at start of data
                w
            });

            // Clear interrupts
            h_timer.icr.write(|w| {
                w.tbmcint().set_bit();
                w.tbtocint().set_bit();
                w
            });

            h_timer.ctl.modify(|_, w| {
                w.taen().set_bit();
                w.tben().set_bit();
                w
            });
        }
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

    /// Write pixels straight to FIFO
    fn write_pixels(&mut self, word: u16) {
        let ssi = unsafe { &*tm4c123x_hal::tm4c123x::SSI2::ptr() };
        while ssi.sr.read().tnf().bit_is_clear() {
            asm::nop();
        }
        ssi.dr.write(|w| unsafe { w.data().bits(word) });
    }
}

/// The test menu item - displays a static bitmap.
fn test_alphabet<'a>(_menu: &Menu, _item: &Item, _input: &str, _context: &mut Context) {
    unsafe {
        FRAMEBUFFER.clear();
        FRAMEBUFFER.goto(0, 0).unwrap();
    }
    let mut old_frame = 0;
    loop {
        asm::wfi();
        let new_frame = unsafe { FRAMEBUFFER.frame() };
        if new_frame != old_frame {
            old_frame = new_frame;
            unsafe {
                write!(FRAMEBUFFER, "0123456789!\"£€$%^&*()_+-=;:'@~#[]{{}}").unwrap();
                write!(FRAMEBUFFER, "abcdefghijklmnopqrstuvwxyz").unwrap();
                write!(FRAMEBUFFER, "ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();
            }
        }
    }
}

/// Another test menu item - displays an animation.
fn test_animation<'a>(_menu: &Menu, _item: &Item, _input: &str, _context: &mut Context) {
    let mut old_frame = 0;
    let mut row = 0;
    let mut col = 0;
    let mut left = true;
    let mut down = true;
    loop {
        asm::wfi();
        let new_frame = unsafe { FRAMEBUFFER.frame() };
        if new_frame != old_frame {
            old_frame = new_frame;
            if left {
                col = col + 1;
            } else {
                col = col - 1;
            }
            if down {
                row = row + 1
            } else {
                row = row - 1;
            }
            if col == 0 {
                left = true;
            }
            if (col + 15) == fb::TEXT_MAX_COL {
                left = false;
            }
            if row == 0 {
                down = true;
            }
            if row == fb::TEXT_MAX_ROW {
                down = false;
            }
            unsafe {
                FRAMEBUFFER.clear();
                FRAMEBUFFER.goto(row, col).unwrap();
                write!(FRAMEBUFFER, "Frame: {:08}", new_frame).unwrap();
            }
        }
    }
}

extern "C" fn timer0a_isr() {
    let ssi = unsafe { &*tm4c123x_hal::tm4c123x::SSI2::ptr() };
    // Disable SSI2 as we don't want pixels yet
    ssi.cr1.modify(|_, w| w.sse().clear_bit());
    unsafe { FRAMEBUFFER.isr_sol() };
    let timer = unsafe { &*tm4c123x_hal::tm4c123x::TIMER0::ptr() };
    timer.icr.write(|w| w.caecint().set_bit());
}

extern "C" fn timer0b_isr() {
    let ssi = unsafe { &*tm4c123x_hal::tm4c123x::SSI2::ptr() };
    // Enable SSI2 to let buffered pixels flow. We're still calculating the
    // end of the line but the SPI FIFO gets us out of trouble
    ssi.cr1.modify(|_, w| w.sse().set_bit());
    let timer = unsafe { &*tm4c123x_hal::tm4c123x::TIMER0::ptr() };
    timer.icr.write(|w| w.cbecint().set_bit());
}

extern "C" fn default_handler() {
    asm::bkpt();
}

#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [Option<extern "C" fn()>; 139] = [
    // GPIO Port A                      16
    Some(default_handler),
    // GPIO Port B                      17
    Some(default_handler),
    // GPIO Port C                      18
    Some(default_handler),
    // GPIO Port D                      19
    Some(default_handler),
    // GPIO Port E                      20
    Some(default_handler),
    // UART 0                           21
    Some(default_handler),
    // UART 1                           22
    Some(default_handler),
    // SSI 0                            23
    Some(default_handler),
    // I2C 0                            24
    Some(default_handler),
    // Reserved                         25
    None,
    // Reserved                         26
    None,
    // Reserved                         27
    None,
    // Reserved                         28
    None,
    // Reserved                         29
    None,
    // ADC 0 Seq 0                      30
    Some(default_handler),
    // ADC 0 Seq 1                      31
    Some(default_handler),
    // ADC 0 Seq 2                      32
    Some(default_handler),
    // ADC 0 Seq 3                      33
    Some(default_handler),
    // WDT 0 and 1                      34
    Some(default_handler),
    // 16/32 bit timer 0 A              35
    Some(timer0a_isr),
    // 16/32 bit timer 0 B              36
    Some(timer0b_isr),
    // 16/32 bit timer 1 A              37
    Some(default_handler),
    // 16/32 bit timer 1 B              38
    Some(default_handler),
    // 16/32 bit timer 2 A              39
    Some(default_handler),
    // 16/32 bit timer 2 B              40
    Some(default_handler),
    // Analog comparator 0              41
    Some(default_handler),
    // Analog comparator 1              42
    Some(default_handler),
    // Reserved                         43
    None,
    // System control                   44
    Some(default_handler),
    // Flash + EEPROM control           45
    Some(default_handler),
    // GPIO Port F                      46
    Some(default_handler),
    // Reserved                         47
    None,
    // Reserved                         48
    None,
    // UART 2                           49
    Some(default_handler),
    // SSI 1                            50
    Some(default_handler),
    // 16/32 bit timer 3 A              51
    Some(default_handler),
    // 16/32 bit timer 3 B              52
    Some(default_handler),
    // I2C 1                            53
    Some(default_handler),
    // Reserved                         54
    None,
    // CAN 0                            55
    Some(default_handler),
    // Reserved                         56
    None,
    // Reserved                         57
    None,
    // Reserved                         58
    None,
    // Hibernation module               59
    Some(default_handler),
    // USB                              60
    Some(default_handler),
    // Reserved                         61
    None,
    // UDMA SW                          62
    Some(default_handler),
    // UDMA Error                       63
    Some(default_handler),
    // ADC 1 Seq 0                      64
    Some(default_handler),
    // ADC 1 Seq 1                      65
    Some(default_handler),
    // ADC 1 Seq 2                      66
    Some(default_handler),
    // ADC 1 Seq 3                      67
    Some(default_handler),
    // Reserved                         68
    None,
    // Reserved                         69
    None,
    // Reserved                         70
    None,
    // Reserved                         71
    None,
    // Reserved                         72
    None,
    // SSI 2                            73
    Some(default_handler),
    // SSI 2                            74
    Some(default_handler),
    // UART 3                           75
    Some(default_handler),
    // UART 4                           76
    Some(default_handler),
    // UART 5                           77
    Some(default_handler),
    // UART 6                           78
    Some(default_handler),
    // UART 7                           79
    Some(default_handler),
    // Reserved                         80
    None,
    // Reserved                         81
    None,
    // Reserved                         82
    None,
    // Reserved                         83
    None,
    // I2C 2                            84
    Some(default_handler),
    // I2C 4                            85
    Some(default_handler),
    // 16/32 bit timer 4 A              86
    Some(default_handler),
    // 16/32 bit timer 4 B              87
    Some(default_handler),
    // Reserved                         88
    None,
    // Reserved                         89
    None,
    // Reserved                         90
    None,
    // Reserved                         91
    None,
    // Reserved                         92
    None,
    // Reserved                         93
    None,
    // Reserved                         94
    None,
    // Reserved                         95
    None,
    // Reserved                         96
    None,
    // Reserved                         97
    None,
    // Reserved                         98
    None,
    // Reserved                         99
    None,
    // Reserved                         100
    None,
    // Reserved                         101
    None,
    // Reserved                         102
    None,
    // Reserved                         103
    None,
    // Reserved                         104
    None,
    // Reserved                         105
    None,
    // Reserved                         106
    None,
    // Reserved                         107
    None,
    // 16/32 bit timer 5 A              108
    Some(default_handler),
    // 16/32 bit timer 5 B              109
    Some(default_handler),
    // 32/64 bit timer 0 A              110
    Some(default_handler),
    // 32/64 bit timer 0 B              111
    Some(default_handler),
    // 32/64 bit timer 1 A              112
    Some(default_handler),
    // 32/64 bit timer 1 B              113
    Some(default_handler),
    // 32/64 bit timer 2 A              114
    Some(default_handler),
    // 32/64 bit timer 2 B              115
    Some(default_handler),
    // 32/64 bit timer 3 A              116
    Some(default_handler),
    // 32/64 bit timer 3 B              117
    Some(default_handler),
    // 32/64 bit timer 4 A              118
    Some(default_handler),
    // 32/64 bit timer 4 B              119
    Some(default_handler),
    // 32/64 bit timer 5 A              120
    Some(default_handler),
    // 32/64 bit timer 5 B              121
    Some(default_handler),
    // System Exception                 122
    Some(default_handler),
    // Reserved                         123
    None,
    // Reserved                         124
    None,
    // Reserved                         125
    None,
    // Reserved                         126
    None,
    // Reserved                         127
    None,
    // Reserved                         128
    None,
    // Reserved                         129
    None,
    // Reserved                         130
    None,
    // Reserved                         131
    None,
    // Reserved                         132
    None,
    // Reserved                         133
    None,
    // Reserved                         134
    None,
    // Reserved                         135
    None,
    // Reserved                         136
    None,
    // Reserved                         137
    None,
    // Reserved                         138
    None,
    // Reserved                         139
    None,
    // Reserved                         140
    None,
    // Reserved                         141
    None,
    // Reserved                         142
    None,
    // Reserved                         143
    None,
    // Reserved                         144
    None,
    // Reserved                         145
    None,
    // Reserved                         146
    None,
    // Reserved                         147
    None,
    // Reserved                         148
    None,
    // Reserved                         149
    None,
    // Reserved                         150
    None,
    // Reserved                         151
    None,
    // Reserved                         152
    None,
    // Reserved                         153
    None,
    // Reserved                         154
    None,
];
