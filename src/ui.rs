use crate::fb::{self, AsciiConsole, BaseConsole};
use crate::hal::prelude::*;
use crate::{api, Context, APPLICATION_LEN, APPLICATION_START_ADDR, FRAMEBUFFER};
use core::fmt::Write;
use embedded_hal::prelude::*;
use menu;

pub(crate) type Menu<'a> = menu::Menu<'a, Context>;
pub(crate) type Item<'a> = menu::Item<'a, Context>;

static ITEM_CLEAR: Item = Item {
    item_type: menu::ItemType::Callback(item_clear),
    command: "clear",
    help: Some("Reset the display."),
};

static ITEM_PEEK: Item = Item {
    item_type: menu::ItemType::Callback(item_peek),
    command: "peek",
    help: Some("<addr> - Read a register."),
};

static ITEM_POKE: Item = Item {
    item_type: menu::ItemType::Callback(item_poke),
    command: "poke",
    help: Some("<addr> <value> - Write a register."),
};

static ITEM_DUMP: Item = Item {
    item_type: menu::ItemType::Callback(item_dump),
    command: "dump",
    help: Some("<addr> <bytes> - Dump RAM/ROM."),
};

static ITEM_LOAD: Item = Item {
    item_type: menu::ItemType::Callback(item_load_file),
    command: "load",
    help: Some("Load program from UART."),
};

static ITEM_DEBUG: Item = Item {
    item_type: menu::ItemType::Callback(item_debug_info),
    command: "debug",
    help: Some("Show some debug info."),
};

static ITEM_RUN: Item = Item {
    item_type: menu::ItemType::Callback(item_run_program),
    command: "run",
    help: Some("Run loaded program."),
};

static ITEM_BEEP: Item = Item {
    item_type: menu::ItemType::Callback(item_beep),
    command: "beep",
    help: Some("Make a beep."),
};

static ITEM_MOUNT: Item = Item {
    item_type: menu::ItemType::Callback(item_mount),
    command: "mount",
    help: Some("Mount a new SD/MMC card."),
};

static ITEM_UNMOUNT: Item = Item {
    item_type: menu::ItemType::Callback(item_unmount),
    command: "unmount",
    help: Some("Unmount an SD/MMC card."),
};

static ITEM_DIR: Item = Item {
    item_type: menu::ItemType::Callback(item_dir),
    command: "dir",
    help: Some("List the root directory"),
};

static ITEM_DLOAD: Item = Item {
    item_type: menu::ItemType::Callback(item_dload),
    command: "dload",
    help: Some("Load file from the SD-Card"),
};

static ITEM_DDUMP: Item = Item {
    item_type: menu::ItemType::Callback(item_ddump),
    command: "ddump",
    help: Some("Load and display a binary file"),
};

static ITEM_DPAGE: Item = Item {
    item_type: menu::ItemType::Callback(item_dpage),
    command: "dpage",
    help: Some("Load and display a text file"),
};

pub(crate) static ROOT_MENU: Menu = Menu {
    label: "root",
    items: &[
        &ITEM_CLEAR,
        &ITEM_PEEK,
        &ITEM_POKE,
        &ITEM_DUMP,
        &ITEM_LOAD,
        &ITEM_RUN,
        &ITEM_DEBUG,
        &ITEM_BEEP,
        &ITEM_MOUNT,
        &ITEM_UNMOUNT,
        &ITEM_DIR,
        &ITEM_DLOAD,
        &ITEM_DDUMP,
        &ITEM_DPAGE,
    ],
    entry: None,
    exit: None,
};

/// Clears the screen
fn item_clear<'a>(_menu: &Menu, _item: &Item, _input: &str, _context: &mut Context) {
    unsafe { FRAMEBUFFER.clear() };
    unsafe { FRAMEBUFFER.set_pos(fb::Position::origin()).unwrap() };
}

fn item_peek<'a>(_menu: &Menu, _item: &Item, input: &str, context: &mut Context) {
    let mut parts = input.split_whitespace();
    parts.next();
    if let Some(addr) = parts
        .next()
        .map_or(None, |p| usize::from_str_radix(p, 16).ok())
    {
        unsafe {
            let data = ::core::ptr::read_volatile(addr as *const u32);
            writeln!(context, "Addr 0x{:08x} is 0x{:08x}", addr, data).unwrap();
        }
    } else {
        writeln!(
            context,
            "Bad address {:?}. Enter hex, without the 0x prefix..",
            input
        )
        .unwrap();
    }
}

