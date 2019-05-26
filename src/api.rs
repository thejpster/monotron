use crate::fb::{AsciiConsole, BaseConsole, Col, Position, Row, TEXT_MAX_COL, TEXT_MAX_ROW};
use crate::{Context, Input, FRAMEBUFFER};
use cortex_m::asm;

/// The ABI we expose to the user applications.
///
/// MS-DOS uses this set of syscalls:
///
/// * 00h - Program terminate; 1.0+
/// * 01h - Character input; 1.0+
/// * 02h - Character output; 1.0+
/// * 03h - Auxiliary input; 1.0+
/// * 04h - Auxiliary output; 1.0+
/// * 05h - Printer output; 1.0+
/// * 06h - Direct console I/O; 1.0+
/// * 07h - Direct console input without echo; 1.0+
/// * 08h - Console input without echo; 1.0+
/// * 09h - Display string; 1.0+
/// * 0Ah - Buffered keyboard input; 1.0+
/// * 0Bh - Get input status; 1.0+
/// * 0Ch - Flush input buffer and input; 1.0+
/// * 0Dh - Disk reset; 1.0+
/// * 0Eh - Set default drive; 1.0+
/// * 0Fh - Open file; 1.0+
/// * 10h - Close file; 1.0+
/// * 11h - Find first file; 1.0+
/// * 12h - Find next file; 1.0+
/// * 13h - Delete file; 1.0+
/// * 14h - Sequential read; 1.0+
/// * 15h - Sequential write; 1.0+
/// * 16h - Create or truncate file; 1.0+
/// * 17h - Rename file; 1.0+
/// * 18h - Reserved; 1.0+
/// * 19h - Get default drive; 1.0+
/// * 1Ah - Set disk transfer address; 1.0+
/// * 1Bh - Get allocation info for default drive; 1.0+
/// * 1Ch - Get allocation info for specified drive; 1.0+
/// * 1Dh - Reserved; 1.0+
/// * 1Eh - Reserved; 1.0+
/// * 1Fh - Get disk parameter block for default drive; 1.0+
/// * 20h - Reserved; 1.0+
/// * 21h - Random read; 1.0+
/// * 22h - Random write; 1.0+
/// * 23h - Get file size in records; 1.0+
/// * 24h - Set random record number; 1.0+
/// * 25h - Set interrupt vector; 1.0+
/// * 26h - Create PSP; 1.0+
/// * 27h - Random block read; 1.0+
/// * 28h - Random block write; 1.0+
/// * 29h - Parse filename; 1.0+
/// * 2Ah - Get date; 1.0+
/// * 2Bh - Set date; 1.0+
/// * 2Ch - Get time; 1.0+
/// * 2Dh - Set time; 1.0+
/// * 2Eh - Set verify flag; 1.0+
/// * 2Fh - Get disk transfer address; 2.0+
/// * 30h - Get DOS version; 2.0+
/// * 31h - Terminate and stay resident; 2.0+
/// * 32h - Get disk parameter block for specified drive; 2.0+
/// * 33h - Get or set Ctrl-Break; 2.0+
/// * 34h - Get InDOS flag pointer; 2.0+
/// * 35h - Get interrupt vector; 2.0+
/// * 36h - Get free disk space; 2.0+
/// * 37h - Get or set switch character; 2.0+
/// * 38h - Get or set country info; 2.0+
/// * 39h - Create subdirectory; 2.0+
/// * 3Ah - Remove subdirectory; 2.0+
/// * 3Bh - Change current directory; 2.0+
/// * 3Ch - Create or truncate file; 2.0+
/// * 3Dh - Open file; 2.0+
/// * 3Eh - Close file; 2.0+
/// * 3Fh - Read file or device; 2.0+
/// * 40h - Write file or device; 2.0+
/// * 41h - Delete file; 2.0+
/// * 42h - Move file pointer; 2.0+
/// * 43h - Get or set file attributes; 2.0+
/// * 44h - I/O control for devices; 2.0+
/// * 45h - Duplicate handle; 2.0+
/// * 46h - Redirect handle; 2.0+
/// * 47h - Get current directory; 2.0+
/// * 48h - Allocate memory; 2.0+
/// * 49h - Release memory; 2.0+
/// * 4Ah - Reallocate memory; 2.0+
/// * 4Bh - Execute program; 2.0+
/// * 4Ch - Terminate with return code; 2.0+
/// * 4Dh - Get program return code; 2.0+
/// * 4Eh - Find first file; 2.0+
/// * 4Fh - Find next file; 2.0+
/// * 50h - Set current PSP; 2.0+
/// * 51h - Get current PSP; 2.0+
/// * 52h - Get DOS internal pointers (SYSVARS; 2.0+
/// * 53h - Create disk parameter block; 2.0+
/// * 54h - Get verify flag; 2.0+
/// * 55h - Create program PSP; 2.0+
/// * 56h - Rename file; 2.0+
/// * 57h - Get or set file date and time; 2.0+
/// * 58h - Get or set allocation strategy; 2.11+
/// * 59h - Get extended error info; 3.0+
/// * 5Ah - Create unique file; 3.0+
/// * 5Bh - Create new file; 3.0+
/// * 5Ch - Lock or unlock file; 3.0+
/// * 5Dh - File sharing functions; 3.0+
/// * 5Eh - Network functions; 3.0+
/// * 5Fh - Network redirection functions; 3.0+
/// * 60h - Qualify filename; 3.0+
/// * 61h - Reserved; 3.0+
/// * 62h - Get current PSP; 3.0+
/// * 63h - Get DBCS lead byte table pointer; 3.0+
/// * 64h - Set wait for external event flag; 3.2+
/// * 65h - Get extended country info; 3.3+
/// * 66h - Get or set code page; 3.3+
/// * 67h - Set handle count; 3.3+
/// * 68h - Commit file; 3.3+
/// * 69h - Get or set media id; 4.0+
/// * 6Ah - Commit file; 4.0+
/// * 6Bh - Reserved; 4.0+
/// * 6Ch - Extended open/create file; 4.0+
///
/// Removable devices don't have partitions and are mapped as A: and B:.
/// Fixed devices are scanned for partitions on startup and mapped as C: onwards.
///
/// POSIX uses this set of APIs:
///
/// * char *getcwd(char *buf, size_t size); - get current working directory
/// * int mkdir(const char *pathname, mode_t mode); - create a directory
/// * int rmdir(const char *pathname); - delete a directory
/// * int chdir(const char *path); - change working directory
/// * int link(const char *oldpath, const char *newpath); - make a new name for a file
/// * int unlink(const char *pathname); - delete a name and possibly the file it refers to
/// * int rename(const char *oldpath, const char *newpath); - change the name or location of a file
/// * int stat(const char *file_name, struct stat *buf); - get file status
/// * int chmod(const char *path, mode_t mode); - change permissions of a file
/// * int chown(const char *path, uid_t owner, gid_t group); - change ownership of a file
/// * int utime(const char *filename, struct utimbuf *buf); - change access and/or modification times of an inode
/// * DIR *opendir(const char *name); - open a directory
/// * struct dirent *readdir(DIR *dir); - read directory entry
/// * int closedir(DIR *dir); - close a directory
/// * void rewinddir(DIR *dir); - reset directory stream
/// * int access(const char *pathname, int mode); - check user's permissions for a file
/// * int open(const char *pathname, int flags); - open and possibly create a file or device
/// * int creat(const char *pathname, mode_t mode); - open and possibly create a file or device
/// * int close(int fd); - close a file descriptor
/// * ssize_t read(int fd, void *buf, size_t count); - read from a file descriptor
/// * ssize_t write(int fd, const void *buf, size_t count); - write to a file descriptor
/// * int fcntl(int fd, int cmd); - manipulate file descriptor
/// * int fstat(int filedes, struct stat *buf); - get file status
/// * off_t lseek(int fildes, off_t offset, int whence); - reposition read/write file offset
/// * int dup(int oldfd); - duplicate a file descriptor
/// * int dup2(int oldfd, int newfd); - duplicate a file descriptor
/// * int pipe(int filedes[2]); - create pipe
/// * int mkfifo(const char *pathname, mode_t mode ); - make a FIFO special file (a named pipe)
/// * mode_t umask(mode_t mask); - set file creation mask
/// * FILE *fdopen(int fildes, const char *mode); - associate a stream with an existing file descriptor
/// * int fileno(FILE *stream); - return file descriptor of stream
///
/// The C standard library offers:
///
/// | Function(s) | Description |
/// |-------------|-------------|
/// | fopen | Opens a file (with a non-Unicode filename on Windows and possible UTF-8 filename on Linux) |
/// | freopen | Opens a different file with an existing stream |
/// | fflush | Synchronizes an output stream with the actual file |
/// | fclose | Closes a file |
/// | setbuf | Sets the buffer for a file stream |
/// | setvbuf | Sets the buffer and its size for a file stream |
/// | fwide | Switches a file stream between wide-character I/O and narrow-character I/O |
/// | fread | Reads from a file |
/// | fwrite | Writes to a file |
/// | scanf/fscanf/sscanf | Reads formatted byte input from stdin, a file stream or a buffer |
/// | vscanf/vfscanf/vsscanf | Reads formatted input byte from stdin, a file stream or a buffer using variable argument list |
/// | printf/fprintf/sprintf/snprintf | Prints formatted byte output to stdout, a file stream or a buffer |
/// | vprintf/vfprintf/vsprintf/vsnprintf | Prints formatted byte output to stdout, a file stream, or a buffer using variable argument list |
/// | perror | Writes a description of the current error to stderr  |
/// | fgetc/getc | Reads a byte from a file stream |
/// | fgets | Reads a byte line from a file stream |
/// | fput/cputc | Writes a byte to a file stream |
/// | fputs | Writes a byte string to a file stream |
/// | getchar | Reads a byte from stdin |
/// | putchar | Writes a byte to stdout |
/// | puts | Writes a byte string to stdout |
/// | ungetc | Puts a byte back into a file stream  |
/// | ftell | Returns the current file position indicator |
/// | fseek | Moves the file position indicator to a specific location in a file |
/// | fgetpos | Gets the file position indicator |
/// | fsetpos | Moves the file position indicator to a specific location in a file |
/// | rewind | Moves the file position indicator to the beginning in a file |
/// | clearerr | Clears errors |
/// | feof | Checks for the end-of-file |
/// | ferror | Checks for a file error |
/// | remove | Erases a file |
/// | rename | Renames a file |
/// | tmpfile | Returns a pointer to a temporary file |
/// | tmpnam | Returns a unique filename  |

