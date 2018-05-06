use fb;
use menu;
use asm;
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

pub(crate) const ROOT_MENU: Menu = Menu {
    label: "root",
    items: &[&TEST_ALPHABET, &TEST_ANIMATION, &TEST_CLEAR],
    entry: None,
    exit: None,
};

/// The test menu item - displays a static bitmap.
fn test_alphabet<'a>(_menu: &Menu, _item: &Item, _input: &str, context: &mut Context) {
    let mut old_frame = 0;
    let mut ch = 0u8;
    const COLOURS: [fb::Attr; 6] = [ fb::RED_ON_BLACK, fb::YELLOW_ON_BLACK, fb::GREEN_ON_BLACK, fb::CYAN_ON_BLACK, fb::BLUE_ON_BLACK, fb::MAGENTA_ON_BLACK ];
    let mut colour_wheel = COLOURS.iter().cloned().cycle();
    loop {
        asm::wfi();
        let new_frame = unsafe { FRAMEBUFFER.frame() };
        if new_frame != old_frame {
            old_frame = new_frame;
            unsafe {
                FRAMEBUFFER.write_glyph(fb::Glyph::from_byte(ch), colour_wheel.next());
            }
            if ch == 255 {
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
    unsafe { FRAMEBUFFER.goto(0, 0).unwrap() };
}

/// Another test menu item - displays an animation.
fn test_animation<'a>(_menu: &Menu, _item: &Item, input: &str, context: &mut Context) {
    let mut old_frame = 0;
    let mut row = 0;
    let mut col = 0;
    let mut left = true;
    let mut down = true;
    let input = input.trim_left_matches("animate ");
    let num_chars = input.chars().count();
    loop {
        asm::wfi();
        let new_frame = unsafe { FRAMEBUFFER.frame() };
        if new_frame != old_frame {
            old_frame = new_frame;
            if left {
                col += 1;
            } else {
                col -= 1;
            }
            if down {
                row += 1
            } else {
                row -= 1;
            }
            if col == 0 {
                left = true;
            }
            if col == (fb::TEXT_MAX_COL - num_chars) {
                left = false;
            }
            if row == 0 {
                down = true;
            }
            if row == fb::TEXT_MAX_ROW {
                down = false;
            }
            unsafe {
                FRAMEBUFFER.clear();
                FRAMEBUFFER.goto(col, row).unwrap();
                write!(FRAMEBUFFER, "{}", input).unwrap();
            }
        }
        if let Ok(_ch) = context.rx.read() {
            break;
        }
    }
}
