use fb;
use menu;
use asm;
use fb::Console;
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

const ITEM_PEEK: Item = Item {
    item_type: menu::ItemType::Callback(item_peek),
    command: "peek",
    help: Some("<addr> - Read a register."),
};

const ITEM_POKE: Item = Item {
    item_type: menu::ItemType::Callback(item_poke),
    command: "poke",
    help: Some("<addr> <value> - Write a register."),
};

const ITEM_DUMP: Item = Item {
    item_type: menu::ItemType::Callback(item_dump),
    command: "dump",
    help: Some("<addr> <bytes> - Dump RAM/ROM."),
};

pub(crate) const ROOT_MENU: Menu = Menu {
    label: "root",
    items: &[&TEST_ALPHABET, &TEST_ANIMATION, &TEST_ART, &TEST_CLEAR, &ITEM_PEEK, &ITEM_POKE, &ITEM_DUMP],
    entry: None,
    exit: None,
};

/// The test menu item - displays a static bitmap.
fn test_alphabet<'a>(_menu: &Menu, _item: &Item, _input: &str, context: &mut Context) {
    let mut old_frame = 0;
    let mut ch = 0u8;
    const COLOURS: [fb::Colour; 8] = [
        fb::Colour::White,
        fb::Colour::Yellow,
        fb::Colour::Magenta,
        fb::Colour::Red,
        fb::Colour::Cyan,
        fb::Colour::Green,
        fb::Colour::Blue,
        fb::Colour::Black
    ];
    let mut fg_wheel = COLOURS.iter().cycle();
    let mut bg_wheel = COLOURS.iter().cycle();
    let mut fg = fg_wheel.next();
    let mut bg = bg_wheel.next();
    loop {
        asm::wfi();
        let new_frame = unsafe { FRAMEBUFFER.frame() };
        if new_frame != old_frame {
            old_frame = new_frame;
            unsafe {
                FRAMEBUFFER.write_glyph(fb::Glyph::from_byte(ch), Some(fb::Attr::new(*fg.unwrap(), *bg.unwrap())));
            }
            fg = fg_wheel.next();
            if ch == 255 {
                bg = bg_wheel.next();
                ch = 0;
            } else {
                ch += 1;
            }
        }
        if let Some(_input) = context.read() {
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
        if let Some(_input) = context.read() {
            break;
        }
    }
    writeln!(context, "\u{001b}B\u{001b}h\u{001b}Z").unwrap();
    writeln!(context, "              1UP   HIGH SCORE").unwrap();
    writeln!(context, "                00        00").unwrap();
    writeln!(context, "\u{001b}F          ╔════════════╦╦════════════╗").unwrap();
    writeln!(context, "          ║\u{001b}E············\u{001b}F║║\u{001b}E············\u{001b}F║").unwrap();
    writeln!(context, "          ║\u{001b}E·\u{001b}F┌──┐\u{001b}E·\u{001b}F┌───┐\u{001b}E·\u{001b}F║║\u{001b}E·\u{001b}F┌───┐\u{001b}E·\u{001b}F┌──┐\u{001b}E·\u{001b}F║").unwrap();
    writeln!(context, "          ║○│  │\u{001b}E·\u{001b}F│   │\u{001b}E·\u{001b}F║║\u{001b}E·\u{001b}F│   │\u{001b}E·\u{001b}F│  │○║").unwrap();
    writeln!(context, "          ║\u{001b}E·\u{001b}F└──┘\u{001b}E·\u{001b}F└───┘\u{001b}E·\u{001b}F╚╝\u{001b}E·\u{001b}F└───┘\u{001b}E·\u{001b}F└──┘\u{001b}E·\u{001b}F║").unwrap();
    writeln!(context, "          ║\u{001b}E··························\u{001b}F║").unwrap();
    writeln!(context, "          ║\u{001b}E·\u{001b}F┌──┐\u{001b}E·\u{001b}F┌┐\u{001b}E·\u{001b}F┌──────┐\u{001b}E·\u{001b}F┌┐\u{001b}E·\u{001b}F┌──┐\u{001b}E·\u{001b}F║").unwrap();
    writeln!(context, "          ║\u{001b}E·\u{001b}F└──┘\u{001b}E·\u{001b}F││\u{001b}E·\u{001b}F└──┐┌──┘\u{001b}E·\u{001b}F││\u{001b}E·\u{001b}F└──┘\u{001b}E·\u{001b}F║").unwrap();
    writeln!(context, "          ║\u{001b}E······\u{001b}F││\u{001b}E····\u{001b}F││\u{001b}E····\u{001b}F││\u{001b}E······\u{001b}F║").unwrap();
    writeln!(context, "          ╚════╗\u{001b}E·\u{001b}F│└──┐ ││ ┌──┘│\u{001b}E·\u{001b}F╔════╝").unwrap();
    writeln!(context, "               ║\u{001b}E·\u{001b}F│┌──┘ └┘ └──┐│\u{001b}E·\u{001b}F║     ").unwrap();
    writeln!(context, "               ║\u{001b}E·\u{001b}F││    \u{001b}B☺\u{001b}F     ││\u{001b}E·\u{001b}F║     ").unwrap();
    writeln!(context, "          ═════╝\u{001b}E·\u{001b}F└┘ ╔══════╗ └┘\u{001b}E·\u{001b}F╚═════").unwrap();
    writeln!(context, "                \u{001b}E·\u{001b}F   ║\u{001b}D☺ \u{001b}F☺ \u{001b}G☺\u{001b}F ║   \u{001b}E·\u{001b}F      ").unwrap();
    writeln!(context, "          ═════╗\u{001b}E·\u{001b}F┌┐ ╚══════╝ ┌┐\u{001b}E·\u{001b}F╔═════").unwrap();
    writeln!(context, "               ║\u{001b}E·\u{001b}F││  READY!  ││\u{001b}E·\u{001b}F║     ").unwrap();
    writeln!(context, "               ║\u{001b}E·\u{001b}F││ ┌──────┐ ││\u{001b}E·\u{001b}F║     ").unwrap();
    writeln!(context, "          ╔════╝\u{001b}E·\u{001b}F└┘ └──┐┌──┘ └┘\u{001b}E·\u{001b}F╚════╗").unwrap();
    writeln!(context, "          ║\u{001b}E············\u{001b}F││\u{001b}E············\u{001b}F║").unwrap();
    writeln!(context, "          ║\u{001b}E·\u{001b}F┌──┐\u{001b}E·\u{001b}F┌───┐\u{001b}E·\u{001b}F││\u{001b}E·\u{001b}F┌───┐\u{001b}E·\u{001b}F┌──┐\u{001b}E·\u{001b}F║").unwrap();
    writeln!(context, "          ║\u{001b}E·\u{001b}F└─┐│\u{001b}E·\u{001b}F└───┘\u{001b}E·\u{001b}F└┘\u{001b}E·\u{001b}F└───┘\u{001b}E·\u{001b}F│┌─┘\u{001b}E·\u{001b}F║").unwrap();
    writeln!(context, "          ║○\u{001b}E··\u{001b}F││\u{001b}E·······\u{001b}B◄►\u{001b}E·······\u{001b}F││\u{001b}E··\u{001b}F○║").unwrap();
    writeln!(context, "          ╠═╗\u{001b}E·\u{001b}F││\u{001b}E·\u{001b}F┌┐\u{001b}E·\u{001b}F┌──────┐\u{001b}E·\u{001b}F┌┐\u{001b}E·\u{001b}F││\u{001b}E·\u{001b}F╔═╣").unwrap();
    writeln!(context, "          ╠═╝\u{001b}E·\u{001b}F└┘\u{001b}E·\u{001b}F││\u{001b}E·\u{001b}F└──┐┌──┘\u{001b}E·\u{001b}F││\u{001b}E·\u{001b}F└┘\u{001b}E·\u{001b}F╚═╣").unwrap();
    writeln!(context, "          ║\u{001b}E······\u{001b}F││\u{001b}E····\u{001b}F││\u{001b}E····\u{001b}F││\u{001b}E······\u{001b}F║").unwrap();
    writeln!(context, "          ║\u{001b}E·\u{001b}F┌────┘└──┐\u{001b}E·\u{001b}F││\u{001b}E·\u{001b}F┌──┘└────┐\u{001b}E·\u{001b}F║").unwrap();
    writeln!(context, "          ║\u{001b}E·\u{001b}F└────────┘\u{001b}E·\u{001b}F└┘\u{001b}E·\u{001b}F└────────┘\u{001b}E·\u{001b}F║").unwrap();
    writeln!(context, "          ║\u{001b}E··························\u{001b}F║").unwrap();
    writeln!(context, "          ╚══════════════════════════╝").unwrap();
    writeln!(context, "             ◄► ◄► ◄►").unwrap();
    write!(context, "\n\n\nPress a key...").unwrap();
    loop {
        asm::wfi();
        if let Some(_input) = context.read() {
            break;
        }
    }

    write!(context, "\u{001b}Z\u{001b}H\u{001b}a  \u{001b}DF\u{001b}Hile   \u{001b}DE\u{001b}Hdit  \u{001b}DV\u{001b}Hiew   \u{001b}DT\u{001b}Hools  \u{001b}DH\u{001b}Help               ").unwrap();
    write!(context, "\u{001b}G\u{001b}e████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████\u{001b}A\u{001b}g┌[Calc]──X┐    \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "████████████████████████████\u{001b}A\u{001b}g│\u{001b}G\u{001b}h         \u{001b}A\u{001b}g│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c┌[Calendar]───────────X┐\u{001b}G\u{001b}e█\u{001b}A\u{001b}g│\u{001b}G\u{001b}h         \u{001b}A\u{001b}g│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}G                  \u{001b}A\u{0000}\u{001b}G  \u{001b}A │\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g│\u{001b}h12345678\u{001b}G \u{001b}A\u{001b}g│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}G                  \u{001b}A\u{0000}\u{001b}G  \u{001b}A │\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g│\u{001b}G\u{001b}h         \u{001b}A\u{001b}g│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│MonTueWedThuFriSatSun │\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g│\u{001b}G\u{001b}e█████████\u{001b}A\u{001b}g│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E┌────────────────────┐\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g│7 8 9 /\u{001b}G\u{001b}e█\u{001b}A\u{001b}gC│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E│\u{001b}A  \u{001b}E│\u{001b}A01\u{001b}E│\u{001b}A02\u{001b}E│\u{001b}A03\u{001b}E│\u{001b}A04\u{001b}E│\u{001b}A05\u{001b}E│\u{001b}A06\u{001b}E│\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g│\u{001b}G\u{001b}e█████████\u{001b}A\u{001b}g│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E├────────────────────┤\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g│4 5 6 ( )│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E│\u{001b}A07\u{001b}E│\u{001b}A08\u{001b}E│\u{001b}A09\u{001b}E│\u{001b}A10\u{001b}E│\u{001b}A11\u{001b}E│\u{001b}A12\u{001b}E│\u{001b}A13\u{001b}E│\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g│\u{001b}G\u{001b}e█████████\u{001b}A\u{001b}g│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E├────────────────────┤\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g│1 2 3 - ²│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E│\u{001b}A14\u{001b}E│\u{001b}A15\u{001b}E│\u{001b}A16\u{001b}E│\u{001b}A17\u{001b}E│\u{001b}A18\u{001b}E│\u{001b}A19\u{001b}E│\u{001b}A20\u{001b}E│\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g│\u{001b}G\u{001b}e█████████\u{001b}A\u{001b}g│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E├────────────────────┤\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g│0 . % + =│\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E│\u{001b}A21\u{001b}E│\u{001b}A22\u{001b}E│\u{001b}A23\u{001b}E│\u{001b}A24\u{001b}E│\u{001b}A25\u{001b}E│\u{001b}A26\u{001b}E│\u{001b}A27\u{001b}E│\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g└─────────┘\u{001b}G\u{001b}h▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E├────────────────────┤\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g \u{001b}G\u{001b}h▒▒▒▒▒▒▒▒▒▒▒\u{001b}e██\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E│\u{001b}A28\u{001b}E│\u{001b}A29\u{001b}E│\u{001b}A30\u{001b}E│\u{001b}A31\u{001b}E│\u{001b}G     \u{001b}A\u{0000}\u{001b}G  \u{001b}E│\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████████████\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E└────────────────────┘\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████████████\u{001b}A\u{001b}g \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}E\u{0011}\u{001b}G \u{001b}E\u{0011}\u{0011}\u{001b}G  \u{001b}BMay 2018\u{001b}G   \u{001b}E\u{0010}\u{0010}\u{001b}G  \u{001b}E\u{0010}\u{001b}A│\u{001b}G\u{001b}h▒\u{001b}A\u{001b}g               \u{001b}G\u{001b}e█████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c│\u{001b}G                  \u{001b}A\u{0000}\u{001b}G  \u{001b}A │\u{001b}G\u{001b}h▒\u{001b}e████████████████████").unwrap();
    write!(context, "███\u{001b}A\u{001b}c└──────────────────────┘\u{001b}G\u{001b}h▒\u{001b}e████████████████████").unwrap();
    write!(context, "████\u{001b}h▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒\u{001b}e████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "\u{001b}H\u{001b}a▒\u{001b}A\u{001b}hCAPS\u{001b}H\u{001b}a▒\u{001b}A\u{001b}hINS\u{001b}H\u{001b}a▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒\u{001b}A\u{001b}h[Version v1.2]\u{001b}H\u{001b}a").unwrap();

    loop {
        asm::wfi();
        if let Some(_input) = context.read() {
            writeln!(context, "\u{001b}A\u{001b}g\u{001b}ZOk...").unwrap();
            break;
        }
    }

    write!(context, "\u{001b}Z\u{001b}B\u{001b}h╔════════════════════════\u{001b}A[MonotronPaint]\u{001b}B═══════╗").unwrap();
    write!(context, "║\u{001b}A┌[Font]──────────┐┌──────────────────────────┐\u{001b}B║").unwrap();

    write!(context, "║\u{001b}A│").unwrap();
    unsafe { FRAMEBUFFER.set_control_char_mode(fb::ControlCharMode::Display); }
    write!(context, " \u{0001}\u{0002}\u{0003}\u{0004}\u{0005}\u{0006}\u{0007}\u{0008}\u{0009}\u{000a}\u{000b}\u{000c}\u{000d}\u{000e}\u{000f}").unwrap();
    unsafe { FRAMEBUFFER.set_control_char_mode(fb::ControlCharMode::Interpret); }
    write!(context, "││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();

    write!(context, "║\u{001b}A│").unwrap();
    unsafe { FRAMEBUFFER.set_control_char_mode(fb::ControlCharMode::Display); }
    write!(context, "\u{0010}\u{0011}\u{0012}\u{0013}\u{0014}\u{0015}\u{0016}\u{0017}\u{0018}\u{0019}\u{001a}\u{001b}\u{001c}\u{001d}\u{001e}\u{001f}").unwrap();
    unsafe { FRAMEBUFFER.set_control_char_mode(fb::ControlCharMode::Interpret); }
    write!(context, "││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();

    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g          ▓▓▓▓▓▓▓         \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g        ▓▓▓▓▓▓▓▓▓▓▓       \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g       ▓▓▓▓▓▓▓▓▓▓▓▓▓      \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g    ▓▓▓\u{001b}H\u{001b}h▓▓▓\u{001b}B\u{001b}g▓▓▓▓▓▓\u{001b}H\u{001b}h▓▓▓\u{001b}B\u{001b}g▓▓▓▓   \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g   ▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓  \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g   ▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓  \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g  ▓▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓▓ \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g  ▓▓▓▓▓\u{001b}H\u{001b}h▓▓▓\u{001b}B\u{001b}g▓▓▓▓▓▓\u{001b}H\u{001b}h▓▓▓\u{001b}B\u{001b}g▓▓▓▓▓▓ \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}H                \u{001b}A││\u{001b}B\u{001b}g  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A└────────────────┘│\u{001b}B\u{001b}g  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A┌[Colour]┐┌[Menu]┐│\u{001b}B\u{001b}g  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}D█\u{001b}F█\u{001b}G█\u{001b}B█\u{001b}C█\u{001b}E█\u{001b}H█\u{001b}A█││L\u{001b}Co\u{001b}Aad\u{001b}H  \u{001b}A││\u{001b}B\u{001b}g  ▓▓▓▓▓\u{001b}H\u{001b}h▓\u{001b}B\u{001b}g▓▓▓▓▓▓▓▓▓▓▓▓\u{001b}H\u{001b}h▓\u{001b}B\u{001b}g▓▓▓▓ \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A└────────┘│\u{001b}CS\u{001b}Aave\u{001b}H  \u{001b}A││\u{001b}B\u{001b}g  ▓▓▓▓▓\u{001b}H\u{001b}h▓▓\u{001b}B\u{001b}g▓▓▓▓▓▓▓▓▓▓\u{001b}H\u{001b}h▓▓\u{001b}B\u{001b}g▓▓▓▓ \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A┌[Tools]─┐│\u{001b}CP\u{001b}Arint\u{001b}H \u{001b}A││\u{001b}B\u{001b}g   ▓▓▓▓▓\u{001b}H\u{001b}h▓▓\u{001b}B\u{001b}g▓▓▓▓▓▓▓▓▓\u{001b}H\u{001b}h▓\u{001b}B\u{001b}g▓▓▓▓  \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}CR\u{001b}Aect\u{001b}H    \u{001b}A││\u{001b}CE\u{001b}Axport││\u{001b}B\u{001b}g   ▓▓▓▓▓▓\u{001b}H\u{001b}h▓\u{001b}B\u{001b}g▓▓▓▓▓▓▓\u{001b}H\u{001b}h▓▓▓\u{001b}B\u{001b}g▓▓▓▓  \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}CC\u{001b}Aell\u{001b}H    \u{001b}A││C\u{001b}Cl\u{001b}Aear\u{001b}H \u{001b}A││\u{001b}B\u{001b}g    ▓▓▓▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓▓   \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}CL\u{001b}Aine\u{001b}H    \u{001b}A││\u{001b}H      \u{001b}A││\u{001b}B\u{001b}g     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}CF\u{001b}Aill\u{001b}H    \u{001b}A│└──────┘│\u{001b}B\u{001b}g     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}C\u{001b}eT\u{001b}Hext    \u{001b}A\u{001b}h│┌[Layr]┐│\u{001b}B\u{001b}g       ▓▓▓▓▓▓▓▓▓▓▓▓▓      \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}CC\u{001b}Aopy\u{001b}H    \u{001b}A││\u{001b}H      \u{001b}A││\u{001b}B\u{001b}g        ▓▓▓▓▓▓▓▓▓▓▓       \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│\u{001b}CP\u{001b}Aaste\u{001b}H   \u{001b}A││1  \u{001b}FH\u{001b}DL\u{001b}AX││\u{001b}B\u{001b}g          ▓▓▓▓▓▓▓         \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A└────────┘│2  \u{001b}FH\u{001b}DL\u{001b}AX││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A┌[Info]──┐│3  \u{001b}DH\u{001b}FL\u{001b}AX││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│N:Test\u{001b}H  \u{001b}A││4  \u{001b}FHL\u{001b}AX││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│S:48x36 ││\u{001b}H      \u{001b}A││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A└────────┘└──────┘└──────────────────────────┘\u{001b}B║").unwrap();
    write!(context, "╚══════════════════════════════════════════════╝").unwrap();
    unsafe { FRAMEBUFFER.set_control_char_mode(fb::ControlCharMode::Interpret); }

    loop {
        asm::wfi();
        if let Some(_input) = context.read() {
            break;
        }
    }

    write!(context, "\u{001b}Z\u{001b}A\u{001b}h╔══════════════════════════════════════════════╗").unwrap();
    write!(context, "║\u{001b}D█████\u{001b}H \u{001b}D\u{001b}b█████\u{001b}H\u{001b}h \u{001b}B██  █\u{001b}H \u{001b}F█████\u{001b}H \u{001b}F\u{001b}b█\u{001b}h█\u{001b}b█\u{001b}h██\u{001b}H \u{001b}G████\u{001b}H \u{001b}G█████\u{001b}H \u{001b}C██  █\u{001b}A║").unwrap();
    write!(context, "║\u{001b}D▓\u{001b}H \u{001b}D▓\u{001b}H \u{001b}D▓\u{001b}H \u{001b}D\u{001b}b▓\u{001b}H\u{001b}h   \u{001b}D\u{001b}b▓\u{001b}H\u{001b}h \u{001b}B▓\u{001b}H \u{001b}B▓ ▓\u{001b}H \u{001b}F▓\u{001b}H   \u{001b}F▓\u{001b}H \u{001b}F \u{001b}H \u{001b}F\u{001b}b▓\u{001b}H\u{001b}h \u{001b}F \u{001b}H \u{001b}G\u{001b}f▓\u{001b}H\u{001b}h  \u{001b}G\u{001b}f▓\u{001b}H\u{001b}h \u{001b}G▓\u{001b}H   \u{001b}G▓\u{001b}H \u{001b}C▓\u{001b}H \u{001b}C▓ ▓\u{001b}A║").unwrap();
    write!(context, "║\u{001b}D▒\u{001b}H \u{001b}D▒\u{001b}H \u{001b}D▒\u{001b}H \u{001b}D\u{001b}b▒\u{001b}H\u{001b}h   \u{001b}D\u{001b}b▒\u{001b}H\u{001b}h \u{001b}B▒\u{001b}H  \u{001b}B▒▒\u{001b}H \u{001b}F▒\u{001b}H   \u{001b}F▒\u{001b}H \u{001b}F \u{001b}H \u{001b}F\u{001b}b▒\u{001b}H\u{001b}h \u{001b}F \u{001b}H \u{001b}G\u{001b}f▒\u{001b}H\u{001b}h \u{001b}G\u{001b}f▒\u{001b}h \u{001b}H \u{001b}G▒\u{001b}H   \u{001b}G▒\u{001b}H \u{001b}C▒\u{001b}H \u{001b}C ▒▒\u{001b}A║").unwrap();
    write!(context, "║\u{001b}D░ ░\u{001b}H \u{001b}D░\u{001b}H \u{001b}D\u{001b}b░░░░░\u{001b}H\u{001b}h \u{001b}B░   ░\u{001b}H \u{001b}F░░░░░\u{001b}H \u{001b}F  \u{001b}b░\u{001b}h  \u{001b}H \u{001b}G\u{001b}f░\u{001b}h  \u{001b}f░\u{001b}H\u{001b}h \u{001b}G░░░░░\u{001b}H \u{001b}C░   ░\u{001b}A║").unwrap();
    write!(context, "╚══════════════════════════════════════════════╝").unwrap();
    write!(context, "\n\n\nPress a key...").unwrap();
    loop {
        asm::wfi();
        if let Some(_input) = context.read() {
            writeln!(context, "\u{001b}A\u{001b}g\u{001b}ZOk...").unwrap();
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
    let attr = unsafe { FRAMEBUFFER.set_attr(fb::Attr::new(fb::Colour::Black, fb::Colour::Yellow)) };
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
        if let Some(_input) = context.read() {
            unsafe { FRAMEBUFFER.set_attr(attr) };
            break;
        }
    }
}

