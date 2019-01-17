# Monotron I/O Controller

## Introduction

This is the firmware for the Monotron's I/O controller. It is an Atmel AVR
Atmega48, and it controls the:

* PS/2 Keyboard port
* PS/2 Mouse port
* PC Printer (aka LPT, or Centronics) port

It talks to the main Monotron MCU over a serial link at 115,200 bps (8-data
bits, no parity, 1 stop bit).

## Protocol

The protocol between the I/O controller and the main MCU is pretty simple. As
the MCU is pretty busy drawing the screen, and there's only a 16-byte UART
FIFO on the MCU, we keep the I/O controller quiet until commanded to send a
packet of data. This will normally occur once per screen refesh (so, 60 Hz),
which should be plenty for reading the keyboard and mouse.

### Requests

These are the messages the MCU can send to the I/O controller. All are 1 byte
long, but some have following bytes.

* RESET_REQ (0x80) - Resets the attached keyboard and mouse as well as all
  status information.
* PS2_DATA_REQ (0x81) - Requests all the keypresses and mouse movements since the
  last call.
* PS2_LED_REQ (0x88) <byte> - Set the keyboard LEDs
* LPT_DATA_REQ (0x82) <byte> - Write a byte to the 8-bit LPT port. If you are
  talking to a printer you have to pulse nSTROBE and poll the ACK signal
  manually. If you are using the port as GPIO then you can do what you like.
* LPT_READ_REQ (0x83) - Read the status bits
* LPT_CTRL_REQ (0x84) <byte> - Set the control bits:
    * Bit 0: nSTROBE
    * Bit 1: nAUTOFEED
    * Bit 2: nSELECT
    * Bit 3: INIT
* LPT_BUFFERED_DATA_REQ (0x85) <len> <bytes...> - Write a a number of bytes to
  a buffer. Each will be automatically clocked out with a pulse of the nSTROBE
  line, then waiting for the printer to raise and then lower the BUSY signal.
* LPT_READ_PEND_REQ (0x86) <mask> <levels> - Send an indication when the pins specified by
  <mask> have the levels specified by <levels>. Use 0x00 and 0x00 to cancel an existing pend.
* LPT_SET_MODE_REQ (0x87) <mode> - Try and configure a specific IEEE-1284 mode.
    * 0x00 - Compatibility mode
    * Other values reserved
* PING_REQ (0x89) - Empty request
* BOOTLOADER_REQ (0x8A) - Go to firmware programming mode


### Confirmations

These are the messages the I/O controller will send to the MCU, in response to the requests.

* RESET_CFM (0x80) - Confirms the reset is in progress. A BOOTED_IND will be
  sent when the reset is complete.
* PS2_DATA_CFM (0x81) <kb0> <kb1> <kb2> <mouse status> <mouse X> <mouse Y> - Keyboard and
  mouse data received since last previous PS2_DATA_CFM sent.
* PS2_LED_CFM (0x88) - Confirm the keyboard LEDs have been set
* LPT_DATA_CFM (0x82) - Sent when a byte has been written to the printer.
* LPT_READ_CFM (0x83) <byte> - Returns the status bits:
    * Bit 0: nACK
    * Bit 1: BUSY
    * Bit 2: OUT_OF_PAPER
    * Bit 3: SELECT
    * Bit 4: nERROR
* LPT_CTRL_CFM (0x84) - Sent when the control bytes have been written.
* LPT_BUFFERED_DATA_CFM (0x85) - Buffered data has been stored.
* LPT_READ_PEND_CFM (0x86) - A pend has been received.
* LPT_SET_MODE_CFM (0x87) <result> - Result of a mode configuration.
    * 0x00 - Mode successfully negotiated
    * Non-zero - An error occurred
* PING_CFM (0x89) - Empty response
* BOOTLOADER_CFM (0x8A) - Entering firmware programming mode

### Indications

These messages can be sent asynchronously by the I/O controller to the MCU.

* BOOTED_IND (0xF0) <status> - Confirms the I/O controller is running and
  gives the keyboard and mouse status.
    * Bit 0 - 1 = Keyboard present, 0 = Keyboard missing
    * Bit 1 - 1 = Mouse present, 0 = Mouse missing
