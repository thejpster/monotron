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
#![allow(deprecated)]
#![feature(asm)]

// ===========================================================================
// Sub-modules
// ===========================================================================

mod api;
mod ui;

// ===========================================================================
// Imports
// ===========================================================================

extern crate panic_halt;

use cortex_m_rt::{entry, exception};
use fb::AsciiConsole;
use monotron_synth::*;
use tm4c123x_hal as hal;
use vga_framebuffer as fb;

use self::cpu::{interrupt, Interrupt};
use self::hal::bb;
use self::hal::i2c::I2c;
use self::hal::prelude::*;
use self::hal::serial::{NewlineMode, Serial};
use self::hal::sysctl;
use self::hal::tm4c123x as cpu;

// ===========================================================================
// Types
// ===========================================================================

/// Holds a `TimeContextInner` in a mutex.
pub struct TimeContext {
    inner: spin::Mutex<TimeContextInner>,
}

/// Holds all of the system peripheral objects and state.
pub struct Context {
    /// The UART connected to the USB-CDC virtual COM port function on the on-board debugger.
    usb_uart: hal::serial::Serial<
        hal::serial::UART0,
        hal::gpio::gpioa::PA1<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        hal::gpio::gpioa::PA0<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        (),
        (),
    >,
    /// The UART connected to the keyboard / mouse controller chip.
    keyboard_mouse_uart: hal::serial::Serial<
        hal::serial::UART7,
        hal::gpio::gpioe::PE1<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        hal::gpio::gpioe::PE0<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        (),
        (),
    >,
    /// The UART connected to the MIDI interface.
    /// * UART Transmit -> MIDI Out
    /// * MIDI In -> UART Receive + MIDI Through
    midi_uart: hal::serial::Serial<
        hal::serial::UART3,
        hal::gpio::gpioc::PC7<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        hal::gpio::gpioc::PC6<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        (),
        (),
    >,
    /// The UART connected to the RS-232 level shifter
    rs232_uart: hal::serial::Serial<
        hal::serial::UART1,
        hal::gpio::gpiob::PB1<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        hal::gpio::gpiob::PB0<hal::gpio::AlternateFunction<hal::gpio::AF1, hal::gpio::PushPull>>,
        hal::gpio::gpioc::PC4<hal::gpio::AlternateFunction<hal::gpio::AF8, hal::gpio::PushPull>>,
        hal::gpio::gpioc::PC5<hal::gpio::AlternateFunction<hal::gpio::AF8, hal::gpio::PushPull>>,
    >,
    /// Processes scan-codes into key events
    keyboard: pc_keyboard::Keyboard<pc_keyboard::layouts::Uk105Key, pc_keyboard::ScancodeSet2>,
    /// Our I2C bus.
    i2c_bus: I2c<
        cpu::I2C1,
        (
            hal::gpio::gpioa::PA6<
                hal::gpio::AlternateFunction<hal::gpio::AF3, hal::gpio::PushPull>,
            >,
            hal::gpio::gpioa::PA7<
                hal::gpio::AlternateFunction<
                    hal::gpio::AF3,
                    hal::gpio::OpenDrain<hal::gpio::Floating>,
                >,
            >,
        ),
    >,
    /// A single item buffer so that we can 'peek' at the input stream.
    buffered_char: Option<Input>,
    /// Our joystick interface
    joystick: Joystick,
    /// Information about the clock speeds we have configured
    clocks: hal::sysctl::Clocks,
    /// If `false`, input errors are squashed (in case we reboot in the middle
    /// of a message from the keyboard controller). Set to `true` when a valid
    /// message has been received.
    seen_keypress: bool,
}

/// Holds our filesystem and disk controller state.
pub struct DiskContext {
    /// Our SD card controller
    pub cont: embedded_sdmmc::Controller<
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
        &'static TimeContext,
    >,
}

/// Describes the current position of the joystick.
#[derive(Copy, Clone, Debug)]
pub struct JoystickState(u8);

/// Used by our menu runner.
pub struct MenuContext;

