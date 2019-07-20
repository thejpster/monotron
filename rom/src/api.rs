use crate::fb::{AsciiConsole, BaseConsole, Col, Position, Row, TEXT_MAX_COL, TEXT_MAX_ROW};
use crate::hal::time::U32Ext;
use crate::GLOBAL_CONTEXT;
use crate::{Input, FRAMEBUFFER};
use cortex_m::asm;
pub use monotron_api::*;

const UART0_HANDLE: Handle = Handle(100);

pub(crate) static CALLBACK_TABLE: Api = Api {
    putchar,
    puts,
    readc,
    wfvbi,
    kbhit,
    move_cursor,
    play,
    change_font,
    get_joystick,
    set_cursor_visible,
    read_char_at,
    open,
    close,
    read,
    write,
    write_then_read,
    seek,
    opendir,
    readdir,
    stat,
    gettime,
    puts_utf8,
};

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
/// * `ESC Z` - clear the screen.
///
/// The screen will automatically scroll when you get to the bottom.
pub(crate) extern "C" fn puts(s: *const u8) -> i32 {
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
pub(crate) extern "C" fn putchar(ch: u8) -> i32 {
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
pub(crate) extern "C" fn readc() -> i32 {
    let mut lock = GLOBAL_CONTEXT.lock();
    let ctx = lock.as_mut().unwrap();
    loop {
        match ctx.read() {
            None => {
                asm::wfi();
            }
            Some(Input::Special(_scancode)) => {
                // TODO: Handle keyboard input
                asm::wfi();
            }
            Some(Input::Cp850(ch)) => {
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
pub(crate) extern "C" fn wfvbi() {
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
pub(crate) extern "C" fn kbhit() -> i32 {
    let mut lock = GLOBAL_CONTEXT.lock();
    let ctx = lock.as_mut().unwrap();
    ctx.has_char() as i32
}

/// Set the screen position for the cursor.
///
/// Monotron has 48 visible columns (numbered 0..47) and 36 visible rows
/// (numbered 0..35). If either `row` or `col` are out of bounds, the call is
/// ignored.
pub(crate) extern "C" fn move_cursor(row: u8, col: u8) {
    if col as usize <= TEXT_MAX_COL {
        if row as usize <= TEXT_MAX_ROW {
            let p = Position::new(Row(row), Col(col));
            unsafe {
                let _ = FRAMEBUFFER.set_pos(p);
            }
        }
    }
}

/// Get the current time.
///
/// The system has no concept of timezones or leap seconds. We get the
/// calendar time from the RTC on start up, then rely on a timer tick to keep
/// the calendar updated.
///
/// TODO: Actually get the RTC time on startup
pub(crate) extern "C" fn gettime() -> monotron_api::Timestamp {
    crate::TIME_CONTEXT.get_timestamp()
}

/// Write a UTF-8 string.
pub(crate) extern "C" fn puts_utf8(string: *const u8, length: usize) {
    use core::fmt::Write as _;
    unsafe {
        FRAMEBUFFER
            .write_str(core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                string, length,
            )))
            .unwrap();
    }
}

/// Play a note.
///
/// `frequency` - the frequency in centi-hertz (i.e. 100_000 = 1 kHz).
/// `channel` - the channel number (0, 1 or 2).
/// `waveform` - the waveform to play (0 for square, 1 for sine, 2 for sawtooth, 3 for noise).
/// `volume` - the volume to use (0..255).
/// Returns 0 on success, anything else on error.
pub(crate) extern "C" fn play(frequency: u32, channel: u8, waveform: u8, volume: u8) -> i32 {
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

/// Set the system font.
///
/// 0 sets it to the normal font (CodePage 850)
/// 1 sets it to the Teletext font.
/// 2 sets it to the given custom font.
///
/// The second argument is only valid if the first argument is 2,
/// and it must be a pointer to an array of 4096 bytes, with static lifetime.
pub(crate) extern "C" fn change_font(mode: u32, p_font: *const u8) {
    let new_font = match mode {
        0 => Some(None),
        1 => Some(Some(&vga_framebuffer::freebsd_teletext::FONT_DATA[..])),
        2 if !p_font.is_null() => {
            let font_data: &'static [u8] = unsafe { core::slice::from_raw_parts(p_font, 4096) };
            Some(Some(font_data))
        }
        _ => None,
    };
    if let Some(f) = new_font {
        unsafe {
            FRAMEBUFFER.set_custom_font(f);
        }
    }
}

/// Get the joystick state
pub(crate) extern "C" fn get_joystick() -> u8 {
    GLOBAL_CONTEXT
        .lock()
        .as_ref()
        .unwrap()
        .joystick
        .get_state()
        .as_u8()
}

/// Change whether the cursor is visible
pub(crate) extern "C" fn set_cursor_visible(visible: u8) {
    unsafe {
        FRAMEBUFFER.set_cursor_visible(visible != 0);
    }
}

/// Return what's on the screen at this point
pub(crate) extern "C" fn read_char_at(row: u8, col: u8) -> u16 {
    let p = Position::new(Row(row), Col(col));
    if let Some((glyph, attr)) = unsafe { FRAMEBUFFER.read_glyph_at(p) } {
        (((glyph as u8) as u16) << 8) + attr.as_u8() as u16
    } else {
        0
    }
}

/// Open/create a device/file. Returns a file handle, or an error.
pub(crate) extern "C" fn open(filename: BorrowedString, _mode: OpenMode) -> HandleResult {
    if filename == BorrowedString::new("/dev/uart0@9600") {
        let mut lock = GLOBAL_CONTEXT.lock();
        let ctx = lock.as_mut().unwrap();
        ctx.rs232_uart.change_baud_rate(9600u32.bps(), &ctx.clocks);
        HandleResult::Ok(UART0_HANDLE)
    } else if filename == BorrowedString::new("/dev/uart0@115200") {
        let mut lock = GLOBAL_CONTEXT.lock();
        let ctx = lock.as_mut().unwrap();
        ctx.rs232_uart
            .change_baud_rate(115200u32.bps(), &ctx.clocks);
        HandleResult::Ok(UART0_HANDLE)
    } else {
        HandleResult::Error(Error::FileNotFound)
    }
}

/// Close a previously opened handle.
pub(crate) extern "C" fn close(handle: Handle) -> EmptyResult {
    if handle == UART0_HANDLE {
        EmptyResult::Ok
    } else {
        EmptyResult::Error(Error::BadFileHandle)
    }
}

/// Read from a file handle into the given buffer. Returns an error, or
/// the number of bytes read (which may be less than `buffer_len`).
pub(crate) extern "C" fn read(
    handle: Handle,
    buffer_ptr: *mut u8,
    buffer_len: usize,
) -> SizeResult {
    if handle == UART0_HANDLE {
        use embedded_hal::serial::Read;
        let buffer = unsafe { core::slice::from_raw_parts_mut(buffer_ptr, buffer_len) };
        let mut read = 0;
        let mut lock = GLOBAL_CONTEXT.lock();
        let ctx = lock.as_mut().unwrap();
        while read < buffer.len() {
            match ctx.rs232_uart.read() {
                Ok(ch) => {
                    buffer[read] = ch;
                    read += 1;
                }
                Err(nb::Error::WouldBlock) => {
                    break;
                }
                Err(_e) => {
                    return SizeResult::Error(Error::IOError);
                }
            }
        }
        SizeResult::Ok(read)
    } else {
        SizeResult::Error(Error::BadFileHandle)
    }
}

/// Write the contents of the given buffer to a file handle. Returns an
/// error, or the number of bytes written (which may be less than
/// `buffer_len`).
pub(crate) extern "C" fn write(
    handle: Handle,
    buffer_ptr: *const u8,
    buffer_len: usize,
) -> SizeResult {
    if handle == UART0_HANDLE {
        use embedded_hal::serial::Write;
        let buffer = unsafe { core::slice::from_raw_parts(buffer_ptr, buffer_len) };
        let mut written = 0;
        let mut lock = GLOBAL_CONTEXT.lock();
        let ctx = lock.as_mut().unwrap();
        while written < buffer.len() {
            let ch = buffer[written];
            match ctx.rs232_uart.write(ch) {
                Ok(_) => {
                    written += 1;
                }
                Err(nb::Error::WouldBlock) => {
                    break;
                }
                Err(_e) => {
                    return SizeResult::Error(Error::IOError);
                }
            }
        }
        SizeResult::Ok(written)
    } else {
        SizeResult::Error(Error::BadFileHandle)
    }
}

/// Write to the handle and the read from the handle. Useful when doing an
/// I2C read of a specific address. It is an error if the complete
/// `out_buffer` could not be written.
pub(crate) extern "C" fn write_then_read(
    _handle: Handle,
    _out_buffer: *const u8,
    _out_buffer_len: usize,
    _in_buffer: *mut u8,
    _in_buffer_len: usize,
) -> SizeResult {
    unimplemented!();
}

/// Move the read/write pointer in a file.
pub(crate) extern "C" fn seek(_handle: Handle, _offset: Offset) -> EmptyResult {
    unimplemented!();
}

/// Open a directory. Returns a file handle, or an error.
pub(crate) extern "C" fn opendir(_filename: BorrowedString) -> HandleResult {
    unimplemented!();
}

/// Read directory entry into given buffer.
pub(crate) extern "C" fn readdir(_handle: Handle, _dir_entry: &mut DirEntry) -> EmptyResult {
    unimplemented!();
}

/// Get information about a file by path
pub(crate) extern "C" fn stat(
    _filename: BorrowedString,
    _stat_entry: &mut DirEntry,
) -> EmptyResult {
    unimplemented!();
}

// End of file