* PS2_DATA_IND (0xF1) - Indicates that data is waiting to be read. Only sent
  once, until a PS2_DATA_REQ is seen.
* LPT_BUFFER_EMPTY_IND (0xF2) - Indicates that the LPT buffer is now empty and
  more bytes can be sent.
* LPT_READ_PEND_IND (0xF3) <pins> - The input pins matched the given levels.

### Keyboard Data Format

The PS/2 Scan Code Set 2 is translated by the I/O controller into a basic code
where each key is numbered 0..127. The top bit is then set for a key-up event
and cleared for a key-down event. The 00 byte is a no-op. Only three key
events can be sent every frame (i.e. at 60 Hz, or 180 key events per second or
90 keystrokes per second). I doubt anyone will need to type that fast on a
Monotron.

* AltLeft: 0x01
* AltRight: 0x02
* ArrowDown: 0x03
* ArrowLeft: 0x04
* ArrowRight: 0x05
* ArrowUp: 0x06
* BackSlash: 0x07
* Backspace: 0x08
* BackTick: 0x09
* BracketSquareLeft: 0x0A
* BracketSquareRight: 0x0B
* CapsLock: 0x0C
* Comma: 0x0D
* ControlLeft: 0x0E
* ControlRight: 0x0F
* Delete: 0x10
* End: 0x11
* Enter: 0x12
* Escape: 0x13
* Equals: 0x14
* F1: 0x15
* F2: 0x16
* F3: 0x17
* F4: 0x18
* F5: 0x19
* F6: 0x1A
* F7: 0x1B
* F8: 0x1C
* F9: 0x1D
* F10: 0x1E
* F11: 0x1F
* F12: 0x20
* Fullstop: 0x21
* Home: 0x22
* Insert: 0x23
* Key1: 0x24
* Key2: 0x25
* Key3: 0x26
* Key4: 0x27
* Key5: 0x28
* Key6: 0x29
* Key7: 0x2A
* Key8: 0x2B
* Key9: 0x2C
* Key0: 0x2D
* Menus: 0x2E
* Minus: 0x2F
* Numpad0: 0x30
* Numpad1: 0x31
* Numpad2: 0x32
* Numpad3: 0x33
* Numpad4: 0x34
* Numpad5: 0x35
* Numpad6: 0x36
* Numpad7: 0x37
* Numpad8: 0x38
* Numpad9: 0x39
* NumpadEnter: 0x3A
* NumpadLock: 0x3B
* NumpadSlash: 0x3C
* NumpadStar: 0x3D
* NumpadMinus: 0x3E
* NumpadPeriod: 0x3F
* NumpadPlus: 0x40
* PageDown: 0x41
* PageUp: 0x42
* PauseBreak: 0x43
* PrintScreen: 0x44
* ScrollLock: 0x45
* SemiColon: 0x46
* ShiftLeft: 0x47
* ShiftRight: 0x48
* Slash: 0x49
* Spacebar: 0x4A
* Tab: 0x4B
* Quote: 0xC4
* WindowsLeft: 0x4D
* WindowsRight: 0x4E
* A: 0x4F
* B: 0x50
* C: 0x51
* D: 0x52
* E: 0x53
* F: 0x54
* G: 0x55
* H: 0x56
* I: 0x57
* J: 0x58
* K: 0x59
* L: 0x5A
* M: 0x5B
* N: 0x5C
* O: 0x5D
* P: 0x5E
* Q: 0x5F
* R: 0x60
* S: 0x61
* T: 0x62
* U: 0x63
* V: 0x64
* W: 0x65
* X: 0x66
* Y: 0x67
* Z: 0x68
* HashTilde: 0x69
* PrevTrack: 0x6A
* NextTrack: 0x6B
* Mute: 0x6C
* Calculator: 0x6D
* Play: 0x70F
* Stop: 0x71
* VolumeDown: 0x72
* VolumeUp: 0x73
* WWWHome: 0x74


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