/// Tracks the most recent date/time stamp, and the frame count at which we
/// calculated that date/time stamp. Whenever we ask for the time, we move the
/// date/time stamp forwards based on how many frames had elapsed since we
/// last asked.
struct TimeContextInner {
    /// The frame count at which `timestamp` was calculated.
    timestamp_frame_count: u32,
    /// The calendar date / time at `timestamp_frame_count`.
    timestamp: monotron_api::Timestamp,
}

/// Contains the peripherals we need to generate VGA video.
struct VideoHardware {
    /// Used to generate the video signal
    h_timer: cpu::TIMER1,
    /// Used to put the CPU in an idle state just before we draw the video
    /// pixels, to reduce interrupt jitter
    h_timer2: cpu::TIMER2,
    /// Clocks out red pixels
    red_ch: cpu::SSI1,
    /// Clocks out green pixels
    green_ch: cpu::SSI2,
    /// Clocks out blue pixels
    blue_ch: cpu::SSI3,
}

/// Contains the peripherals we need to read an Atari two-axis / one-button
/// joystick.
struct Joystick {
    /// GPIO pin for the up direction
    up: hal::gpio::gpioe::PE2<hal::gpio::Input<hal::gpio::PullUp>>,
    /// GPIO pin for the down direction
    down: hal::gpio::gpioe::PE3<hal::gpio::Input<hal::gpio::PullUp>>,
    /// GPIO pin for the left direction
    left: hal::gpio::gpiod::PD6<hal::gpio::Input<hal::gpio::PullUp>>,
    /// GPIO pin for the right direction
    right: hal::gpio::gpiod::PD7<hal::gpio::Input<hal::gpio::PullUp>>,
    /// GPIO pin for the fire button
    fire: hal::gpio::gpiof::PF4<hal::gpio::Input<hal::gpio::PullUp>>,
}

/// We handle two sorts of input - visible characters which map to CodePage
/// 850, and keys which are special (like 'Page Up').
enum Input {
    /// A special key
    Special(pc_keyboard::KeyCode),
    /// A key which corresponds to a visible character. The byte represents
    /// the CodePage 850 character.
    Cp850(u8),
}

// ===========================================================================
// Constants
// ===========================================================================

/// This is a magic value to make the video timing work.
const ISR_LATENCY: u32 = 24;

/// This is a magic value for a pre-ISR which puts the CPU into a known state
/// before our pixel start ISR.
const ISR_LATENCY_WARMUP: u32 = 3;

/// This is how much RAM we have.
const TOTAL_RAM_LEN: usize = 32768;

/// This is how much RAM we allocate for OS usage (including stack).
const OS_RAM_LEN: usize = 8192;

/// This is where applications start in RAM. Changing this value breaks the
/// ABI.
const APPLICATION_START_ADDR: *mut u8 = (0x20000000 + OS_RAM_LEN) as *mut u8;

/// This is how big applications can be
const APPLICATION_LEN: usize = TOTAL_RAM_LEN - OS_RAM_LEN;

/// Our clock speed in Hz
const CLOCK_SPEED: u32 = 80_000_000;

// ===========================================================================
// Global Variables
// ===========================================================================

/// Stores all the system state (that isn't accessed by interrupts).
/// * We have to use Option<Context> because we can't statically initialise
///   the Context (it needs some single hardware).
/// * We use a spin::Mutex because statics need to be read-only to be
///   shareable, but we do need to mutate the Context.
///
/// Access this object with:
///
/// ```ignore
/// // Lock the mutex
/// let lock = GLOBAL_CONTEXT.lock();
/// // Convert to mutable reference and unwrap the Option
/// let ctx = lock.as_mut().unwrap();
/// ```
pub static GLOBAL_CONTEXT: spin::Mutex<Option<Context>> = spin::Mutex::new(None);

/// Stores all the system state for the filesystem and disk controller.
///
/// We keep this separate to the rest of the system state because any open
/// files hold a reference to this object and it's annoying if you can't read
/// keyboard input while you've got files open.
///
/// * We have to use Option<Context> because we can't statically initialise
///   the Context (it needs some single hardware).
/// * We use a spin::Mutex because statics need to be read-only to be
///   shareable, but we do need to mutate the Context.
///
/// Access this object with:
///
/// ```ignore
/// // Lock the mutex
/// let lock = DISK_CONTEXT.lock();
/// // Convert to mutable reference and unwrap the Option
/// let ctx = lock.as_mut().unwrap();
/// ```
pub static DISK_CONTEXT: spin::Mutex<Option<DiskContext>> = spin::Mutex::new(None);

