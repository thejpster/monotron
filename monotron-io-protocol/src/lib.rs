//! # monotron-io-protocol
//!
//! Copyright (c) Jonathan 'theJPster' Pallant
//!
//! Licensed under either of
//!
//! - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
//!   http://www.apache.org/licenses/LICENSE-2.0)
//!
//! - MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.
#![no_std]
#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

/// Describes the three LEDs available on a standard IBM PC keyboard
#[derive(Serialize, Deserialize, Debug)]
pub enum KeyboardLed {
    /// The Caps Lock LED
    CapsLock,
    /// The Scroll Lock LED
    ScrollLock,
    /// The Numeric Pad (Numpad) Lock LED
    NumLock,
}

/// Describes the control bits on a parallel port. This mapping matches a
/// standard IBM PC parallel port control register.
///
/// Bit 3 = ~17 / Select-Printer (inverted)
/// Bit 2 = 16 / Reset
/// Bit 1 = ~14 / Linefeed (inverted)
/// Bit 0 = ~1 / Strobe (inverted)
#[derive(Serialize, Deserialize, Debug)]
pub struct ParallelControlBits(u8);

/// Describes the status bits on a parallel port. This mapping matches a
/// standard IBM PC parallel port control register.
///
/// Bit 7 = Pin ~11 / Busy (inverted)
/// Bit 6 = Pin 10 / Ack
/// Bit 5 = Pin 12 / Paper Out
/// Bit 4 = Pin 13 / Select
/// Bit 3 = Pin 15 / Error
#[derive(Serialize, Deserialize, Debug)]
pub struct ParallelStatusBits(u8);

/// A packet of information from the Mouse. Includes movement, button status,
/// etc.
#[derive(Serialize, Deserialize, Debug)]
pub struct MouseInfo {
    /// The status bits (mouse buttons, etc)
    status: u8,
    /// The X position
    x: u8,
    /// The Y position
    y: u8,
}

/// Messages from the main processor to the IO processor
#[derive(Serialize, Deserialize, Debug)]
pub enum McuToIoMessage {
    /// The IO Processor should reboot now
    RebootReq,
    /// The IO Processor should turn on/off an LED on the PS/2 keyboard
    KeyboardLedSetReq(KeyboardLed, bool),
    /// The IO Processor should enable/disable the PS/2 keyboard
    KeyboardEnableReq(bool),
    /// The IO Processor should enable/disable the PS/2 mouse
    MouseEnableReq(bool),
    /// The IO Processor should write these 8 bits to Parallel Port D0..D7
    ParallelSetDataReq(u8),
    /// The IO Processor should write these 6 bits to the Parallel Port control lines
    ParallelSetControlReq(ParallelControlBits),
    /// The IO Processor should should read and return the value of the Paralel Port status lines
    ParallelRxStatusReq,
}

/// Messages from the IO processor to the main processor
#[derive(Serialize, Deserialize, Debug)]
pub enum IoToMcuMessage {
    /// IO processor has received Reboot request and will reboot now
    RebootCfm,
    /// IO processor has understood and actioned KeyboardLedSetReq
    KeyboardLedSetCfm,
    /// IO processor has understood and actioned KeyboardEnableReq
    KeyboardEnableCfm,
    /// IO processor has received byte from the keyboard
    KeyboardByteInd(u8),
    /// IO processor has understood and actioned MouseEnableReq
    MouseEnableCfm,
    /// IO processor has received message from the mouse
    MouseDataInd(MouseInfo),
    /// IO processor has understood and actioned ParallelSetDataReq
    ParallelSetDataCfm,
    /// IO processor has understood and actioned ParallelSetControlReq
    ParallelSetControlCfm,
    /// IO processor has understood and actioned ParallelRxStatusReq; the parameter is the value of the status pins
    ParallelRxStatusCfm(ParallelStatusBits),
}
