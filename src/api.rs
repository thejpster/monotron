use crate::{Context, Input, FRAMEBUFFER};
use cortex_m::asm;
use crate::fb::{BaseConsole, AsciiConsole, Position, Row, Col, TEXT_MAX_COL, TEXT_MAX_ROW};

/// Print a null-terminated 8-bit string, in Code Page 850, to the screen.
/// Escape sequences are handled by the `vga-framebuffer` crate, but they include:
///
/// * `\n`    - Move to start of next line,
/// * `\r`    - Move to start of current line,
/// * `\t`    - Move to next tab stop (9 characters per tab stop),
/// * `0x08`  - Move backwards one character.
/// * `0x1b`  - aka `ESC`. Escapes the next character.
/// * `ESC W` - set the foreground colour for subsequent characters to White.
/// * `ESC C` - set the foreground colour for subsequent characters to Cyan.
/// * `ESC M` - set the foreground colour for subsequent characters to Magenta.
/// * `ESC Y` - set the foreground colour for subsequent characters to Yellow.
/// * `ESC R` - set the foreground colour for subsequent characters to Red.
/// * `ESC G` - set the foreground colour for subsequent characters to Green.
/// * `ESC B` - set the foreground colour for subsequent characters to Blue.
/// * `ESC K` - set the foreground colour for subsequent characters to Black.
/// * `ESC w` - set the background colour for subsequent characters to White.
/// * `ESC c` - set the background colour for subsequent characters to Cyan.
/// * `ESC m` - set the background colour for subsequent characters to Magenta.
/// * `ESC y` - set the background colour for subsequent characters to Yellow.
/// * `ESC r` - set the background colour for subsequent characters to Red.
/// * `ESC g` - set the background colour for subsequent characters to Green.
/// * `ESC b` - set the background colour for subsequent characters to Blue.
/// * `ESC k` - set the background colour for subsequent characters to Black.
/// * `ESC z` - clear the screen.
///
/// The screen will automatically scroll when you get to the bottom.
pub(crate) extern "C" fn puts(_raw_ctx: *mut Context, s: *const u8) -> i32 {
    let mut i = 0;
    unsafe {
        while *s.offset(i) != 0 {
            let ch: u8 = *s.offset(i);
            FRAMEBUFFER.write_character(ch).unwrap();
            i += 1;
        }
    }
    0
}

/// Print a single 8-bit character, in Code Page 850, to the screen. See
/// `puts` for details.
pub(crate) extern "C" fn putchar(_raw_ctx: *mut Context, ch: u8) -> i32 {
    unsafe { FRAMEBUFFER.write_character(ch).unwrap() };
    ch as i32
}

/// Read an 8-bit character from standard input (which may be UART or may be
/// the keyboard). If there is no character waiting, this routine will block
/// until one arrives. Call `kbhit()` to check first if you want to avoid
/// blocking.
///
/// TODO: Currently UTF-8 input is passed through unchanged and there's no
/// keyboard support.
pub(crate) extern "C" fn readc(raw_ctx: *mut Context) -> i32 {
    let ctx = unsafe {
        &mut *raw_ctx
    };
    loop {
        match ctx.read() {
            None => {
                asm::wfi();
            }
            Some(Input::Unicode(_unicode_char)) => {
                // TODO: Handle keyboard input
                asm::wfi();
            }
            Some(Input::Special(_scancode)) => {
                // TODO: Handle keyboard input
                asm::wfi();
            }
            Some(Input::Utf8(ch)) => {
                return ch as i32;
            }
        }
    }
}

/// Wait For Vertical Blanking Interval. Spins until the current frame has
/// completed drawing. You then have a brief period of time to do some work in
/// the frame buffer before we start drawing the next frame.
///
/// Also useful for pausing for up to 1/60th of a second.
pub(crate) extern "C" fn wfvbi(_raw_ctx: *mut Context) {
    let old_frame = unsafe { FRAMEBUFFER.frame() };
    loop {
        asm::wfi();
        let new_frame = unsafe { FRAMEBUFFER.frame() };
        if new_frame != old_frame {
            break;
        }
    }
}

/// Returns 1 if there is a character in the input buffer (i.e. a key has been
/// pressed), and returns 0 otherwise.
pub(crate) extern "C" fn kbhit(raw_ctx: *mut Context) -> i32 {
    let ctx = unsafe {
        &mut *raw_ctx
    };
    ctx.has_char() as i32
}

/// Set the screen position for the cursor.
///
/// Monotron has 48 visible columns (numbered 0..47) and 36 visible rows
/// (numbered 0..35). If either `row` or `col` are out of bounds, the call is
/// ignored.
pub(crate) extern "C" fn move_cursor(_raw_ctx: *mut Context, row: u8, col: u8) {
    if col as usize <= TEXT_MAX_COL {
        if row as usize <= TEXT_MAX_ROW {
            let p = Position::new(Row(row), Col(col));
            unsafe { let _ = FRAMEBUFFER.set_pos(p); }
        }
    }
}

/// Play a note.
///
/// `frequency` - the frequency in centi-hertz (i.e. 100_000 = 1 kHz).
/// `channel` - the channel number (0, 1 or 2).
/// `waveform` - the waveform to play (0 for square, 1 for sine, 2 for sawtooth, 3 for noise).
/// `volume` - the volume to use (0..255).
/// Returns 0 on success, anything else on error.
pub(crate) extern "C" fn play(_raw_ctx: *mut Context, frequency: u32, channel: u8, waveform: u8, volume: u8) -> i32 {
    use monotron_synth::*;
    let frequency = Frequency::from_centi_hertz(frequency);
    let channel = match channel {
        0 => Channel::Channel0,
        1 => Channel::Channel1,
        2 => Channel::Channel2,
        _ => {
            return -1;
        }
    };
    let waveform = match waveform {
        0 => Waveform::Square,
        1 => Waveform::Sine,
        2 => Waveform::Sawtooth,
        3 => Waveform::Noise,
        _ => {
            return -1;
        }
    };

    unsafe {
        crate::G_SYNTH.play(channel, frequency, volume, waveform);
    }

    0
}
