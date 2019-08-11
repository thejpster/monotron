# Monotron.

> A simple 1980's home computer style application for the Tiva-C Launchpad

## Introduction

Monotron is powered by a Texas Instruments TM4C123 microcontroller, containing
an ARM Cortex-M4 core and a number of peripherals. This processor was chosen
because it is available on an inexpensive dev-kit - the Tiva-C Launchpad - and
I happened to have some lying around. The challenge I set myself was, how much
can you squeeze out of this tiny CPU? And can you do it all in pure-Rust?

![YouTube screenshot of a video of Monotron](screenshot.jpg "Monotron on Youtube")
[See Monotron in Action!](https://www.youtube.com/watch?v=7x_92PXKSN4)

## Features

* 800x600 8-colour VGA output
* 32 KiB SRAM (24 KiB usable for applications)
* 256 KiB Flash ROM
* Choice of low-memory 48x36 text display mode or full 384x288 bitmap graphics mode
* 8-bit mono audio output with 3 channel, 4 waveform synthesiser
* USB Serial I/O
* Atari 9-pin joystick interface
* Simple command-line interface
* I2C expansion interface
* Loadable apps
* Battery backed real-time clock*
* 25-pin IBM PC style parallel printer port*
* MIDI In, Out and Through*
* PS/2 Keyboard and Mouse*
* RS-232 Serial port*
* Watch this space!


 _* Requires additional hardware, included on the [Monotron PCB](#monotron-pcb)_

## Video

Monotron generates an 800x600 VGA video signal at 60 Hz using three SPI
peripherals and a timer to generate the horizontal sync signals. It can do
this because the VGA signal has a pixel clock of 40 MHz and the Tiva-C
Launchpad's TM4C123 CPU runs at 80 MHz. We save on the number of pixels we
have to push through the SPIs by running at half-resolution horizontally
(giving 400x600), which also halves the pixel clock to 20 MHz. I did try 40
MHz mode and it didn't work.

Monotron has two 'modes' it can display on this VGA output.

### Text Mode

Text Mode has a 48 character by 36 line display. Each character cell is 8
pixels wide and 16 pixels high and can take any character from the 8-bit [MS-
DOS Code Page 850](https://en.wikipedia.org/wiki/Code_page_850) character set,
and can have any foreground and background colour from the supported set:

* White
* Red
* Yellow
* Green
* Cyan
* Blue
* Magenta
* Black

The text buffer takes up `48 x 36 x 2 = 3,456` bytes of SRAM.

The built-in font is taken from
[FreeBSD](http://web.mit.edu/freebsd/head/share/syscons/fonts/cp850-8x16.fnt).
There's also a second font which implements Teletext block graphics (or
'sixels').

Any line of text can be displayed in "double height" mode, showing either the
top-half or bottom-half.

Finally, the display is framed with an 8 pixel border at the sides and a 12
pixel border at the top and bottom, to make everything fit neatly and to help
with any minor overscan issues if you use an actual CRT monitor.

### Graphics Mode

Graphics Mode can be enabled and disabled at run-time. It's not enabled by
default because bitmap graphics take up a lot of RAM!

You can attach a 1-bit-per-pixel graphics buffer that is some multiple of 384
pixels (i.e. 384 bits or 48 bytes) long. This buffer is displayed using line-
doubling (i.e. every line is shown twice) so you can go up to 384x288
resolution maximum, which will fill the screen. Each bit from the bitmap is
coloured according to the text cell (see above) it sits on top of, much like a
ZX Spectrum. A full-screen bitmap therefore uses the 3,456 bytes of SRAM from
text-mode plus an additional `384x288 / 8 = 13,824` bytes of SRAM.

## Compiling

You will need to build using Rust Nightly, as we need various experimental
features for Embedded development that are not yet available in Stable.

```
$ rustup toolchain install nightly
$ git clone https://github.com/thejpster/monotron.git
$ cd monotron
$ rustup override set nightly
$ rustup target add thumbv7em-none-eabihf
$ cargo build --release
```

To program the board, you can use lm4flash:

```
$ cargo build --release
$ arm-none-eabi-objcopy -O binary ./target/thumbv7em-none-eabihf/release/monotron ./target/thumbv7em-none-eabihf/release/monotron.bin
$ lm4flash ./target/thumbv7em-none-eabihf/release/monotron.bin
```

Or you can debug in GDB (which will automatically load the program first):

```
$ openocd
<switch to a different terminal>
$ cargo run --release
```

OpenOCD should read our `openocd.cfg` file, which directs it to use the
correct configuration. You may need to run `sudo openocd` if your user doesn't
have permission to open the USB device.

To exit GDB, you may need to press Ctrl-C multiple
times, as it seems it can get a bit stuck.

## Connecting

You have two options when running a Monotron.

1. You can take a bare Tiva-C Launchpad and wire up various connectors using
   the pin-outs in the following sections.
2. You can skip all that make yourself a [Monotron PCB](#monotron-pcb)!

### VGA

Your VGA connector requires five wires:

* Pin 1: Red - connect to PF1 via a 330 Ohm resistor.
* Pin 2: Green - connect to PB7 via a 330 Ohm resistor.
* Pin 3: Blue - connect to PD3 via a 330 Ohm resistor.
* Pin 5: Ground - connect to GND
* Pin 6: Red Return - connect to GND
* Pin 7: Green Return - connect to GND
* Pin 8: Blue Return - connect to GND
* Pin 13: H-Sync - connect to PB4
* Pin 14: V-Sync - connect to PB5

I'm using this arrangement using random resistors I found on my desk, and it
works for me (although the picture is a bit dim, as it actually produces about
0.6V peak rather than 0.7V):

```
-----+
     |     +------+ 330 Ohm        Co-ax in the VGA cable
PB7 o+-----|      |------------(o)==================)+
     |     +------+                                  |
-----+                                               |
                                                    +-+
                                                    | |
                                                    | | 75 Ohm
                                                    | | (in Monitor)
                                                    +-+
                                                     |
                                                     o
                                                    GND
```

The 330 Ohm resistor forms a resistive divider with the 75 Ohm resistor in the
monitor. This is needed to drop the 3.3V output down to 0.7V. Some monitors
are more tolerant of over voltage than others. The higher the resistor you
use, the less current you pulling out of the GPIO pin (we're just over 8mA
currently, which is a bit high) but the lower the voltage the monitor will see
and the dimmer your picture will be. Conversely if you lower the resistor,
more current will flow but it'll be a brigher picture. I'd save your chip from
damage and just wind the brightness control up!

Obviously only one channel is shown above - wire up the blue and red channels
in exactly the same fashion. Finally, don't forget to keep your wires short!
You will have noise if you try and send a 20 MHz signal down 10cm of
unshielded wire.

In a perfect world, your board would offer a 75 ohm source impendance matching
the monitor's 75 ohm impedance, to reduce reflections, but at this resolution
it doesn't seem to matter. If you want do do that, you'll need to make a
resistive divider to drop the 3.3V to 1.4V, and then feed that through a
high-bandwidth (>20 MHz) unity-gain amplifier, with a 75 ohm resistor on the
output. The pair of 75 ohm resistors will then drop the 1.4V to 0.7V in the
monitor.

### UART

Monotron primarily uses UART0, which is converted to USB Serial by the
on-board companion chip on the Tiva-C Launchpad. Connect with your favourite
Serial terminal at 115,200bps, then send UTF-8 characters and they'll get
converted to virtual keyboard input, allowing you to drive the Monotron.

There's also a second UART (UART1), which has RTS/CTS hardware handshaking
lines connected (but not DSR, DTR or RI). On the Monotron PCB these are
brought out to pin header J8. By fitting six jumpers on this header, the
on-board MAX3232 level shifter is activated, driving RS-232 signals on the
DE9M connector, J12. This connector is wired as [Data Terminal
Equipment](https://en.wikipedia.org/wiki/Data_terminal_equipment) or _DTE_ (as
opposed to [Data Communications
Equipment](https://en.wikipedia.org/wiki/Data_circuit-terminating_equipment)
or _DCE_). This means that Monotron transmits data on DE9 pin 3, and it's
designed to connect to Serial AT modems or other peripherals. Other computers
(like an old IBM PC) and even USB to RS-232 adaptors are most likely wired DTE
and so to connect one to the Monotron you'll most likely need a [null-modem
cable](https://en.wikipedia.org/wiki/Null_modem) (one that swaps pins 3 + 4
and pins 7 + 8).

Bonus points to the first person to write a BBS program for Monotron that lets
you dial up on a 56k modem.

Extra bonus points to the first person to write an xmodem file transfer
program so you can transfer files from SD card over RS-232 to an xmodem
utility on your old MS-DOS 3.3 IBM PC.

_Note: The Joystick connector looks the same as the RS232 connector - don't
mix them up!_

On the Monotron PCB, a third UART (UART3) is routed through various
opto-isolators to the MIDI In and MIDI Out ports. The MIDI Through port just
repeats everything received on the MIDI In port.

Finally, also on the Monotron PCB, a fourth UART (UART7) connects to a 5V
AtMega48 which is used as an I/O expander. This microcontroller drives two
PS/2 ports (one for keyboard, one for the mouse) as well as a full IBM
PC-style 25-pin parallel printer port.

| Launchpad Pin | Tiva-C Pin | External Pin | Function (from Monotron's point of view) |
|---------------|------------|--------------|---------------|
| N/A           | PA0        | N/A          | USB Serial Rx |
| N/A           | PA1        | N/A          | USB Serial Tx |
| J4.8          | PD6        | J12 pin 3    | RS-232 Tx     |
| J4.9          | PD7        | J12 pin 2    | RS-232 Rx     |
| J4.10         | PF4        | J12 pin 7    | RS-232 RTS    |
| J3.2          | GND        | J12 pin 8    | RS-232 CTS    |
| J4.6          | PC6        | J11 pin 4/5  | MIDI In       |
| J4.7          | PC7        | J9 pin 4/5   | MIDI Out      |
| N/A           | N/A        | J10 pin 4/5  | MIDI Through  |

### Audio

Monotron can generate 8-bit audio output using PWM on pin PE4. I use the
[monotron- synth](https://github.com/thejpster/monotron-synth) which has a
three-channel wavetable synthesiser which can bleep and bloop with square
waves, sine waves, sawtooth waves and generate white noise.

You'll need to run the pin through a low-pass filter to remove the noise, and
connect it to an amplifier as the GPIO pin won't really supply much current.

| Launchpad Pin | Tiva-C Pin | TRS          | Function  |
|---------------|------------|--------------|-----------|
| J1.5          | PE4        | Ring/Tip     |Audio Left |
| J1.6          | PE5        | Tip          |Audio Right|
| J3.2          | GND        | Sleeve       |Common     |

Currently audio only comes out as Mono on PE4 (Left). If this is a problem for
your amplifier, setting jumper J1 on the PCB to 1-2 allows you to route Audio
Left to Sleeve as well as Tip. Setting J1 to 2-3 enables stereo support,
should that ever get developed.

### Joystick

There are five active-low inputs corresponding to Up, Down, Left, Right and
Fire. You can connect these inputs to a standard Atari or Commodore 9-pin
Joystick as follows:

| Launchpad Pin | Tiva-C Pin | Joystick Pin | Function |
|---------------|------------|--------------|----------|
| J3.8          | PE2        | 1            | Up       |
| J3.9          | PE3        | 2            | Down     |
| J4.8          | PD6        | 3            | Left     |
| J4.9          | PD7        | 4            | Right    |
| J4.10         | PF4        | 6            | Fire     |
| J3.2          | GND        | 8            | Ground   |

A Sega Master System (tm) controller should work, except you'll only be able
to use Button 1 not Button 2. A Sega Mega Drive (tm) controller probably won't
work because it has more buttons than there are pins, and so uses a 5V powered
multiplexer to select between the two banks of buttons. 15-pin Analog PC
joysticks won't work either.

### SD Card

You can load programs and data from an SD card, from the first Primary MBR
(standard old-style MS-DOS) partition, formatted as either FAT16 or FAT32. Use
a standard SD Card SPI breakout adaptor, connected up as follows:

| Launchpad Pin | Tiva-C Pin | Function     |
|---------------|------------|--------------|
| J2.10         | PA2        | Clock        |
| J2.9          | PA3        | Chip Select  |
| J2.8          | PA4        | MISO/Data Out|
| J1.8          | PA5        | MOSI/Data In |

SD cards operate at 3.3v so no level shifters are required, but many cards
will require a 47k (well, between 10k and 100k) pull-up to 3.3v on all four
data pins. If you don't use these pull-ups, the pins float while the Monotron
is booting, and these fluctuations may upset the SD card, giving you random
timeouts and other spurious effects. Some cards tolerate this better than
others, so YMMV. If in doubt, just add the pull-ups (they're missing on rev
0.7.0 of the PCB, but will be on 0.8.0+).

On the console you use the `mount` command to scan the disk, then `dir` to
show the root directory contents. You can also `dload`, `ddump` and `dpage`
commands to load, hex-dump and print files to the screen. A program loaded
with `dload` can subsequently be executed with the `run` command.

### I2C

The [Monotron PCB](#monotron-pcb) has an I2C expansion header connected to
`I2C1`. The PCB can also optionally be fitted with an [Microchip
MCP7940N](https://www.microchip.com/wwwproducts/en/MCP7940N) battery-backed
real time clock chip. Driver support for this chip is TBC.

The PCB pulls-up the I2C bus to 5V through 4.7k pull-ups. On these particular
pins the TM4C123 happens to be 5V tolerant and so it simply runs the I2C pins
in open-drain mode without a level shifter.

| Launchpad Pin | Tiva-C Pin | External Pin | Function     |
|---------------|------------|--------------|--------------|
| J2.1          | GND        | J6 pin 1     | Ground       |
| J3.1          | 5V         | J6 pin 2     | 5V Power     |
| J1.10         | PA7        | J6 pin 3     | SDA          |
| J1.9          | PA6        | J6 pin 4     | SCL          |

### PS/2 Keyboard

The initial PS/2 keyboard support sort of worked, but it wasn't reliable I so
took it out again. The fundamental problem was that it's really hard to sample
a 10 to 15 kHz incoming synchronous signal without using SPI slave peripheral
(they're all in use) and while bit-bashing timing-critical video signals at 20
MHz. Either you drop random key events (leading to missing characters or stuck
keys), or the video wobbles every time a key is pressed (like a ZX81). Neither
was ideal.

On the [Monotron PCB](#monotron-pcb) I work around this issue by adding an
Atmel AtMega48 microcontroller as an I/O expander, connected to [UART](#uart)
7. Look in the [`avr_kb`](./avr_kb) folder for more information.

__Note:__ There's a bug in the 0.7.0 PCB where both PS/2 connector pinouts are
mirror image, compared to how they should be. It happened because although the
connector drawings said "Bottom View" quite clearly, I didn't pay enough
attention and assumed they were viewed from above! To use the PS/2 connectors,
they must be fitted to the underside of the PCB (i.e. opposite to all the
other connectors). This will be fixed in the next PCB revision.

### MIDI

The [Monotron PCB](#monotron-pcb) has three 5-pin DIN MIDI ports: In, Out and
Through. These are connected to UART 3.

### Parallel Printer Port

The 25-pin Parallel Printer Port is connected to the Monotron PCB's AtMega48
I/O controller. You send commands to the AtMega to get it to drive the printer
port. Currently only SPP (classic mono-directional support as found on the
original IBM PC) is planned for support. Support for the fancier EPP and ECP
modes is TBD.

## Monotron PCB

There's now a PCB! It features:

* 2x20 pin headers, to mate with the TM4C123 Launchpad (other Launchpads like
  the TM4C129 and MSP430 have not been tested and almost certainly won't work)
* 5V power jack (2.1mm barrel jack, centre +ve)
* [VGA](#vga) port (DE15HD female)
* Atari/Commodore [joystick](#joystick) port (DE9 male)
* [Micro-SD Card](#sd-card) slot
* [3.5mm stereo line out](#audio)
* [RS-232](#uart) port (DE9 male)
* 2.54mm (100 mil) jumpers to disconnect the RS232 driver and allow you to
  connect a 3.3v FTDI type TTL serial cable instead.
* [MIDI](#midi) In, Out and Through ports, with opto-isolation (5-pin DIN)
* 4-pin 2.54mm pitch (100 mil) [I2C](#i2c) expansion header (GND, 5V, SDA, SCL)
* Microchip MCP7940N I2C Real-Time Clock with battery back-up
* AtMega48 microcontroller, controlling:
  * IBM PC style parallel printer port (DB25 female)
  * PS/2 Keyboard port (6-pin mini-DIN)
  * PS/2 Mouse port (6-pin mini-DIN)

See the [pcb](./pcb) folder for Kicad files, Gerbers and PDF Schematics. If
you want to buy a PCB (with or without a kit of parts) send a message to
@therealjpster on Twitter, or via my Github e-mail. Rev 0.7.0 has some rough
edges, but I'm working on it.

## Running

When running, a simple command driven interface is presented. Commands can be
entered over serial, or using a PS/2 keyboard. Commands are split on
whitespace and then interpreted based on the left-most word. Enter the command
'help' to see a list of commands. Some commands place you in to a sub-menu -
use 'exit' to return to the previous menu.

## Loading apps

Applications can be compiled and loaded into RAM for exection. They must be
linked to run from address `0x2000_2000` and take less than 24 KiB for all
code and data combined. See the table below:

| Address     | Length  | Description  |
|-------------|---------|--------------|
| 0x0000_0000 | 256 KiB | OS Code      |
| 0x2000_0000 | 8 KiB   | OS Data      |
| 0x2000_2000 | 24 KiB  | Application  |

The first four bytes of the image must be the address of the start function,
with prototype `fn start(const struct callbacks_t* callbacks) -> int32`. Apart
from that, applications are free to apportion the remaining 24,572 bytes as
they see fit.

*Note:* The application does not need to provide a stack region - the Monotron
ROM will handle that using the system stack.

The callback structure supplied to the application's entry function is defined
in `api.rs`, but in C looks like:

```C
struct callbacks_t {
    int32_t (*putchar)(void* p_context, char ch);
    int32_t (*puts)(void* p_context, const char*);
    int32_t (*readc)(void* p_context);
    void (*wfvbi)(void* p_context);
    int32_t (*kbhit)(void* p_context);
    void (*move_cursor)(void* p_context, unsigned char row, unsigned char col);
    int32_t (*play)(void* p_context, uint32_t frequency, uint8_t channel, uint8_t waveform, uint8_t volume);
    void (*change_font)(void* p_context, uint32_t mode, const void* p_font);
    uint8_t (*get_joystick)(void* p_context);
    void (*set_cursor_visible)(void* p_context, uint8_t visible);
};
```

The C functions exported to the apps are:

* `puts` - print an 8-bit string (certain escape sequences are understood).
  Note that unlike the C routine of the same name, this function does not
  append a newline automatically. It is more like `fputs(s, stdout)`.
* `putchar` - print an 8-bit character
* `readc` - blocking wait for keyboard/serial input
* `wfvbi` - Wait For next Vertical Blanking Interval
* `kbhit` - return 1 if a key has been pressed (and so `readc` won't block),
  else return 0
* `move_cursor` - move the cursor to change where the next print goes
* `play` - play a note on one of the synthesizer channels
* `change_font` - changes the font used on screen to the normal CodePage 850,
  the Teletext font, or a custom font supplied by the application.
* `get_joystick` - returns the current state of the joystick input. Bits 0-4
  correspond to Fire, Right, Left, Down and Up respectively.
* `set_cursor_visible` - Pass 0 to disable the `_` cursor, or non-zero to enable it.

You can use the `upload` Python script in this repo to upload binary images
into RAM, or you can use the `dload` to load them from SD card.

See [monotron-apps](https://github.com/thejpster/monotron-apps) for example
apps which will run from Monotron's RAM, along with a wrapper which makes
using the callbacks as simple as using a normal C library.

## Unreleased changes (will be 0.10.0)

* Fixed video interrupt jitter by entering WFI before drawing pixels.
* Updated VGA framebuffer callback API

## Changelog

* Version 0.9.2 - Updated menu interface crate
* Version 0.9.0 - Added I2C commands and support for the AtMega keyboard
  controller (WIP) on the Monotron PCB.
* Version 0.8.0 - Added cursor support to ABI. Added basic SD Card support
  (read-only).
* Version 0.7.0 - Move application RAM to 0x2000_2000. Added cursor support.
  Moved callback pointer.
* Version 0.6.3 - Fixed Joystick support.
* Version 0.6.2 - Add Joystick support.
* Version 0.6.1 - Add Teletext font and support for font-switching in apps.
* Version 0.6.0 - Added sound and support for apps running from RAM. Removed
  PS/2 keyboard support.
* Version 0.5.0 - Added 1bpp graphics mode.
* Version 0.4.0 - Added PS/2 keyboard support.
* Version 0.3.0 - Backspace works.
* Version 0.2.0 - Switch to a text buffer to save RAM. Basic animations work.
* Version 0.1.0 - First release. VGA output works but menu is full of dummy
  commands and there's no keyboard input.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
