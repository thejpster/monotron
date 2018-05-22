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

struct Fire {
    seed: u32,
    buffer: [u32; Fire::FLAME_BUFFER_LEN]
}

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
    write!(context, "║\u{001b}A│ ☺☻♥♦♣♠•◘○◙♂♀♪♫☼││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│►◄↕‼¶§▬↨↑↓→←∟↔▲▼││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│ !\"#$%&'()*+,-./││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│0123456789:;<=>?││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│@ABCDEFGHIJKLMNO││\u{001b}B\u{001b}g                          \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│PQRSTUVWXYZ[\\]^_││\u{001b}B\u{001b}g          ▓▓▓▓▓▓▓         \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│`abcdefghijklmno││\u{001b}B\u{001b}g        ▓▓▓▓▓▓▓▓▓▓▓       \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│pqrstuvwxyz{{|}}~ ││\u{001b}B\u{001b}g       ▓▓▓▓▓▓▓▓▓▓▓▓▓      \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│ÇüéâäàåçêëèïîìÄÅ││\u{001b}B\u{001b}g     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│ÉæÆôöòûùÿÖÜø£Ø×ƒ││\u{001b}B\u{001b}g     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│áíóúñÑªº¿®¬½¼¡«»││\u{001b}B\u{001b}g    ▓▓▓\u{001b}H\u{001b}h▓▓▓\u{001b}B\u{001b}g▓▓▓▓▓▓\u{001b}H\u{001b}h▓▓▓\u{001b}B\u{001b}g▓▓▓▓   \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│░▒▓│┤ÁÂÀ©╣║╗╝¢¥┐││\u{001b}B\u{001b}g   ▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓  \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│└┴┬├─┼ãÃ╚╔╩╦╠═╬¤││\u{001b}B\u{001b}g   ▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓  \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│ðÐÊËÈıÍÎÏ┘┌█▄¦Ì▀││\u{001b}B\u{001b}g  ▓▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓\u{001b}H\u{001b}h▓▓▓▓▓\u{001b}B\u{001b}g▓▓▓▓▓ \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│ÓßÔÒõÕµþÞÚÛÙýÝ¯´││\u{001b}B\u{001b}g  ▓▓▓▓▓\u{001b}H\u{001b}h▓▓▓\u{001b}B\u{001b}g▓▓▓▓▓▓\u{001b}H\u{001b}h▓▓▓\u{001b}B\u{001b}g▓▓▓▓▓▓ \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
    write!(context, "║\u{001b}A│-±‗¾¶§÷¸°¨·¹³²■\u{00a0}││\u{001b}B\u{001b}g  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ \u{001b}A\u{001b}h│\u{001b}B║").unwrap();
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
    write!(context, "╚══════════════════════════════════════════════").unwrap();

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
    write!(context, "          by theJPster / @therealjpster").unwrap();
    let mut pos = fb::Position::new(fb::Row(6), fb::Col(0));
    let mut next_frame = unsafe { FRAMEBUFFER.frame() } + 1;
    let mut f = Fire::new();
    loop {
        asm::wfi();
        let this_frame = unsafe { FRAMEBUFFER.frame() };
        if this_frame == next_frame {
            next_frame = this_frame + 1;
            f.draw_fire(unsafe { &mut FRAMEBUFFER });
        }
        match unsafe { FRAMEBUFFER.line() } {
            Some(95) => {
                for col in 13..=21 {
                    pos.col = fb::Col(col);
                    unsafe { FRAMEBUFFER.set_attr_at(pos, fb::Attr::new(fb::Colour::Blue, fb::Colour::Black)); }
                }
            }
            Some(101) => {
                for col in 13..=21 {
                    pos.col = fb::Col(col);
                    unsafe { FRAMEBUFFER.set_attr_at(pos, fb::Attr::new(fb::Colour::White, fb::Colour::Black)); }
                }
            }
            Some(104) => {
                for col in 13..=21 {
                    pos.col = fb::Col(col);
                    unsafe { FRAMEBUFFER.set_attr_at(pos, fb::Attr::new(fb::Colour::Red, fb::Colour::Black)); }
                }
            }
            _ => {},
        }
        if let Some(_input) = context.read() {
            writeln!(context, "\u{001b}A\u{001b}g\u{001b}ZOk...").unwrap();
            break;
        }
    }
}

impl Fire {
    const WIDTH: usize = 48;
    const HEIGHT: usize = 20;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;
    const FLAME_BUFFER_LEN: usize = Self::SIZE + Self::WIDTH + 1;

    fn new() -> Fire {
        Fire {
            seed: 123456789,
            buffer: [0u32; Self::FLAME_BUFFER_LEN]
        }
    }

    /// Draws a flame effect.
    /// Based on https://gist.github.com/msimpson/1096950.
    fn draw_fire(&mut self, fb: &mut fb::FrameBuffer<super::VideoHardware>) {
        use fb::Glyph;
        const CHARS: [Glyph; 10] = [
            Glyph::Space,
            Glyph::FullStop,
            Glyph::Colon,
            Glyph::CircumflexAccent,
            Glyph::Asterisk,
            Glyph::LatinSmallLetterX,
            Glyph::LatinSmallLetterS,
            Glyph::LatinCapitalLetterS,
            Glyph::NumberSign,
            Glyph::DollarSign
        ];
        // Seed the fire on the last line
        for _i in 0..5 {
            let idx = (Self::WIDTH*(Self::HEIGHT-1)) + self.random_up_to(Self::WIDTH as u32) as usize;
            self.buffer[idx] = 65;
        }
        // Cascade the flames
        for i in 0..Self::SIZE {
            self.buffer[i] = (self.buffer[i] + self.buffer[i+1] + self.buffer[i+Self::WIDTH] + self.buffer[i+Self::WIDTH+1]) / 4;
            let colour = if self.buffer[i] > 15 {
                fb::Colour::Blue
            } else if self.buffer[i] > 9 {
                fb::Colour::Red
            } else if self.buffer[i] > 4 {
                fb::Colour::Yellow
            } else {
                fb::Colour::White
            };
            let glyph = if self.buffer[i] > 9 {
                CHARS[9]
            } else {
                CHARS[self.buffer[i] as usize]
            };
            let pos = fb::Position::new(fb::Row(((i / Self::WIDTH) as u8) + 16), fb::Col((i % Self::WIDTH) as u8));
            fb.write_glyph_at(glyph, pos, Some(fb::Attr::new(colour, fb::Colour::Black)));
        }
    }

    /// Generates a number in the range [0, limit)
    fn random_up_to(&mut self, limit: u32) -> u32 {
        let buckets = ::core::u32::MAX / limit;
        let upper_edge = buckets * limit;
        loop {
            let try = self.random();
            if try < upper_edge {
                return try / buckets;
            }
        }
    }

    /// Generate a random 32-bit number
    fn random(&mut self) -> u32 {
        self.seed = (self.seed.wrapping_mul(1103515245)).wrapping_add(12345);
        self.seed
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
