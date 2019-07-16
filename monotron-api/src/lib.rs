//! # Monotron API
//!
//! This crate contains the Userspace to Kernel API for the Monotron.
//!
//! It is pulled in by the Kernel (github.com/thejpster/monotron) and the
//! various user-space example applications
//! (github.com/thejpster/monotron-apps).
//!
//! The API in here is modelled after both the UNIX/POSIX API and the MS-DOS
//! API. We use function pointers rather than `SWI` calls (software
//! interrupts), provided in a structure. This structure is designed to be
//! extensible.
//!
//! A C header file version of this API can be generated with `cbindgen`.
//!
//! All types in this file must be `#[repr(C)]`.
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

/// The set of Error codes the API can report.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// The given filename was not found
    FileNotFound,
    /// The given file handle was not valid
    BadFileHandle,
    /// You can't do that operation on that sort of file
    NotSupported,
    /// An unknown error occured
    Unknown = 0xFFFF,
}

/// Describes a handle to some resource.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Handle(pub u16);

/// Describes a string of fixed length, which must not be free'd by the
/// recipient. The given length must not include any null terminators that may
/// be present. The string must be valid UTF-8 (or 7-bit ASCII, which is a
/// valid subset of UTF-8).
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BorrowedString {
    /// The start of the string
    pub ptr: *const u8,
    /// The length of the string in bytes
    pub length: usize,
}

/// Describes the result of a function which may return a `Handle` if
/// everything was Ok, or return an `Error` if something went wrong.
///
/// This is not a standard Rust `Result` because they are not `#[repr(C)]`.
#[repr(C)]
#[derive(Debug)]
pub enum HandleResult {
    /// Success - a handle is returned
    Ok(Handle),
    /// Failure - an error is returned
    Error(Error),
}

/// Describes the result of a function which may return nothing if everything
/// was Ok, or return an `Error` if something went wrong.
///
/// This is not a standard Rust `Result` because they are not `#[repr(C)]`.
#[repr(C)]
#[derive(Debug)]
pub enum EmptyResult {
    /// Success - nothing is returned
    Ok(u8),
    /// Failure - an error is returned
    Error(Error),
}

/// Describes the result of a function which may return a numeric count of
/// bytes read/written if everything was Ok, or return an `Error` if something
/// went wrong.
///
/// This is not a standard Rust `Result` because they are not `#[repr(C)]`.
#[repr(C)]
#[derive(Debug)]
pub enum SizeResult {
    /// Success - a size in bytes is returned
    Ok(usize),
    /// Failure - an error is returned
    Error(Error),
}

/// Describes the sort of files you will find in the system-wide virtual
/// filesystem. Some exist on disk, and some do not.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileType {
    /// A regular file
    File,
    /// A directory contains other files and directories
    Directory,
    /// A device you can read/write a block (e.g. 512 bytes) at a time
    BlockDevice,
    /// A device you can read/write one or more bytes at a time
    CharDevice,
}

/// Describes an instant in time. The system only supports local time and has
/// no concept of time zones.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timestamp {
    /// The Gregorian calendar year, minus 1970 (so 10 is 1980, and 30 is the year 2000)
    pub year_from_1970: u8,
    /// The month of the year, where January is 1 and December is 12
    pub month: u8,
    /// The day of the month where 1 is the first of the month, through to 28,
    /// 29, 30 or 31 (as appropriate)
    pub day: u8,
    /// The hour in the day, from 0 to 23
    pub hour: u8,
    /// The minutes past the hour, from 0 to 59
    pub minute: u8,
    /// The seconds past the minute, from 0 to 59. Note that some filesystems
    /// only have 2-second precision on their timestamps.
    pub second: u8,
}

/// Represents the seven days of the week
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DayOfWeek {
    /// First day of the week
    Monday,
    /// Comes after Monday
    Tuesday,
    /// Middle of the week
    Wednesday,
    /// Between Wednesday and Friday
    Thursday,
    /// Almost the weekend
    Friday,
    /// First day of the weekend
    Saturday,
    /// Last day of the week
    Sunday,
}

impl DayOfWeek {
    /// Returns the UK English word for the day of the week
    pub fn day_str(&self) -> &'static str {
        match self {
            DayOfWeek::Monday => "Monday",
            DayOfWeek::Tuesday => "Tuesday",
            DayOfWeek::Wednesday => "Wednesday",
            DayOfWeek::Thursday => "Thursday",
            DayOfWeek::Friday => "Friday",
            DayOfWeek::Saturday => "Saturday",
            DayOfWeek::Sunday => "Sunday",
        }
    }
}

