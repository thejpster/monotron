use fb;
use menu;
use asm;
use fb::Console;
use embedded_hal::prelude::*;
use core::fmt::Write;
use super::{Context, FRAMEBUFFER};

pub(crate) type Menu<'a> = menu::Menu<'a, Context>;
pub(crate) type Item<'a> = menu::Item<'a, Context>;

const TEST_ALPHABET: Item = Item {
    item_type: menu::ItemType::Callback(test_alphabet),
    command: "alphabet",
    help: Some("Scrolls some test text output."),
};

const TEST_CLEAR: Item = Item {
    item_type: menu::ItemType::Callback(test_clear),
    command: "clear",
    help: Some("Resets the display."),
};

const TEST_ANIMATION: Item = Item {
    item_type: menu::ItemType::Callback(test_animation),
    command: "animate",
    help: Some("Bounces argument around."),
};

const TEST_ART: Item = Item {
    item_type: menu::ItemType::Callback(test_art),
    command: "art",
    help: Some("Show some art."),
};

pub(crate) const ROOT_MENU: Menu = Menu {
    label: "root",
    items: &[&TEST_ALPHABET, &TEST_ANIMATION, &TEST_ART, &TEST_CLEAR],
    entry: None,
    exit: None,
};

/// The test menu item - displays a static bitmap.
fn test_alphabet<'a>(_menu: &Menu, _item: &Item, _input: &str, context: &mut Context) {
    let mut old_frame = 0;
    let mut ch = 0u8;
    const COLOURS: [fb::Attr; 4] = [ fb::Attr::Normal, fb::Attr::Reverse, fb::Attr::WhiteOnBlack, fb::Attr::GreenOnBlack ];
    let mut colour_wheel = COLOURS.iter().cloned().cycle();
    let mut attr = colour_wheel.next();
    loop {
        asm::wfi();
        let new_frame = unsafe { FRAMEBUFFER.frame() };
        if new_frame != old_frame {
            old_frame = new_frame;
            unsafe {
                FRAMEBUFFER.write_glyph(fb::Glyph::from_byte(ch), attr);
            }
            if ch == 255 {
                attr = colour_wheel.next();
                ch = 0;
            } else {
                ch += 1;
            }
        }
        if let Ok(_ch) = context.rx.read() {
            break;
        }
    }
}

/// Another test menu item - displays an animation.
fn test_clear<'a>(_menu: &Menu, _item: &Item, _input: &str, _context: &mut Context) {
    unsafe { FRAMEBUFFER.clear() };
    unsafe { FRAMEBUFFER.set_pos(fb::Position::origin()).unwrap() };
}

