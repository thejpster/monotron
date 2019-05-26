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

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum KeyboardLed {
	CapsLock,
	ScrollLock,
	NumLock
}

/// Messages from the main processor to the IO processor
#[derive(Serialize, Deserialize, Debug)]
pub enum McuToIoMessage {
	RebootReq,
	KeyboardLedSetReq(KeyboardLed, bool),
	KeyboardEnableReq(bool),
	MouseEnableReq(bool),
	LptSetDataReq(u8),
	LptSetControlReq(u8),
	LptRxStatusReq,
}

/// Messages from the IO processor to the main processor
#[derive(Serialize, Deserialize, Debug)]
pub enum IoToMcuMessage {
	RebootCfm,
	KeyboardLedSetCfm,
	KeyboardEnableCfm,
	KeyboardByteInd(u8),
	MouseEnableCfm,
	MouseDataInd {
		status: u8,
		x: u8,
		y: u8
	},
	LptSetDataCfm,
	LptSetControlCfm,
	LptRxStatusCfm(u8),
}