///
/// The problem with the MS-DOS API is it doesn't work well with removable,
/// partitioned, devices. Also, I don't think I can reliably detect card
/// removal. The problem with the POSIX API is that it is very C-like, with
/// null-terminated strings and so on. I would like to use bounded strings,
/// which better suit a Rust slice. Both MS-DOS and POSIX have an 'everything
/// is a file' (or at least, 'most things are files') approach - for example,
/// the MS-DOS reserved filenames for AUX, NUL, etc.

#[repr(C)]
pub(crate) struct Table {
    putchar: extern "C" fn(*mut Context, u8) -> i32,
    puts: extern "C" fn(*mut Context, *const u8) -> i32,
    readc: extern "C" fn(*mut Context) -> i32,
    wfvbi: extern "C" fn(*mut Context),
    kbhit: extern "C" fn(*mut Context) -> i32,
    move_cursor: extern "C" fn(*mut Context, u8, u8),
    play: extern "C" fn(*mut Context, u32, u8, u8, u8) -> i32,
    change_font: extern "C" fn(*mut Context, u32, *const u8),
    get_joystick: extern "C" fn(*mut Context) -> u8,
    set_cursor_visible: extern "C" fn(*mut Context, u8),
}

pub(crate) static CALLBACK_TABLE: Table = Table {
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
    let ctx = unsafe { &mut *raw_ctx };
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
    let ctx = unsafe { &mut *raw_ctx };
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
            unsafe {
                let _ = FRAMEBUFFER.set_pos(p);
            }
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
pub(crate) extern "C" fn play(
    _raw_ctx: *mut Context,
    frequency: u32,
    channel: u8,
    waveform: u8,
    volume: u8,
) -> i32 {
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
pub(crate) extern "C" fn change_font(_raw_ctx: *mut Context, mode: u32, p_font: *const u8) {
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
pub(crate) extern "C" fn get_joystick(raw_ctx: *mut Context) -> u8 {
    let ctx = unsafe { &mut *raw_ctx };
    ctx.joystick.get_state().as_u8()
}

/// Change whether the cursor is visible
pub(crate) extern "C" fn set_cursor_visible(_raw_ctx: *mut Context, visible: u8) {
    unsafe {
        FRAMEBUFFER.set_cursor_visible(visible != 0);
    }
}
