# Monotron I/O Controller

## Introduction

This is the firmware for the Monotron's I/O controller. It is an Atmel AVR
Atmega48, and it controls the:

* PS/2 Keyboard port
* PS/2 Mouse port
* PC Printer (aka LPT, or Centronics) port

It talks to the main Monotron MCU over a serial link at 115,200 bps (8-data
bits, no parity, 1 stop bit).

## Compilation

This firmware builds with `avr-gcc` and `meson`.

```shell
$ meson --cross-file=./avr-cross.txt ./build
$ cd ./build
$ ninja # Builds a debug ELF and .hex file
$ ninja size # Tells you how big it is
```

## Pinout

| Port   | Pin |  Direction     | Name          | Routed to  | Description              |
|--------|-----|----------------|------------- -|------------|--------------------------|
| B      | 0   | Output         | LPT_D0        | J13 pin 2  | LPT Output bit 0         |
| B      | 1   | Output         | LPT_D1        | J13 pin 3  | LPT Output bit 1         |
| B      | 2   | Output         | LPT_D2        | J13 pin 4  | LPT Output bit 2         |
| B      | 3   | Output         | LPT_D3        | J13 pin 5  | LPT Output bit 3         |
| B      | 4   | Output         | LPT_D4        | J13 pin 6  | LPT Output bit 4         |
| B      | 5   | Output         | LPT_D5        | J13 pin 7  | LPT Output bit 5         |
| B      | 6   | Output         | LPT_D6        | J13 pin 8  | LPT Output bit 6         |
| B      | 7   | Output         | LPT_D7        | J13 pin 9  | LPT Output bit 7         |
| C      | 0   | I/O Open-drain | KB_CLK        | J14 pin 5  | PS/2 Keyboard clock      |
| C      | 1   | I/O Open-drain | MS_CLK        | J15 pin 5  | PS/2 Mouse clock         |
| C      | 2   | I/O Open-drain | KB_DATA       | J14 pin 1  | PS/2 Keyboard data       |
| C      | 3   | I/O Open-drain | MS_DATA       | J15 pin 1  | PS/2 Mouse data          |
| C      | 4   | Output         | LPT_nINIT     | J13 pin 16 | Initialise Printer       |
| C      | 5   | Input          | LPT_SEL       | J13 pin 13 | Select (from printer)    |
| C      | 6   | Output         | LPT_nSELPRIN  | J13 pin 17 | Select (to printer)      |
| D      | 0   | Input          | UART_RX       | U1 PE1     | UART receive from MCU    |
| D      | 1   | Output         | UART_TX       | U1 PE0     | UART transmit to MCU     |
| D      | 2   | Input          | LPT_nACK      | J13 pin 10 | Acknowledge from Printer |
| D      | 3   | Input          | LPT_BUSY      | J13 pin 11 | Printer is Busy          |
| D      | 4   | Input          | LPT_nPE       | J13 pin 12 | Printer found Paper End  |
| D      | 5   | Input          | LPT_nERROR    | J13 pin 15 | Printer Error            |
| D      | 6   | Output         | LPT_nAUTOFEED | J13 pin 14 | Enable Auto Feed         |
| D      | 7   | Output         | LPT_nSTROBE   | J13 pin 1  | Latch data               |

Refer to [the schematic](../pcb/schematic.pdf) for more details.

## Protocol

The protocol between the I/O controller and the main MCU is pretty simple. As
the MCU is pretty busy drawing the screen, and there's only a 16-byte UART
FIFO on the MCU, we keep the I/O controller mostly quiet until commanded to
send a packet of data. This will normally occur once per screen refesh (so, 60
Hz), which should be plenty for reading the keyboard and mouse.

### Requests

These are the messages the MCU can send to the I/O controller. All are 1 byte
long, but some have following bytes.

* RESET_REQ (0x41) - Resets the attached keyboard and mouse as well as all
  status information.
* PS2_DATA_REQ (0x42) - Requests all the keypresses and mouse movements since the
  last call.