fn item_poke<'a>(_menu: &Menu, _item: &Item, input: &str, context: &mut Context) {
    let mut parts = input.split_whitespace();
    parts.next();
    if let Some(addr) = parts
        .next()
        .map_or(None, |p| usize::from_str_radix(p, 16).ok())
    {
        if let Some(value) = parts
            .next()
            .map_or(None, |p| u32::from_str_radix(p, 16).ok())
        {
            writeln!(context, "Poking 0x{:08x} to addr 0x{:08x}...", value, addr).unwrap();
            unsafe {
                ::core::ptr::write_volatile(addr as *mut u32, value);
            }
        } else {
            writeln!(context, "Missing or bad value.").unwrap();
        }
    } else {
        writeln!(context, "Missing or bad address.").unwrap();
    }
}

fn item_dump<'a>(_menu: &Menu, _item: &Item, input: &str, context: &mut Context) {
    let mut parts = input.split_whitespace();
    parts.next();
    if let Some(mut addr) = parts
        .next()
        .map_or(None, |p| usize::from_str_radix(p, 16).ok())
    {
        if let Some(count) = parts
            .next()
            .map_or(None, |p| u32::from_str_radix(p, 16).ok())
        {
            writeln!(
                context,
                "Dumping 0x{:08x} bytes from 0x{:08x}...",
                count, addr
            )
            .unwrap();
            for i in 0..count {
                let data = unsafe { ::core::ptr::read_volatile(addr as *const u8) };
                write!(context, "{:02x}", data).unwrap();
                if ((i + 1) % 4) == 0 {
                    write!(context, " ").unwrap();
                }
                if ((i + 1) % 16) == 0 {
                    write!(context, "\n").unwrap();
                }
                addr += 1;
            }
            writeln!(context, "\nDone.").unwrap();
        } else {
            writeln!(context, "Missing or bad value.").unwrap();
        }
    } else {
        writeln!(context, "Missing or bad address.").unwrap();
    }
}

/// Reads raw binary from the UART and dumps it into application RAM.
fn item_load_file<'a>(_menu: &Menu, _item: &Item, _input: &str, context: &mut Context) {
    let application_ram: &'static mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(APPLICATION_START_ADDR, APPLICATION_LEN) };
    for b in application_ram.iter_mut() {
        *b = 0x00;
    }
    writeln!(context, "Reading hex...").unwrap();
    context.usb_uart.write_all(b"READY");
    let mut i = 0;
    let max_bytes = application_ram.len();
    const ACK_EVERY: usize = 4;
    let mut ack_count = 0;
    while i < max_bytes {
        let ch = loop {
            match context.usb_uart.read() {
                Ok(x) => break x,
                _ => {}
            }
        };
        let mut byte = match ch {
            b'0' => 0x00,
            b'1' => 0x10,
            b'2' => 0x20,
            b'3' => 0x30,
            b'4' => 0x40,
            b'5' => 0x50,
            b'6' => 0x60,
            b'7' => 0x70,
            b'8' => 0x80,
            b'9' => 0x90,
            b'A' => 0xA0,
            b'B' => 0xB0,
            b'C' => 0xC0,
            b'D' => 0xD0,
            b'E' => 0xE0,
            b'F' => 0xF0,
            b'a' => 0xA0,
            b'b' => 0xB0,
            b'c' => 0xC0,
            b'd' => 0xD0,
            b'e' => 0xE0,
            b'f' => 0xF0,
            _ => break,
        };
        let ch = loop {
            match context.usb_uart.read() {
                Ok(x) => break x,
                _ => {}
            }
        };
        byte |= match ch {
            b'0' => 0x00,
            b'1' => 0x01,
            b'2' => 0x02,
            b'3' => 0x03,
            b'4' => 0x04,
            b'5' => 0x05,
            b'6' => 0x06,
            b'7' => 0x07,
            b'8' => 0x08,
            b'9' => 0x09,
            b'A' => 0x0A,
            b'B' => 0x0B,
            b'C' => 0x0C,
            b'D' => 0x0D,
            b'E' => 0x0E,
            b'F' => 0x0F,
            b'a' => 0x0A,
            b'b' => 0x0B,
            b'c' => 0x0C,
            b'd' => 0x0D,
            b'e' => 0x0E,
            b'f' => 0x0F,
            _ => break,
        };
        application_ram[i] = byte;
        ack_count += 1;
        if ack_count >= ACK_EVERY {
            let _ = context.usb_uart.write(b'X');
            ack_count = 0;
        }
        i = i + 1;
    }
    let digest = crc::crc32::checksum_ieee(&application_ram[0..i]);
    writeln!(context, "Loaded {} bytes, CRC32 0x{:08x}", i, digest).unwrap();
}