/// Display some art
fn test_art<'a>(_menu: &Menu, _item: &Item, _input: &str, context: &mut Context) {
    unsafe { FRAMEBUFFER.clear(); }
    write!(context, "SCORE 0300      HIGH 0000          3    ╩       ").unwrap();
    write!(context, "  ▀▄ ▄▀     ▀▄ ▄▀     ▀▄ ▄▀     ▀▄ ▄▀           ").unwrap();
    write!(context, " ▄█▀█▀█▄   ▄█▀█▀█▄   ▄█▀█▀█▄   ▄█▀█▀█▄          ").unwrap();
    write!(context, "█▀█████▀█ █▀█████▀█ █▀█████▀█ █▀█████▀█         ").unwrap();
    write!(context, "▀ ▀▄ ▄▀ ▀ ▀ ▀▄ ▄▀ ▀ ▀ ▀▄ ▄▀ ▀ ▀ ▀▄ ▄▀ ▀         ").unwrap();
    writeln!(context, "").unwrap();
    write!(context, "  ▀▄ ▄▀     ▀▄ ▄▀     ▀▄ ▄▀     ▀▄ ▄▀           ").unwrap();
    write!(context, " ▄█▀█▀█▄   ▄█▀█▀█▄   ▄█▀█▀█▄   ▄█▀█▀█▄          ").unwrap();
    write!(context, "█▀█████▀█ █▀█████▀█ █▀█████▀█ █▀█████▀█         ").unwrap();
    write!(context, "▀ ▀▄ ▄▀ ▀ ▀ ▀▄ ▄▀ ▀ ▀ ▀▄ ▄▀ ▀ ▀ ▀▄ ▄▀ ▀         ").unwrap();
    writeln!(context, "").unwrap();
    write!(context, "  ▀▄ ▄▀     ▀▄ ▄▀     ▀▄ ▄▀     ▀▄ ▄▀           ").unwrap();
    write!(context, " ▄█▀█▀█▄   ▄█▀█▀█▄   ▄█▀█▀█▄   ▄█▀█▀█▄          ").unwrap();
    write!(context, "█▀█████▀█ █▀█████▀█ █▀█████▀█ █▀█████▀█         ").unwrap();
    write!(context, "▀ ▀▄ ▄▀ ▀ ▀ ▀▄ ▄▀ ▀ ▀ ▀▄ ▄▀ ▀ ▀ ▀▄ ▄▀ ▀         ").unwrap();
    writeln!(context, "").unwrap();
    writeln!(context, "        |").unwrap();
    write!(context, "  ██░▒  |   ████      ████      ████      ████  ").unwrap();
    write!(context, "██████▓▓  ████████  ████████  ████████  ████████").unwrap();
    writeln!(context, "").unwrap();
    writeln!(context, "       ═╩═").unwrap();
    writeln!(context, "Press a key...").unwrap();
    loop {
        asm::wfi();
        if let Ok(_ch) = context.rx.read() {
            break;
        }
    }
    writeln!(context, "              1UP   HIGH SCORE").unwrap();
    writeln!(context, "                00        00").unwrap();
    writeln!(context, "          ╔════════════╦╦════════════╗").unwrap();
    writeln!(context, "          ║············║║············║").unwrap();
    writeln!(context, "          ║·┌──┐·┌───┐·║║·┌───┐·┌──┐·║").unwrap();
    writeln!(context, "          ║○│  │·│   │·║║·│   │·│  │○║").unwrap();
    writeln!(context, "          ║·└──┘·└───┘·╚╝·└───┘·└──┘·║").unwrap();
    writeln!(context, "          ║··························║").unwrap();
    writeln!(context, "          ║·┌──┐·┌┐·┌──────┐·┌┐·┌──┐·║").unwrap();
    writeln!(context, "          ║·└──┘·││·└──┐┌──┘·││·└──┘·║").unwrap();
    writeln!(context, "          ║······││····││····││······║").unwrap();
    writeln!(context, "          ╚════╗·│└──┐ ││ ┌──┘│·╔════╝").unwrap();
    writeln!(context, "               ║·│┌──┘ └┘ └──┐│·║     ").unwrap();
    writeln!(context, "               ║·││    ☺     ││·║     ").unwrap();
    writeln!(context, "          ═════╝·└┘ ╔══════╗ └┘·╚═════").unwrap();
    writeln!(context, "                ·   ║☺ ☺ ☺ ║   ·      ").unwrap();
    writeln!(context, "          ═════╗·┌┐ ╚══════╝ ┌┐·╔═════").unwrap();
    writeln!(context, "               ║·││  READY!  ││·║     ").unwrap();
    writeln!(context, "               ║·││ ┌──────┐ ││·║     ").unwrap();
    writeln!(context, "          ╔════╝·└┘ └──┐┌──┘ └┘·╚════╗").unwrap();
    writeln!(context, "          ║············││············║").unwrap();
    writeln!(context, "          ║·┌──┐·┌───┐·││·┌───┐·┌──┐·║").unwrap();
    writeln!(context, "          ║·└─┐│·└───┘·└┘·└───┘·│┌─┘·║").unwrap();
    writeln!(context, "          ║○··││·······◄►·······││··○║").unwrap();
    writeln!(context, "          ╠═╗·││·┌┐·┌──────┐·┌┐·││·╔═╣").unwrap();
    writeln!(context, "          ╠═╝·└┘·││·└──┐┌──┘·││·└┘·╚═╣").unwrap();
    writeln!(context, "          ║······││····││····││······║").unwrap();
    writeln!(context, "          ║·┌────┘└──┐·││·┌──┘└────┐·║").unwrap();
    writeln!(context, "          ║·└────────┘·└┘·└────────┘·║").unwrap();
    writeln!(context, "          ║··························║").unwrap();
    writeln!(context, "          ╚══════════════════════════╝").unwrap();
    writeln!(context, "             ◄► ◄► ◄►").unwrap();
    writeln!(context, "\n\nPress a key...").unwrap();
    loop {
        asm::wfi();
        if let Ok(_ch) = context.rx.read() {
            break;
        }
    }
}

/// Another test menu item - displays an animation.
fn test_animation<'a>(_menu: &Menu, _item: &Item, input: &str, context: &mut Context) {
    let mut old_frame = 0;
    let mut pos = fb::Position::origin();
    let mut left = true;
    let mut down = true;
    let width = unsafe { FRAMEBUFFER.get_width() };
    let height = unsafe { FRAMEBUFFER.get_height() };
    let input = input.trim_left_matches("animate ");
    let num_chars = input.chars().count();
    loop {
        asm::wfi();
        let new_frame = unsafe { FRAMEBUFFER.frame() };
        if new_frame != old_frame {
            old_frame = new_frame;
            if left {
                pos.col.incr();
            } else {
                pos.col.decr();
            }
            if down {
                pos.row.incr();
            } else {
                pos.row.decr();
            }
            if pos.col == fb::Col::origin() {
                left = true;
            }
            if pos.col == fb::Col(width.0 - num_chars as u8) {
                left = false;
            }
            if pos.row == fb::Row::origin() {
                down = true;
            }
            if pos.row == height {
                down = false;
            }
            unsafe {
                FRAMEBUFFER.clear();
                FRAMEBUFFER.set_pos(pos).unwrap();
                write!(FRAMEBUFFER, "{}", input).unwrap();
            }
        }
        if let Ok(_ch) = context.rx.read() {
            break;
        }
    }
}

// End of file