* PS2_LED_REQ (0x43) <byte> - Set the keyboard LEDs
* LPT_DATA_REQ (0x44) <byte> - Write a byte to the 8-bit LPT port. If you are
  talking to a printer you have to pulse nSTROBE and poll the ACK signal
  manually. If you are using the port as GPIO then you can do what you like.
* LPT_READ_REQ (0x45) - Read the status bits
* LPT_CTRL_REQ (0x46) <byte> - Set the control bits:
    * Bit 0: nSTROBE
    * Bit 1: nAUTOFEED
    * Bit 2: nSELECT
    * Bit 3: INIT
* LPT_BUFFERED_DATA_REQ (0x47) <len> <bytes...> - Write a a number of bytes to
  a buffer. Each will be automatically clocked out with a pulse of the nSTROBE
  line, then waiting for the printer to raise and then lower the BUSY signal.
* LPT_READ_PEND_REQ (0x48) <mask> <levels> - Send an indication when the pins specified by
  <mask> have the levels specified by <levels>. Use 0x00 and 0x00 to cancel an existing pend.
* LPT_SET_MODE_REQ (0x49) <mode> - Try and configure a specific IEEE-1284 mode.
    * 0x00 - Compatibility mode
    * Other values reserved
* PING_REQ (0x4A) - Empty request
* BOOTLOADER_REQ (0x4B) - Go to firmware programming mode

### Confirmations

These are the messages the I/O controller will send to the MCU, in response to the requests.

* RESET_CFM (0x61) - Confirms the reset is in progress. A BOOTED_IND will be
  sent when the reset is complete.
* PS2_DATA_CFM (0x62) <kb0> <kb1> <kb2> <mouse status> <mouse X> <mouse Y> - Keyboard and
  mouse data received since last previous PS2_DATA_CFM sent.
* PS2_LED_CFM (0x63) - Confirm the keyboard LEDs have been set
* LPT_DATA_CFM (0x64) - Sent when a byte has been written to the printer.
* LPT_READ_CFM (0x65) <byte> - Returns the status bits:
    * Bit 0: nACK
    * Bit 1: BUSY
    * Bit 2: OUT_OF_PAPER
    * Bit 3: SELECT
    * Bit 4: nERROR
* LPT_CTRL_CFM (0x66) - Sent when the control bytes have been written.
* LPT_BUFFERED_DATA_CFM <status> (0x67) - If status=0, buffered data has been
  stored, else there was an error (e.g. too much data).
* LPT_READ_PEND_CFM (0x68) - A pend has been received.
* LPT_SET_MODE_CFM (0x69) <result> - Result of a mode configuration.
    * 0x00 - Mode successfully negotiated
    * Non-zero - An error occurred
* PING_CFM (0x6A) - Empty response
* BOOTLOADER_CFM (0x6B) - Entering firmware programming mode

### Indications

These messages can be sent asynchronously by the I/O controller to the MCU.

* BOOTED_IND (0x30) <status> - Confirms the I/O controller is running and
  gives the keyboard and mouse status.
    * Bit 0 - 1 = Keyboard present, 0 = Keyboard missing
    * Bit 1 - 1 = Mouse present, 0 = Mouse missing
    * Bit 2:5 - 4-bit firmware version (v0..v15)
    * Bit 6 - Reserved (MCU should ignore)
    * Bit 7 - Always 1
* PS2_DATA_IND (0x31) - Indicates that data is waiting to be read. Only sent
  once, until a PS2_DATA_REQ is seen.
* LPT_BUFFER_EMPTY_IND (0x32) - Indicates that the LPT buffer is now empty and
  more bytes can be sent.
* LPT_READ_PEND_IND (0x33) <pins> - The input pins matched the given levels.
* BAD_COMMAND_IND (0x34) - Send when a bad request is received. That request
  will not receive a Confirmation.

### Keyboard Data Format