impl Timestamp {
    /// Returns the day of the week for the given timestamp.
    pub fn day_of_week(&self) -> DayOfWeek {
        let zellers_month = ((i32::from(self.month) + 9) % 12) + 1;
        let k = i32::from(self.day);
        let year = if zellers_month >= 11 {
            i32::from(self.year_from_1970) + 1969
        } else {
            i32::from(self.year_from_1970) + 1970
        };
        let d = year % 100;
        let c = year / 100;
        let f = k + (((13 * zellers_month) - 1) / 5) + d + (d / 4) + (c / 4) - (2 * c);
        let day_of_week = f % 7;
        match day_of_week {
            0 => DayOfWeek::Sunday,
            1 => DayOfWeek::Monday,
            2 => DayOfWeek::Tuesday,
            3 => DayOfWeek::Wednesday,
            4 => DayOfWeek::Thursday,
            5 => DayOfWeek::Friday,
            _ => DayOfWeek::Saturday,
        }
    }

    /// Returns the current month as a UK English string (e.g. "August").
    pub fn month_str(&self) -> &'static str {
        match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }
}

/// Describes a file as it exists on disk.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// The file of the file this entry represents
    pub file_type: FileType,
    /// The name of the file (not including its full path)
    pub name: [u8; 11],
    /// The sie of the file in bytes
    pub size: u32,
    /// When this file was last modified
    pub mtime: Timestamp,
    /// When this file was created
    pub ctime: Timestamp,
    /// The various mode bits set on this file
    pub mode: FileMode,
}

/// A bitfield indicating if a file is:
///
/// * read-only
/// * a volume label
/// * a system file
/// * in need of archiving
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FileMode(u8);

/// Is the read-only bit set in this FileMode bit-field?
#[no_mangle]
pub extern "C" fn monotron_filemode_is_readonly(flags: FileMode) -> bool {
    (flags.0 & FileMode::READ_ONLY) != 0
}

/// Is the volume label bit set in this FileMode bit-field?
#[no_mangle]
pub extern "C" fn monotron_filemode_is_volume(flags: FileMode) -> bool {
    (flags.0 & FileMode::VOLUME) != 0
}

/// Is the system bit set in this FileMode bit-field?
#[no_mangle]
pub extern "C" fn monotron_filemode_is_system(flags: FileMode) -> bool {
    (flags.0 & FileMode::SYSTEM) != 0
}

/// Is the archive bit set in this FileMode bit-field?
#[no_mangle]
pub extern "C" fn monotron_filemode_is_archive(flags: FileMode) -> bool {
    (flags.0 & FileMode::ARCHIVE) != 0
}

impl FileMode {
    const READ_ONLY: u8 = 1;
    const VOLUME: u8 = 2;
    const SYSTEM: u8 = 4;
    const ARCHIVE: u8 = 8;
}

/// Represents how far to move the current read/write pointer through a file.
/// You can specify the position as relative to the start of the file,
/// relative to the end of the file, or relative to the current pointer
/// position.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offset {
    /// Set the pointer to this many bytes from the start of the file
    FromStart(u32),
    /// Set the pointer to this many bytes from the current position (+ve is forwards, -ve is backwards)
    FromCurrent(i32),
    /// Set the pointer to this many bytes back from the end of the file
    FromEnd(u32),
}

/// The ways in which we can open a file.
///
/// TODO: Replace all these booleans with a u8 flag-set
#[repr(C)]
pub enum OpenMode {
    /// Open file in read-only mode. No writes allowed. One file can be opened in read-only mode multiple times.
    ReadOnly {
        /// Set to true if read/write requests on this handle should be non-blocking
        non_blocking: bool,
    },
    /// Open a file for writing, but not reading.
    WriteOnly {
        /// If true, the write pointer will default to the end of the file
        append: bool,
        /// If true, the file will be created if it doesn't exist. If false, the file must exist. See also the `exclusive` flag.
        create: bool,
        /// If true AND the create flag is true, the open will fail if the file already exists.
        exclusive: bool,
        /// If true, the file contents will be deleted on open, giving a zero byte file.
        truncate: bool,
        /// Set to true if read/write requests on this handle should be non-blocking
        non_blocking: bool,
    },
    /// Open a file for reading and writing.
    ReadWrite {
        /// If true, the write pointer will default to the end of the file
        append: bool,
        /// If true, the file will be created if it doesn't exist. If false, the file must exist. See also the `exclusive` flag.
        create: bool,
        /// If true AND the create flag is true, the open will fail if the file already exists.
        exclusive: bool,
        /// If true, the file contents will be deleted on open, giving a zero byte file.
        truncate: bool,
        /// Set to true if read/write requests on this handle should be non-blocking
        non_blocking: bool,
    },
}

/// Create a new Read Only open mode object, for passing to the `open` syscall.
#[no_mangle]
pub extern "C" fn monotron_openmode_readonly(non_blocking: bool) -> OpenMode {
    OpenMode::ReadOnly { non_blocking }
}

/// Create a new Write Only open mode object, for passing to the `open` syscall.
#[no_mangle]
pub extern "C" fn monotron_openmode_writeonly(
    append: bool,
    create: bool,
    exclusive: bool,
    truncate: bool,
    non_blocking: bool,
) -> OpenMode {
    OpenMode::WriteOnly {
        append,
        create,
        exclusive,
        truncate,
        non_blocking,
    }
}

