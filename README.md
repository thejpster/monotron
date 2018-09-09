# Monotron.

> A simple 1980's home computer style application for the Stellaris Launchpad

## Introduction

Monotron is powered by a Texas Instruments LM4F120 microcontroller, containing
an ARM Cortex-M4 core and a number of peripherals. This processor was chosen
because it is available on an inexpensive dev-kit - the Stellaris Launchpad -
and I happened to have some lying around. The challenge I set myself was, how
much can you squeeze out of this tiny CPU? And can you do it all in pure-Rust?

![YouTube screenshot of a video of Monotron](screenshot.jpg "Monotron on Youtube")
[See Monotron in Action!](https://www.youtube.com/watch?v=7x_92PXKSN4)

## Features

* 800x600 8-colour VGA output
* 32 KiB SRAM (24 KiB usable for applications)
* 256 KiB Flash ROM
* Choice of low-memory text display mode or full bitmap graphics mode
* PS/2 keyboard input
* USB Serial I/O
* Simple command-line interface
* Watch this space!

## Video

Monotron generates an 800x600 VGA video signal at 60 Hz using three SPI
peripherals and a timer to generate the horizontal sync signals. It can
do this because the VGA signal has a pixel clock of 40 MHz and the Stellaris
Launchpad's LM4F120 CPU runs at 80 MHz. We save on the number of pixels
we have to push through the SPIs by running at half-resolution horizontally
(giving 400x600), which also halves the pixel clock to 20 MHz.

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

The built-in front is taken from [FreeBSD](http://web.mit.edu/freebsd/head/share/syscons/fonts/cp850-8x16.fnt).

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
$ sudo openocd -f /usr/share/openocd/scripts/board/ek-lm4f120xl.cfg
<switch to a different terminal>
$ cargo run --release
```

To exit GDB, you may need to press Ctrl-C multiple times, as it seems it can get a bit stuck.

## Connecting

### VGA

Your VGA connector requires five wires:

* Pin 1: Red - connect to PF1 via a resistive divider.
* Pin 2: Green - connect to PB7 via a resistive divider.
* Pin 3: Blue - connect to PD3 via a resistive divider.
* Pin 5: Ground - connect to GND
* Pin 6: Red Return - connect to GND
* Pin 7: Green Return - connect to GND
* Pin 8: Blue Return - connect to GND
* Pin 13: H-Sync - connect to PB4
* Pin 14: V-Sync - connect to PB5

The resistive divider needs to drop the 3.3V output down to 0.7V. Some
monitors are more tolerant of over voltage than others. In a perfect world,
your board would offer a 75 ohm source impendance matching the monitor's 75
ohm impedance, to reduce reflections, but at this resolution it doesn't really
matter. In any case, a 75 ohm source impedance will probably pull too much
current out of the GPIO pin and cause you problems.

I'm using this arrangement using random resistors I found on my desk, and it
works for me (although the picture is a bit dim, as it actually produces about
0.2V peak rather than 0.7V):

```
-----+
     |
PB7 o+---------------+
     |               |
     |              +-+
     |              | |
     |              | | 330 Ohm
     |              | |
     |              +-+
-----+               |
                     +----------- VGA pin 2----------+
                     |                               |
                    +-+                             +-+
                    | |                             | |
                    | | DNF                         | | 75
                    | |                             | | (in Monitor)
                    +-+                             +-+
                     |                               |
                     o                               o
                    GND                             GND
```

DNF means 'did not fit' - you may need to experiment with putting a resistor
here but I left it out (so basically its resistance is infinite) and it's
working OK on my monitor. Obviously only one channel is shown above - wire up
the blue and red channels in exactly the same fashion.

### PS/2 Keyboard

PS/2 keyboard support is mostly working, but consider it alpha grade. See the
[pc- keyboard](https://github.com/thejpster/pc-keyboard) crate. Any UK 102-key
or 105-key keyboard should work - support for other layouts welcome as a PR!

Be aware, there's clearly an issue with the interrupt handling as about 10% of
keystrokes get corrupted. Currently this displays an error on the screen, or
perhaps as a double character when you only pressed a key once. I'm hoping to
improve it.

The pinout is:

* +CLK: PA2
* +DATA: PA4
* Ground: GND
* Vcc: 5V

Tie PA3 (Keyboard Chip Select) to Ground.

PS/2 keyboards have 5V I/O. It's specified as open-collector but keyboards
sometimes contain internal pull-up resistors to 5V. All of the LM4F120/TM4C123
I/O pins are 5V tolerant when in input mode (except PB0, PB1, PD4, PD5). You
should probably add a 10k pull-up resistor to 5V on both +CLK and +DATA just
in case your keyboard hasn't got one.

Monotron currently doesn't support talking back to the keyboard (e.g. to turn
the SCROLL, NUM and CAPS-LOCK lights on)- to do so would probably require more
robust interface circuitry.

### UART

Monotron uses UART0 on the Stellaris Launchpad, which is converted to USB
Serial by the on-board companion chip. Connect with your favourite Serial
terminal at 115,200bps. Send UTF-8 and it'll get converted to MS-DOS Code Page
850 inside the Monotron.

## Running

When running, a simple command driven interface is presented. Commands can be
entered over serial, or using a PS/2 keyboard. Commands are split on
whitespace and then interpreted based on the left-most word. Enter the command
'help' to see a list of commands. Some commands place you in to a sub-menu -
use 'exit' to return to the previous menu.

## Changelog

* Version 0.6.0 - Changed pinout to move PS/2 keyboard to SPI interface
* Version 0.5.0 - Added 1bpp graphics mode.
* Version 0.4.0 - Added PS/2 keyboard support.
* Version 0.3.0 - Backspace works.
* Version 0.2.0 - Switch to a text buffer to save RAM. Basic animations work.
* Version 0.1.0 - First release. VGA output works but menu is full of dummy commands and there's no keyboard input.

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