The PS/2 Scan Code Set 2 is translated by the I/O controller into a basic code
where each key is numbered 0..127. The top bit is then set for a key-up event
and cleared for a key-down event. The 00 byte is a no-op. Only three key
events can be sent every frame (i.e. at 60 Hz, or 180 key events per second or
90 keystrokes per second). I doubt anyone will need to type that fast on a
Monotron.

* F9: 0x01
* AltRight: 0x02
* F5: 0x03
* F3: 0x04
* F1: 0x05
* F2: 0x06
* F12: 0x07
* ControlRight: 0x08
* F10: 0x09
* F8: 0x0A
* F6: 0x0B
* F4: 0x0C
* Tab: 0x0D
* BackTick: 0x0E
* WindowsLeft: 0x0F
* WindowsRight: 0x10
* AltLeft: 0x11
* ShiftLeft: 0x12
* Menus: 0x13
* ControlLeft: 0x14
* Q: 0x15
* Key1: 0x16
* NumpadSlash: 0x17
* NumpadEnter: 0x18
* End: 0x19
* Z: 0x1A
* S: 0x1B
* A: 0x1C
* W: 0x1D
* Key2: 0x1E
* ArrowLeft: 0x1F
* Home: 0x20
* C: 0x21
* X: 0x22
* D: 0x23
* E: 0x24
* Key4: 0x25
* Key3: 0x26
* Insert: 0x27
* Delete: 0x28
* Spacebar: 0x29
* V: 0x2A
* F: 0x2B
* T: 0x2C
* R: 0x2D
* Key5: 0x2E
* ArrowDown: 0x2F
* ArrowRight: 0x30
* N: 0x31
* B: 0x32
* H: 0x33
* G: 0x34
* Y: 0x35
* Key6: 0x36
* ArrowUp: 0x37
* PageDown: 0x38
* PageUp: 0x39
* M: 0x3A
* J: 0x3B
* U: 0x3C
* Key7: 0x3D
* Key8: 0x3E
* Comma: 0x41
* K: 0x42
* I: 0x43
* O: 0x44
* Key0: 0x45
* Key9: 0x46
* Fullstop: 0x49
* Slash: 0x4A
* L: 0x4B
* SemiColon: 0x4C
* P: 0x4D
* Minus: 0x4E
* Quote: 0x52
* BracketSquareLeft: 0x54
* Equals: 0x55
* CapsLock: 0x58
* ShiftRight: 0x59
* Enter: 0x5A
* BracketSquareRight: 0x5B
* BackSlash: 0x5D
* Backspace: 0x66
* Numpad1: 0x69
* Numpad4: 0x6B
* Numpad7: 0x6C
* Numpad0: 0x70
* NumpadPeriod: 0x71
* Numpad2: 0x72
* Numpad5: 0x73
* Numpad6: 0x74
* Numpad8: 0x75
* Escape: 0x76
* NumpadLock: 0x77
* F11: 0x78
* NumpadPlus: 0x79
* Numpad3: 0x7A
* NumpadMinus: 0x7B
* NumpadStar: 0x7C
* Numpad9: 0x7D
* ScrollLock: 0x7E
* F7: 0x7F

### Mouse Data Format

This is as per the PS/2 specification, except that mutiple mouse packets (sent at 100 Hz) may be coalesced into a single message to the MCU (which only reads data at 60 Hz or less).

* Status:
    * Bit 7 - Y overflow
    * Bit 6 - X overflow
    * Bit 5 - Y sign bit
    * Bit 4 - X sign bit
    * Bit 3 - Always 1
    * Bit 2 - Middle Button
    * Bit 1 - Right Button
    * Bit 0 - Left Button
* X: Distance moved in the X direction since the last reading, in the range 0..255 (plus the sign bit above, so -255 to +255)
* Y: Distance moved in the Y direction since the last reading. Values are per `X` above.

## Licence

The code is Copyright (c) Jonathan 'theJPster' Pallant 2019. It is available under the Apache 2.0 or MIT licences, at your option.

The code in the `third_party/avr_uart` folder is under the following licence:

```
Copyright (C) 2012 Andy Gock

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
```