/// Print some debug info.
fn item_debug_info<'a>(_menu: &Menu, _item: &Item, _input: &str, context: &mut Context) {
    writeln!(context, "Framebuffer: {:08p}", unsafe {
        &FRAMEBUFFER as *const _
    })
    .unwrap();
    writeln!(context, "Application: {:08p}", APPLICATION_START_ADDR).unwrap();
    writeln!(
        context,
        "Chip:\n{:#?}",
        tm4c123x_hal::sysctl::chip_id::get()
    )
    .unwrap();
}

/// Runs a program from application RAM, then returns.
fn item_run_program<'a>(_menu: &Menu, _item: &Item, _input: &str, context: &mut Context) {
    let application_ram: &'static mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(APPLICATION_START_ADDR, APPLICATION_LEN) };
    let addr = ((application_ram[3] as u32) << 24)
        | ((application_ram[2] as u32) << 16)
        | ((application_ram[1] as u32) << 8)
        | ((application_ram[0] as u32) << 0);
    writeln!(context, "Executing from 0x{:08x}", addr).unwrap();
    let ptr = addr as *const ();
    let result = unsafe {
        let code: extern "C" fn(*const api::Table, *mut Context) -> u32 =
            ::core::mem::transmute(ptr);
        code(&api::CALLBACK_TABLE, context as *mut Context)
    };
    writeln!(context, "Result: {}", result).unwrap();
}

/// Makes a short beep.
///
/// The first argument sets the waveform (sine, sawtooth, square or noise).
/// The second sets the frequency (in Hz).
/// The third sets the duration (in 60Hz frames).
/// The fourth sets the channel.
fn item_beep<'a>(_menu: &Menu, _item: &Item, input: &str, context: &mut Context) {
    use monotron_synth::*;
    let mut parts = input.split_whitespace();
    parts.next(); // skip command itself
    let waveform = match parts.next() {
        Some("square") | None => Waveform::Square,
        Some("sine") => Waveform::Sine,
        Some("sawtooth") => Waveform::Sawtooth,
        Some("noise") => Waveform::Noise,
        e => {
            writeln!(context, "Unknown wave argument {:?}", e).unwrap();
            return;
        }
    };
    let frequency = if let Some(arg) = parts.next() {
        match u16::from_str_radix(arg, 10) {
            Ok(f) => f,
            Err(e) => {
                writeln!(context, "Bad frequency argument {:?}", e).unwrap();
                return;
            }
        }
    } else {
        440
    };
    let duration = if let Some(arg) = parts.next() {
        match usize::from_str_radix(arg, 10) {
            Ok(f) => f,
            Err(e) => {
                writeln!(context, "Bad duration argument {:?}", e).unwrap();
                return;
            }
        }
    } else {
        60
    };
    let channel = match parts.next() {
        Some("0") | None => CHANNEL_0,
        Some("1") => CHANNEL_1,
        Some("2") => CHANNEL_2,
        e => {
            writeln!(context, "Unknown duration argument {:?}", e).unwrap();
            return;
        }
    };

    writeln!(
        context,
        "Playing...\r\nWaveform: {:?}\r\nFreq: {} Hz\r\nDuration: {} frames",
        waveform, frequency, duration
    )
    .unwrap();

    unsafe {
        crate::G_SYNTH.play(
            channel,
            Frequency::from_hertz(frequency),
            MAX_VOLUME,
            waveform,
        );
    }

    for _ in 0..duration {
        api::wfvbi(context);
    }

    unsafe {
        crate::G_SYNTH.play(channel, Frequency::from_hertz(frequency), 0, waveform);
    }
}