/// Tracks the current system time in a race-hazard safe way.
pub static TIME_CONTEXT: TimeContext = TimeContext {
    inner: spin::Mutex::new(TimeContextInner {
        timestamp_frame_count: 0,
        timestamp: monotron_api::Timestamp {
            year_from_1970: 0,
            month: 1,
            days: 1,
            hours: 0,
            minutes: 0,
            seconds: 0,
        },
    }),
};

/// This is the version number from Cargo.toml
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// This is the version number generated by git.
static GIT_DESCRIBE: &'static str = env!("GIT_DESCRIBE");

/// This is the wavetable synthesiser. We have 2112 clock cycles per video line.
static mut G_SYNTH: Synth = Synth::new(CLOCK_SPEED / 2112);

/// This is both the video renderer state, and the buffer into which text
/// characters are drawn. These should probably be two separate things.
static mut FRAMEBUFFER: fb::FrameBuffer<VideoHardware> = fb::FrameBuffer::new();

// ===========================================================================
// Macros
// ===========================================================================

/// Prints to the screen
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write as _;
            write!(unsafe { &mut FRAMEBUFFER }, $($arg)*).unwrap();
        }
    };
}

/// Prints to the screen and puts a new-line on the end
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => {
        {
            use core::fmt::Write as _;
            writeln!(unsafe { &mut FRAMEBUFFER }, $($arg)*).unwrap();
        }
    };
}

// ===========================================================================
// Functions and Impls
// ===========================================================================

impl TimeContextInner {
    /// Get the current calendar date/time.
    ///
    /// Grabs the frame count from the video system, and advances the calendar
    /// date/time stored in this object by the appropriate amount before
    /// returning a copy of that date/time.
    pub fn get_timestamp(&mut self) -> monotron_api::Timestamp {
        let num_frames = unsafe { FRAMEBUFFER.frame() } as u32;
        if num_frames != self.timestamp_frame_count {
            let delta = num_frames.wrapping_sub(self.timestamp_frame_count) / 60;
            let days = delta / (3600 * 24);
            let seconds = delta % (3600 * 24);
            self.timestamp.increment(days, seconds);
            self.timestamp_frame_count += delta * 60;
        }
        self.timestamp.clone()
    }
}

impl TimeContext {
    /// Get the current date/time. Uses a spin-lock so do not call concurrently.
    pub fn get_timestamp(&self) -> monotron_api::Timestamp {
        let mut inner = self.inner.lock();
        inner.get_timestamp()
    }

    /// Set the current date/time to the given value and notes the curren
    /// frame count from the video system. Uses a spin-lock so do not call
    /// concurrently.
    pub fn set_timestamp(&self, timestamp: monotron_api::Timestamp) {
        let mut inner = self.inner.lock();
        inner.timestamp = timestamp;
        inner.timestamp_frame_count = unsafe { FRAMEBUFFER.frame() } as u32;
    }
}

impl embedded_sdmmc::TimeSource for &TimeContext {
    /// Supplies a timestamp suitable for use with the filesystem. Uses a
    /// spin-lock so do not call concurrently.
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        let time = self.inner.lock().get_timestamp();
        embedded_sdmmc::Timestamp {
            year_since_1970: time.year_from_1970,
            zero_indexed_month: time.month - 1,
            zero_indexed_day: time.days - 1,
            hours: time.hours,
            minutes: time.minutes,
            seconds: time.seconds,
        }
    }
}

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

    /// Converts the joystick state to an integer, for the C API.
    ///
    /// The bit fields are:
    ///
    /// * UP = 0b10000
    /// * DOWN = 0b01000
    /// * LEFT = 0b00100
    /// * RIGHT = 0b00010
    /// * FIRE = 0b00001
    pub fn as_u8(&self) -> u8 {
        self.0
    }

    /// Returns true if the joystick was in the up position.
    pub fn is_up(&self) -> bool {
        (self.0 & Self::UP) != 0
    }

    /// Returns true if the joystick was in the down position.
    pub fn is_down(&self) -> bool {
        (self.0 & Self::DOWN) != 0
    }

    /// Returns true if the joystick was in the left position.
    pub fn is_left(&self) -> bool {
        (self.0 & Self::LEFT) != 0
    }

    /// Returns true if the joystick was in the right position.
    pub fn is_right(&self) -> bool {
        (self.0 & Self::RIGHT) != 0
    }

    /// Returns true if the joystick had the fire button pressed.
    pub fn fire_pressed(&self) -> bool {
        (self.0 & Self::FIRE) != 0
    }
}