/// Create a new Read Write open mode object, for passing to the `open` syscall.
#[no_mangle]
pub extern "C" fn monotron_openmode_readwrite(
    append: bool,
    create: bool,
    exclusive: bool,
    truncate: bool,
    non_blocking: bool,
) -> OpenMode {
    OpenMode::ReadWrite {
        append,
        create,
        exclusive,
        truncate,
        non_blocking,
    }
}

/// Standard Output
pub static STDOUT: Handle = Handle(0);

/// Standard Error
pub static STDERR: Handle = Handle(1);

/// Standard Input
pub static STDIN: Handle = Handle(2);

/// This structure contains all the function pointers the application can use
/// to access OS functions.
#[repr(C)]
pub struct Api {
    /// Old function for writing a single 8-bit character to the screen.
    pub putchar: extern "C" fn(ch: u8) -> i32,

    /// Old function for writing a null-terminated 8-bit string to the screen.
    pub puts: extern "C" fn(string: *const u8) -> i32,

    /// Old function for reading one byte from stdin, blocking.
    pub readc: extern "C" fn() -> i32,

    /// Old function for checking if readc() would block.
    pub kbhit: extern "C" fn() -> i32,

    /// Old function for moving the cursor on screen. To be replaced with ANSI
    /// escape codes.
    pub move_cursor: extern "C" fn(row: u8, col: u8),

    /// Old function for playing a note.
    pub play: extern "C" fn(frequency: u32, channel: u8, volume: u8, waveform: u8) -> i32,

    /// Old function for changing the on-screen font.
    pub change_font: extern "C" fn(font_id: u32, font_data: *const u8),

    /// Old function for reading the Joystick status.
    pub get_joystick: extern "C" fn() -> u8,

    /// Old function for turning the cursor on/off.
    pub set_cursor_visible: extern "C" fn(enabled: u8),

    /// Old function for reading the contents of the screen.
    pub read_char_at: extern "C" fn(row: u8, col: u8) -> u16,

    /// Wait for next vertical blanking interval.
    pub wfvbi: extern "C" fn(),

    /// Open/create a device/file. Returns a file handle, or an error.
    pub open: extern "C" fn(filename: BorrowedString, mode: OpenMode) -> HandleResult,

    /// Close a previously opened handle.
    pub close: extern "C" fn(handle: Handle) -> EmptyResult,

    /// Read from a file handle into the given buffer. Returns an error, or
    /// the number of bytes read (which may be less than `buffer_len`).
    pub read: extern "C" fn(handle: Handle, buffer: *mut u8, buffer_len: usize) -> SizeResult,

    /// Write the contents of the given buffer to a file handle. Returns an
    /// error, or the number of bytes written (which may be less than
    /// `buffer_len`).
    pub write: extern "C" fn(handle: Handle, buffer: *const u8, buffer_len: usize) -> SizeResult,

    /// Write to the handle and the read from the handle. Useful when doing an
    /// I2C read of a specific address. It is an error if the complete
    /// `out_buffer` could not be written.
    pub write_then_read: extern "C" fn(
        handle: Handle,
        out_buffer: *const u8,
        out_buffer_len: usize,
        in_buffer: *mut u8,
        in_buffer_len: usize,
    ) -> SizeResult,

    /// Move the read/write pointer in a file.
    pub seek: extern "C" fn(handle: Handle, offset: Offset) -> EmptyResult,

    /// Open a directory. Returns a file handle, or an error.
    pub opendir: extern "C" fn(filename: BorrowedString) -> HandleResult,

    /// Read directory entry into given buffer.
    pub readdir: extern "C" fn(handle: Handle, dir_entry: &mut DirEntry) -> EmptyResult,

    /// Get information about a file by path
    pub stat: extern "C" fn(filename: BorrowedString, stat_entry: &mut DirEntry) -> EmptyResult,

    /// Get the current time
    pub gettime: extern "C" fn() -> Timestamp,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_of_week() {
        let samples = [
            (
                Timestamp {
                    year_from_1970: 49,
                    month: 7,
                    day: 16,
                    hour: 0,
                    minute: 0,
                    second: 0,
                },
                DayOfWeek::Tuesday,
            ),
            (
                Timestamp {
                    year_from_1970: 49,
                    month: 7,
                    day: 17,
                    hour: 0,
                    minute: 0,
                    second: 0,
                },
                DayOfWeek::Wednesday,
            ),
            (
                Timestamp {
                    year_from_1970: 49,
                    month: 7,
                    day: 18,
                    hour: 0,
                    minute: 0,
                    second: 0,
                },
                DayOfWeek::Thursday,
            ),
        ];
        for (timestamp, day) in samples.iter() {
            assert_eq!(timestamp.day_of_week(), *day);
        }
    }
}
