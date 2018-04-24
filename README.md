# Monotron.

> A simple 1980's home computer style application for the Stellaris Launchpad

## Features

Monotron has a 400x600 pixel black-and-white (well, black-and-green) rendering
of a 48x36 character text buffer, scaled up to an 800x600 SVGA output at 60
Hz, courtesy of an 80 MHz CPU clock (which can provide the 20 MHz pixel clock)
and the [vga-framebuffer](https://github.com/thejpster/vga-framebuffer-rs)
crate. The video is produced using a Timer peripheral for the sync signals and
an SPI peripheral for the pixels. Just three wires and a little resisitive
divider are required.

UTF-8 encoded Unicode characters (as emmitted by `write!`) are converted to a
custom 8-bit character set for storage in the framebuffer. Eventually I hope
to standardise on something like [MS-DOS Code Page
850](https://en.wikipedia.org/wiki/Code_page_850) to give a mix of latin,
accented and graphic glyphs. The font is 8x16 pixels. There a plan to abuse
the glyph attribute field and turn each glyph octet into a 2 pixel by 4 pixel
bitmap for a Teletext-style 96x144 pixel display, and perhaps even to
dynamically switch to a full 400x300 framebuffer (which will take 15 KiB of
SRAM at 1 bit-per-pixel). We may even get full RGB 8-colour output, if I can
get three separate SPI peripherals to synchronise properly.

When running, a simple command driven interface is presented. Commands can be
entered over serial, or using a PS/2 keyboard (not yet implemented). Commands
are split on whitespace and then interpreted based on the left-most word.
Enter the command 'help' to see a list of commands. Some commands place you in
to a sub-menu - use 'exit' to return to the previous menu.

## Compiling

You will need to build using Rust Nightly, as we need various experimental features for Embedded development that are not yet available in Stable.

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

* Pin 2: Green - connect to PB7 via a resistive divider.
* Pin 5: Ground - connect to GND
* Pin 7: Green Return - connect to GND
* Pin 13: H-Sync - connect to PB6
* Pin 14: V-Sync - connect to PC4

The resistive divider needs to drop the 3.3V output down to 0.7V. Some
monitors are more tolerant of over voltage than others. The ideal resistor
values will depend upon your monitors input impedance - this is typically 75
ohms but some TFT monitors may have a much higher input impedance. In a
perfect world, your board would offer a 75 ohm source impendance, to reduce
reflections, but at this resolution it doesn't really matter, and a 75 ohm
source impedance will probably pull too much current out of the GPIO pin.

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

### PS/2 Keyboard

PS/2 keyboard support is TBD. See the [pc-keyboard](https://github.com/thejpster/pc-keyboard) crate.

### UART

Monotron uses UART0 on the Stellaris Launchpad, which is converted to USB
Serial by the on-board companion chip. Connect with your favourite Serial
terminal at 115,200bps.

## Changelog

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