impl Joystick {
    /// Capture the state of the joystick.
    fn get_state(&self) -> JoystickState {
        let is_up = self.up.is_low();
        let is_down = self.down.is_low();
        let is_left = self.left.is_low();
        let is_right = self.right.is_low();
        let is_fire = self.fire.is_low();
        JoystickState::new(is_up, is_down, is_left, is_right, is_fire)
    }
}

impl core::fmt::Write for MenuContext {
    /// The `menu` runner will `write!` to the menu context for output. We
    /// just pass on the output to the screen.
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        unsafe { FRAMEBUFFER.write_str(string) }
    }
}

impl Context {
    /// Is there a character in the input buffer?
    fn has_char(&mut self) -> bool {
        let attempt = self.input_read();
        if attempt.is_some() {
            self.buffered_char = attempt;
            true
        } else {
            false
        }
    }

    /// Read from the input buffer, non-blocking.
    ///
    /// Input can come from the USB UART, or from the Keyboard controller.
    ///
    /// Returns `None` if there's nothing waiting.
    fn input_read(&mut self) -> Option<Input> {
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
            let key = if let Ok(ch) = self.keyboard_mouse_uart.read() {
                // Got something in the buffer from the keyboard/mouse
                // controller.
                match self.keyboard.add_byte(ch) {
                    Ok(Some(event)) => {
                        self.seen_keypress = true;
                        self.keyboard.process_keyevent(event)
                    }
                    Ok(None) => None,
                    Err(e) if self.seen_keypress => {
                        println!("Bad key input! {:?} (0x{:02x})", e, ch);
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

    /// Write an 8-bit ASCII/CodePgae 850 character to the screen.
    fn write_u8(&mut self, ch: u8) {
        unsafe { FRAMEBUFFER.write_character(ch).unwrap() }
    }
}

/// Power on a peripheral and then reset it.
fn enable(p: sysctl::Domain, sc: &mut hal::sysctl::PowerControl) {
    sysctl::control_power(sc, p, sysctl::RunMode::Run, sysctl::PowerState::On);
    sysctl::control_power(sc, p, sysctl::RunMode::Sleep, sysctl::PowerState::On);
    sysctl::reset(sc, p);
}

/// The main routine for our program. Called when the global variable init
/// routines are complete.
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
    nvic.enable(Interrupt::TIMER2A);
    // Make Timer1A (start of line) lower priority than Timer1B (clocking out
    // data) so that it can be interrupted. Timer2A is between the two.
    // Priorities go from 0*16 (most urgent) to 15*16 (least urgent).
    unsafe {
        nvic.set_priority(Interrupt::TIMER1A, 8 * 16);
        nvic.set_priority(Interrupt::TIMER2A, 6 * 16);
        nvic.set_priority(Interrupt::TIMER1B, 4 * 16);
    }

    enable(sysctl::Domain::Timer1, &mut sc.power_control);
    enable(sysctl::Domain::Timer2, &mut sc.power_control);
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
    // Use the HAL driver for SPI We start off slow, but ramp up the speed
    // once init is complete.
    let sdmmc_spi = hal::spi::Spi::spi0(
        p.SSI0,
        (sdmmc_clk, sdmmc_miso, sdmmc_mosi),
        embedded_hal::spi::MODE_0,
        250_000.hz(),
        &clocks,
        &sc.power_control,
    );

    // USB Serial UART
    let usb_uart = Serial::uart0(
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

    // UART for the keyboard/mouse controller.
    let keyboard_mouse_uart = Serial::uart7(
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

    // I²C bus - SDA is open-drain but SCL isn't (see the TM4C123 TRM page 657)
    let i2c_bus = I2c::i2c1(
        p.I2C1,
        (
            porta
                .pa6
                .into_af_push_pull::<hal::gpio::AF3>(&mut porta.control),
            porta
                .pa7
                .into_af_open_drain::<hal::gpio::AF3, hal::gpio::Floating>(&mut porta.control),
        ),
        100_000.hz(),
        &clocks,
        &sc.power_control,
    );

    unsafe {
        let hw = VideoHardware {
            h_timer: p.TIMER1,
            h_timer2: p.TIMER2,
            red_ch: p.SSI1,
            green_ch: p.SSI2,
            blue_ch: p.SSI3,
        };
        FRAMEBUFFER.init(hw);
    }

    // TODO let users pick a keyboard layout, and store their choice in EEPROM somewhere
    let keyboard = pc_keyboard::Keyboard::new(
        pc_keyboard::layouts::Uk105Key,
        pc_keyboard::ScancodeSet2,
        pc_keyboard::HandleControl::MapLettersToUnicode,
    );

    *GLOBAL_CONTEXT.lock() = Some(Context {
        usb_uart,
        keyboard_mouse_uart,
        midi_uart,
        rs232_uart,
        keyboard,
        i2c_bus,
        buffered_char: None,
        joystick: Joystick {
            up: porte.pe2.into_pull_up_input(),
            down: porte.pe3.into_pull_up_input(),
            left: portd.pd6.into_pull_up_input(),
            right: portd.pd7.unlock(&mut portd.control).into_pull_up_input(),
            fire: portf.pf4.into_pull_up_input(),
        },
        clocks,
        seen_keypress: false,
    });

    *DISK_CONTEXT.lock() = Some(DiskContext {
        cont: embedded_sdmmc::Controller::new(
            embedded_sdmmc::SdMmcSpi::new(sdmmc_spi, sdmmc_cs),
            &TIME_CONTEXT,
        ),
    });

    while GLOBAL_CONTEXT
        .lock()
        .as_mut()
        .unwrap()
        .usb_uart
        .read()
        .is_ok()
    {
        // Try again to empty the buffer
    }

    load_time_from_rtc();

    // Print the sign-on banner
    println!("\u{001b}W\u{001b}k\u{001b}Z");
    println!(" \u{001b}R█████\u{001b}K \u{001b}R\u{001b}y█████\u{001b}K\u{001b}k \u{001b}Y██  █\u{001b}K \u{001b}G█████\u{001b}K \u{001b}G\u{001b}y█\u{001b}k█\u{001b}y█\u{001b}k██\u{001b}K \u{001b}B████\u{001b}K \u{001b}B█████\u{001b}K \u{001b}M██  █\u{001b}W");
    println!(" \u{001b}R▓\u{001b}K \u{001b}R▓\u{001b}K \u{001b}R▓\u{001b}K \u{001b}R\u{001b}y▓\u{001b}K\u{001b}k   \u{001b}R\u{001b}y▓\u{001b}K\u{001b}k \u{001b}Y▓\u{001b}K \u{001b}Y▓ ▓\u{001b}K \u{001b}G▓\u{001b}K   \u{001b}G▓\u{001b}K \u{001b}G \u{001b}K \u{001b}G\u{001b}y▓\u{001b}K\u{001b}k \u{001b}G \u{001b}K \u{001b}B\u{001b}g▓\u{001b}K\u{001b}k  \u{001b}B\u{001b}g▓\u{001b}K\u{001b}k \u{001b}B▓\u{001b}K   \u{001b}B▓\u{001b}K \u{001b}M▓\u{001b}K \u{001b}M▓ ▓\u{001b}W");
    println!(" \u{001b}R▒\u{001b}K \u{001b}R▒\u{001b}K \u{001b}R▒\u{001b}K \u{001b}R\u{001b}y▒\u{001b}K\u{001b}k   \u{001b}R\u{001b}y▒\u{001b}K\u{001b}k \u{001b}Y▒\u{001b}K  \u{001b}Y▒▒\u{001b}K \u{001b}G▒\u{001b}K   \u{001b}G▒\u{001b}K \u{001b}G \u{001b}K \u{001b}G\u{001b}y▒\u{001b}K\u{001b}k \u{001b}G \u{001b}K \u{001b}B\u{001b}g▒\u{001b}K\u{001b}k \u{001b}B\u{001b}g▒\u{001b}k \u{001b}K \u{001b}B▒\u{001b}K   \u{001b}B▒\u{001b}K \u{001b}M▒\u{001b}K \u{001b}M ▒▒\u{001b}W");
    println!(" \u{001b}R░ ░\u{001b}K \u{001b}R░\u{001b}K \u{001b}R\u{001b}y░░░░░\u{001b}K\u{001b}k \u{001b}Y░   ░\u{001b}K \u{001b}G░░░░░\u{001b}K \u{001b}G  \u{001b}y░\u{001b}k  \u{001b}K \u{001b}B\u{001b}g░\u{001b}k  \u{001b}g░\u{001b}K\u{001b}k \u{001b}B░░░░░\u{001b}K \u{001b}M░   ░\u{001b}W");
    println!("* Monotron v{}", VERSION);
    println!("* {}", GIT_DESCRIBE);
    println!("* Copyright © theJPster 2019");
    println!("* https://github.com/thejpster/monotron");

    // Calculates the size of the stack by looking at the size of the data and
    // bss segments. Assumes bss comes after data.
    let stack_space = unsafe {
        extern "C" {
            static __ebss: u32;
            static __sdata: u32;
        }
        let ebss = &__ebss as *const u32 as usize;
        let sdata = &__sdata as *const u32 as usize;
        let total = ebss - sdata;
        8192 - total
    };
    println!(
        "{} bytes stack, {} bytes free.",
        stack_space, APPLICATION_LEN
    );

    // Set up our menu system.
    let mut buffer = [0u8; 64];
    let mut r = menu::Runner::new(&ui::ROOT_MENU, &mut buffer, MenuContext);

    loop {
        // Wait For Vertical Blanking Interval
        api::wfvbi();
        // Grab the lock, convert to mutable reference and unwrap the
        // Option<>, then grab any new input
        let input = GLOBAL_CONTEXT.lock().as_mut().unwrap().input_read();
        // Now we do the match having released the lock
        match input {
            Some(Input::Cp850(octet)) => {
                // Feed the menu system. It expects UTF-8 but it's happy with
                // CP850 as long as all our commands are ASCII.
                r.input_byte(octet);
            }
            Some(Input::Special(code)) => {
                // Can't handle special chars yet.
                println!("\rSpecial char {:?}", code);
                r.prompt(false);
            }
            None => {}
        }
    }
}

/// Makes it possible to share an I2C bus with a driver that wants to own the
/// bus.
struct I2cBus<'a, T>(&'a mut T)
where
    T: embedded_hal::blocking::i2c::Write + embedded_hal::blocking::i2c::WriteRead;

impl<'a, T> embedded_hal::blocking::i2c::Write for I2cBus<'a, T>
where
    T: embedded_hal::blocking::i2c::Write + embedded_hal::blocking::i2c::WriteRead,
{
    type Error = <T as embedded_hal::blocking::i2c::Write>::Error;
    fn write(&mut self, register: u8, buffer: &[u8]) -> Result<(), Self::Error> {
        self.0.write(register, buffer)
    }
}

impl<'a, T> embedded_hal::blocking::i2c::WriteRead for I2cBus<'a, T>
where
    T: embedded_hal::blocking::i2c::Write + embedded_hal::blocking::i2c::WriteRead,
{
    type Error = <T as embedded_hal::blocking::i2c::WriteRead>::Error;

    fn write_read(
        &mut self,
        register: u8,
        out_buffer: &[u8],
        in_buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.0.write_read(register, out_buffer, in_buffer)
    }
}

impl fb::Hardware for VideoHardware {
    /// Set up the SPI peripherals to clock out RGB video with the given timings.
    ///
    /// * `width` - length of a line (in pixels)
    /// * `sync_end` - elapsed time (in pixels) before H-Sync needs to fall
    ///   (it starts at the beginning of the line).
    /// * `line_start` - elapsed time (in pixels) before line_start ISR needs
    ///   to fire
    /// * `pixel_clock` - the pixel clock rate in Hz (e.g. 40_000_000 for 40
    ///   MHz)
    fn configure(&mut self, width: u32, sync_end: u32, line_start: u32, pixel_clock: u32) {
        // Need to configure SSI1, SSI2 and SSI3 at `pixel_clock` Hz.
        // First up, we disable all three.
        self.red_ch.cr1.modify(|_, w| w.sse().clear_bit());
        self.blue_ch.cr1.modify(|_, w| w.sse().clear_bit());
        self.green_ch.cr1.modify(|_, w| w.sse().clear_bit());
        // SSIClk = SysClk / (CPSDVSR * (1 + SCR))
        // e.g. 20 MHz = 80 MHz / (4 * (1 + 0))
        // CPSDVSR = 4 ------------^
        // SCR = 0 -------------------------^
        let ratio = CLOCK_SPEED / pixel_clock;
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
        // Each channel needs to clock out 8 bit words, with the correct
        // phase/polarity.
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
        // Timer runs in dual-16-bit mode.
        // Timer A is periodic with PWM enabled.
        self.h_timer.cfg.modify(|_, w| w.cfg()._16_bit());
        self.h_timer.tamr.modify(|_, w| {
            w.taams().set_bit();
            w.tacmr().clear_bit();
            w.tapwmie().set_bit();
            w.tamr().period();
            w
        });
        // Timer B is periodic with PWM enabled.
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
        let convert_to_clockset = |i: u32| -> u32 { (ratio * i) - 1 };
        self.h_timer
            .tailr
            .modify(|_, w| unsafe { w.bits(convert_to_clockset(width)) });
        self.h_timer
            .tbilr
            .modify(|_, w| unsafe { w.bits(convert_to_clockset(width)) });
        self.h_timer
            .tamatchr
            .modify(|_, w| unsafe { w.bits(convert_to_clockset(width - sync_end)) });
        // Counting down, so adding here makes it earlier
        self.h_timer.tbmatchr.modify(|_, w| unsafe {
            w.bits(convert_to_clockset(ISR_LATENCY + width - line_start))
        });
        self.h_timer.imr.modify(|_, w| {
            w.caeim().set_bit(); // Timer1A fires at start of line
            w.cbeim().set_bit(); // Timer1B fires at start of data
            w
        });

        // Configure Timer2A to run just before Timer 1B
        self.h_timer2.ctl.modify(|_, w| {
            w.taen().clear_bit();
            w.tben().clear_bit();
            w
        });
        self.h_timer2.cfg.modify(|_, w| w.cfg()._16_bit());
        self.h_timer2.tamr.modify(|_, w| {
            w.taams().set_bit();
            w.tacmr().clear_bit();
            w.tapwmie().set_bit();
            w.tamr().period();
            w
        });
        self.h_timer2.ctl.modify(|_, w| {
            // Trigger Timer A capture on falling edge (i.e. just before data start)
            w.tapwml().set_bit();
            w
        });
        // We're counting down in PWM mode, so start at the end
        // We start a few pixels before Timer1B
        self.h_timer2
            .tailr
            .modify(|_, w| unsafe { w.bits(convert_to_clockset(width)) });
        // Counting down, so adding here makes it earlier
        self.h_timer2.tamatchr.modify(|_, w| unsafe {
            w.bits(convert_to_clockset(
                ISR_LATENCY + ISR_LATENCY_WARMUP + width - line_start,
            ))
        });
        self.h_timer2.imr.modify(|_, w| {
            w.caeim().set_bit(); // Timer1A fires just before at start of data
            w
        });

        // Clear interrupts
        self.h_timer.icr.write(|w| {
            w.tbmcint().set_bit();
            w.tbtocint().set_bit();
            w
        });

        // Clear interrupts
        self.h_timer2.icr.write(|w| {
            w.tbmcint().set_bit();
            w.tbtocint().set_bit();
            w
        });

        self.h_timer2.ctl.modify(|_, w| {
            w.taen().set_bit();
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
    fn write_pixels(&mut self, xrgb: vga_framebuffer::XRGBColour) {
        let ssi_r = unsafe { &*cpu::SSI1::ptr() };
        let ssi_g = unsafe { &*cpu::SSI2::ptr() };
        let ssi_b = unsafe { &*cpu::SSI3::ptr() };
        while (ssi_r.sr.read().bits() & 0x02) == 0 {}
        ssi_r.dr.write(|w| unsafe { w.bits(xrgb.red()) });
        ssi_g.dr.write(|w| unsafe { w.bits(xrgb.green()) });
        ssi_b.dr.write(|w| unsafe { w.bits(xrgb.blue()) });
    }
}

fn load_time_from_rtc() {
    use mcp794xx::Rtcc;
    // Grab the lock
    let mut lock = GLOBAL_CONTEXT.lock();
    // Convert to mutable reference and unwrap the Option<>
    let ctx = lock.as_mut().unwrap();
    let bus = I2cBus(&mut ctx.i2c_bus);
    let mut rtc = mcp794xx::Mcp794xx::new_mcp7940n(bus);
    let dt = match rtc.get_datetime() {
        Ok(dt) => dt,
        Err(e) => {
            drop(rtc);
            drop(ctx);
            drop(lock);
            println!("Error reading RTC: {:?}", e);
            return;
        }
    };
    let timestamp = monotron_api::Timestamp {
        year_from_1970: (dt.year - 1970) as u8,
        month: dt.month,
        days: dt.day,
        hours: match dt.hour {
            mcp794xx::Hours::H24(n) => n,
            mcp794xx::Hours::AM(n) => n,
            mcp794xx::Hours::PM(n) => n + 12,
        },
        minutes: dt.minute,
        seconds: dt.second,
    };
    TIME_CONTEXT.set_timestamp(timestamp);
}

// ===========================================================================
// Interrupts
// ===========================================================================

interrupt!(TIMER2A, timer2a);

/// Called just before Timer1B, which gives Timer1B lower interrupt jitter.
fn timer2a() {
    unsafe {
        asm!("wfi");
        let timer = &*cpu::TIMER2::ptr();
        timer.icr.write(|w| w.caecint().set_bit());
    }
}

interrupt!(TIMER1A, timer1a);

/// Called on start of sync pulse (end of front porch). This is unsafe because
/// we mutate statics while the main thread might be using them at the same
/// time (technically this is undefined behaviour).
fn timer1a() {
    unsafe {
        let pwm = &*cpu::PWM0::ptr();
        let ssi_r = &*cpu::SSI1::ptr();
        let ssi_g = &*cpu::SSI2::ptr();
        let ssi_b = &*cpu::SSI3::ptr();
        static mut NEXT_SAMPLE: u8 = 128;
        // Play the previously calculated and buffered audio sample. We play
        // it here as the video generation is variable-length, and we don't
        // want audio jitter.
        pwm._2_cmpa.write(|w| w.compa().bits(NEXT_SAMPLE as u16));
        // Disable the SPIs as we don't want pixels yet
        ssi_r.cr1.modify(|_, w| w.sse().clear_bit());
        ssi_g.cr1.modify(|_, w| w.sse().clear_bit());
        ssi_b.cr1.modify(|_, w| w.sse().clear_bit());
        // Pre-load red with 2 bytes and green 1 with (they start early so we can line them up)
        ssi_r.dr.write(|w| w.data().bits(0));
        ssi_r.dr.write(|w| w.data().bits(0));
        ssi_g.dr.write(|w| w.data().bits(0));
        // Run the draw routine
        FRAMEBUFFER.isr_sol();
        // Run the audio routine
        NEXT_SAMPLE = G_SYNTH.next().into();
        // Clear timer A interrupt
        let timer = &*cpu::TIMER1::ptr();
        timer.icr.write(|w| w.caecint().set_bit());
    }
}

interrupt!(TIMER1B, timer1b);

/// Called on start of pixel data (end of back porch)
fn timer1b() {
    // Activate the three FIFOs exactly 32 clock cycles (or 8 pixels) apart This
    // gets the colour video lined up, as we preload the red channel with 0x00
    // 0x00 and the green channel with 0x00.
    unsafe {
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
        // Clear timer B interrupt
        let timer = &*cpu::TIMER1::ptr();
        timer.icr.write(|w| w.cbecint().set_bit());
    }
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

// ===========================================================================
// End of file
// ===========================================================================