/// Init the card and dump some details
fn item_mount<'a>(_menu: &Menu, _item: &Item, _input: &str, c: &mut Context) {
    let f = |c: &mut Context| -> Result<(), embedded_sdmmc::SdMmcError> {
        write!(c, "Init SD card...").unwrap();
        c.cont.device().init()?;
        c.cont.device().spi().reclock(10u32.mhz(), &c.clocks);
        write!(c, "OK!\nCard size...").unwrap();
        let size = c.cont.device().card_size_bytes()?;
        writeln!(c, "{}", size).unwrap();
        Ok(())
    };
    match f(c) {
        Err(e) => writeln!(c, "Error: {:?}", e).unwrap(),
        _ => (),
    }
}

/// De-init the card so it can't be used.
fn item_unmount<'a>(_menu: &Menu, _item: &Item, _input: &str, c: &mut Context) {
    write!(c, "De-init SD card...").unwrap();
    c.cont.device().deinit();
    writeln!(c, "OK!").unwrap();
}

/// List the root directory
fn item_dir<'a>(_menu: &Menu, _item: &Item, _input: &str, c: &mut Context) {
    let f = |c: &mut Context| -> Result<(), embedded_sdmmc::Error<_>> {
        write!(c, "Volume 0...").unwrap();
        let v = c.cont.get_volume(embedded_sdmmc::VolumeIdx(0))?;
        writeln!(c, "{:?}", v).unwrap();
        let dir = c.cont.open_root_dir(&v)?;
        c.cont.iterate_dir(&v, &dir, |x| {
            if !x.attributes.is_hidden() && !x.attributes.is_volume() {
                if x.attributes.is_directory() {
                    unsafe {
                        writeln!(FRAMEBUFFER, "{:13} {} <DIR>", x.name, x.mtime).unwrap();
                    }
                } else {
                    unsafe {
                        writeln!(FRAMEBUFFER, "{:13} {} {} bytes", x.name, x.mtime, x.size)
                            .unwrap();
                    }
                }
            }
        })?;
        c.cont.close_dir(&v, dir);
        Ok(())
    };
    match f(c) {
        Err(e) => writeln!(c, "Error: {:?}", e).unwrap(),
        _ => (),
    }
}

/// Load a file from the SD card.
/// TODO work out how to release the directory handle and file handle when the
/// function aborts (e.g. with file not found).
fn item_dload<'a>(_menu: &Menu, _item: &Item, input: &str, c: &mut Context) {
    let f = |c: &mut Context| -> Result<(), embedded_sdmmc::Error<_>> {
        let mut parts = input.split_whitespace();
        parts.next(); // skip command itself
        let file = parts.next().unwrap();
        write!(c, "Loading {:?}...", file).unwrap();
        let volume = c.cont.get_volume(embedded_sdmmc::VolumeIdx(0))?;
        let dir = c.cont.open_root_dir(&volume)?;
        let mut f = c
            .cont
            .open_file_in_dir(&volume, &dir, file, embedded_sdmmc::Mode::ReadOnly)?;
        let application_ram: &'static mut [u8] =
            unsafe { core::slice::from_raw_parts_mut(APPLICATION_START_ADDR, APPLICATION_LEN) };
        for b in application_ram.iter_mut() {
            *b = 0x00;
        }
        c.cont.read(&volume, &mut f, application_ram)?;
        let digest = crc::crc32::checksum_ieee(&application_ram[0..f.length() as usize]);
        writeln!(c, "Loaded {} bytes, CRC32 0x{:08x}", f.length(), digest).unwrap();
        c.cont.close_file(&volume, f)?;
        c.cont.close_dir(&volume, dir);
        Ok(())
    };
    match f(c) {
        Err(e) => writeln!(c, "Error: {:?}", e).unwrap(),
        _ => (),
    }
}