fn item_peek<'a>(_menu: &Menu, _item: &Item, input: &str, context: &mut Context) {
    let mut parts = input.split_whitespace();
    parts.next();
    if let Some(addr) = parts.next().map_or(None, |p| usize::from_str_radix(p, 16).ok()) {
        unsafe {
            let data = ::core::ptr::read_volatile(addr as *const u32);
            writeln!(context, "Addr 0x{:08x} is 0x{:08x}", addr, data).unwrap();
        }
    } else {
        writeln!(context, "Bad address {:?}. Enter hex, without the 0x prefix..", input).unwrap();
    }
}

fn item_poke<'a>(_menu: &Menu, _item: &Item, input: &str, context: &mut Context) {
    let mut parts = input.split_whitespace();
    parts.next();
    if let Some(addr) = parts.next().map_or(None, |p| usize::from_str_radix(p, 16).ok()) {
        if let Some(value) = parts.next().map_or(None, |p| u32::from_str_radix(p, 16).ok()) {
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
    if let Some(mut addr) = parts.next().map_or(None, |p| usize::from_str_radix(p, 16).ok()) {
        if let Some(count) = parts.next().map_or(None, |p| u32::from_str_radix(p, 16).ok()) {
            writeln!(context, "Dumping 0x{:08x} bytes from 0x{:08x}...", count, addr).unwrap();
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



// End of file