/// Do a hex-dump of a file on disk
/// TODO work out how to release the directory handle and file handle when the
/// function aborts (e.g. with file not found).
fn item_ddump<'a>(_menu: &Menu, _item: &Item, input: &str, c: &mut Context) {
    let f = |c: &mut Context| -> Result<(), embedded_sdmmc::Error<_>> {
        let mut parts = input.split_whitespace();
        parts.next(); // skip command itself
        let file = parts.next().unwrap();
        write!(c, "Dumping {:?}...", file).unwrap();
        let volume = c.cont.get_volume(embedded_sdmmc::VolumeIdx(0))?;
        let dir = c.cont.open_root_dir(&volume)?;
        let mut f = c
            .cont
            .open_file_in_dir(&volume, &dir, file, embedded_sdmmc::Mode::ReadOnly)?;
        let application_ram: &'static mut [u8] =
            unsafe { core::slice::from_raw_parts_mut(APPLICATION_START_ADDR, APPLICATION_LEN) };
        c.cont.read(&volume, &mut f, application_ram)?;
        let digest = crc::crc32::checksum_ieee(&application_ram[0..f.length() as usize]);
        writeln!(c, "Loaded {} bytes, CRC32 0x{:08x}", f.length(), digest).unwrap();
        const CHUNK_SIZE: usize = 16;
        let mut lines_printed = 0;
        const MAX_LINES: usize = 35;
        for (idx, line) in application_ram[0..f.length() as usize]
            .chunks(CHUNK_SIZE)
            .enumerate()
        {
            write!(c, "{:06x}:", idx * CHUNK_SIZE).unwrap();
            for (idx, b) in line.iter().enumerate() {
                if (idx % 4) == 0 {
                    write!(c, " ").unwrap();
                }
                write!(c, "{:02x}", b).unwrap();
            }
            writeln!(c, "").unwrap();
            lines_printed += 1;
            if lines_printed == MAX_LINES {
                lines_printed = 0;
                write!(c, "Press a key...").unwrap();
                loop {
                    crate::api::wfvbi(c);
                    // Wait for new input
                    match c.read() {
                        None => {}
                        _ => break,
                    }
                }
                write!(c, "\r                \r").unwrap();
            }
        }
        c.cont.close_file(&volume, f)?;
        c.cont.close_dir(&volume, dir);
        Ok(())
    };
    match f(c) {
        Err(e) => writeln!(c, "Error: {:?}", e).unwrap(),
        _ => (),
    }
}

/// Display a text file on disk a page at a time
/// TODO work out how to release the directory handle and file handle when the
/// function aborts (e.g. with file not found).
fn item_dpage<'a>(_menu: &Menu, _item: &Item, input: &str, c: &mut Context) {
    let f = |c: &mut Context| -> Result<(), embedded_sdmmc::Error<_>> {
        let mut parts = input.split_whitespace();
        parts.next(); // skip command itself
        let file = parts.next().unwrap();
        writeln!(c, "Displaying {:?}...", file).unwrap();
        let volume = c.cont.get_volume(embedded_sdmmc::VolumeIdx(0))?;
        let dir = c.cont.open_root_dir(&volume)?;
        let mut f = c
            .cont
            .open_file_in_dir(&volume, &dir, file, embedded_sdmmc::Mode::ReadOnly)?;
        let application_ram: &'static mut [u8] =
            unsafe { core::slice::from_raw_parts_mut(APPLICATION_START_ADDR, APPLICATION_LEN) };
        c.cont.read(&volume, &mut f, application_ram)?;
        let mut lines_printed = 0;
        let mut line_length = 0;
        const MAX_LINES: usize = 35;
        for &b in application_ram[0..f.length() as usize].iter() {
            unsafe {
                let _ = FRAMEBUFFER.write_character(b);
            }
            line_length += 1;
            if (b == b'\n') || (line_length == 48) {
                lines_printed += 1;
                line_length = 0;
                if lines_printed == MAX_LINES {
                    lines_printed = 0;
                    write!(c, "Press a key...").unwrap();
                    loop {
                        crate::api::wfvbi(c);
                        // Wait for new input
                        match c.read() {
                            None => {}
                            _ => break,
                        }
                    }
                    write!(c, "\r                \r").unwrap();
                }
            }
        }
        c.cont.close_file(&volume, f)?;
        c.cont.close_dir(&volume, dir);
        Ok(())
    };
    match f(c) {
        Err(e) => writeln!(c, "Error: {:?}", e).unwrap(),
        _ => (),
    }
}

// End of file
