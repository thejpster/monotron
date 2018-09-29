use super::{Context, FRAMEBUFFER};
use asm;
use core::fmt::Write;
use fb;
use fb::BaseConsole;
use rust_logo;
use ui;

struct Fire {
    seed: u32,
    buffer: [u32; Fire::FLAME_BUFFER_LEN],
}

/// The test menu item - displays all the glyphs in all the colour combinations
pub(crate) fn test_alphabet<'a>(_menu: &ui::Menu, _item: &ui::Item, _input: &str, context: &mut Context) {
    let mut old_frame = 0;
    let mut ch = 0u8;
    const COLOURS: [fb::Colour; 8] = [
        fb::Colour::Blue,
        fb::Colour::Black,
        fb::Colour::Red,
        fb::Colour::Magenta,
        fb::Colour::Green,
        fb::Colour::Yellow,
        fb::Colour::Cyan,
        fb::Colour::White,
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
                FRAMEBUFFER.write_glyph(
                    fb::Char::from_byte(ch),
                    Some(fb::Attr::new(*fg.unwrap(), *bg.unwrap())),
                );
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

/// Display some art
pub(crate) fn test_art<'a>(_menu: &ui::Menu, _item: &ui::Item, _input: &str, context: &mut Context) {
    unsafe {
        FRAMEBUFFER.clear();
    }
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
    writeln!(context, "\u{001b}Y\u{001b}k\u{001b}Z").unwrap();
    writeln!(context, "              1UP   HIGH SCORE").unwrap();
    writeln!(context, "                00        00").unwrap();
    writeln!(context, "\u{001b}G          ╔════════════╦╦════════════╗").unwrap();
    writeln!(context, "          ║\u{001b}C············\u{001b}G║║\u{001b}C············\u{001b}G║").unwrap();
    writeln!(context, "          ║\u{001b}C·\u{001b}G┌──┐\u{001b}C·\u{001b}G┌───┐\u{001b}C·\u{001b}G║║\u{001b}C·\u{001b}G┌───┐\u{001b}C·\u{001b}G┌──┐\u{001b}C·\u{001b}G║").unwrap();
    writeln!(context, "          ║○│  │\u{001b}C·\u{001b}G│   │\u{001b}C·\u{001b}G║║\u{001b}C·\u{001b}G│   │\u{001b}C·\u{001b}G│  │○║").unwrap();
    writeln!(context, "          ║\u{001b}C·\u{001b}G└──┘\u{001b}C·\u{001b}G└───┘\u{001b}C·\u{001b}G╚╝\u{001b}C·\u{001b}G└───┘\u{001b}C·\u{001b}G└──┘\u{001b}C·\u{001b}G║").unwrap();
    writeln!(context, "          ║\u{001b}C··························\u{001b}G║").unwrap();
    writeln!(context, "          ║\u{001b}C·\u{001b}G┌──┐\u{001b}C·\u{001b}G┌┐\u{001b}C·\u{001b}G┌──────┐\u{001b}C·\u{001b}G┌┐\u{001b}C·\u{001b}G┌──┐\u{001b}C·\u{001b}G║").unwrap();
    writeln!(context, "          ║\u{001b}C·\u{001b}G└──┘\u{001b}C·\u{001b}G││\u{001b}C·\u{001b}G└──┐┌──┘\u{001b}C·\u{001b}G││\u{001b}C·\u{001b}G└──┘\u{001b}C·\u{001b}G║").unwrap();
    writeln!(context, "          ║\u{001b}C······\u{001b}G││\u{001b}C····\u{001b}G││\u{001b}C····\u{001b}G││\u{001b}C······\u{001b}G║").unwrap();
    writeln!(context, "          ╚════╗\u{001b}C·\u{001b}G│└──┐ ││ ┌──┘│\u{001b}C·\u{001b}G╔════╝").unwrap();
    writeln!(context, "               ║\u{001b}C·\u{001b}G│┌──┘ └┘ └──┐│\u{001b}C·\u{001b}G║     ").unwrap();
    writeln!(context, "               ║\u{001b}C·\u{001b}G││    \u{001b}Y☺\u{001b}G     ││\u{001b}C·\u{001b}G║     ").unwrap();
    writeln!(context, "          ═════╝\u{001b}C·\u{001b}G└┘ ╔══════╗ └┘\u{001b}C·\u{001b}G╚═════").unwrap();
    writeln!(context, "                \u{001b}C·\u{001b}G   ║\u{001b}R☺ \u{001b}G☺ \u{001b}B☺\u{001b}G ║   \u{001b}C·\u{001b}G      ").unwrap();
    writeln!(context, "          ═════╗\u{001b}C·\u{001b}G┌┐ ╚══════╝ ┌┐\u{001b}C·\u{001b}G╔═════").unwrap();
    writeln!(context, "               ║\u{001b}C·\u{001b}G││  READY!  ││\u{001b}C·\u{001b}G║     ").unwrap();
    writeln!(context, "               ║\u{001b}C·\u{001b}G││ ┌──────┐ ││\u{001b}C·\u{001b}G║     ").unwrap();
    writeln!(context, "          ╔════╝\u{001b}C·\u{001b}G└┘ └──┐┌──┘ └┘\u{001b}C·\u{001b}G╚════╗").unwrap();
    writeln!(context, "          ║\u{001b}C············\u{001b}G││\u{001b}C············\u{001b}G║").unwrap();
    writeln!(context, "          ║\u{001b}C·\u{001b}G┌──┐\u{001b}C·\u{001b}G┌───┐\u{001b}C·\u{001b}G││\u{001b}C·\u{001b}G┌───┐\u{001b}C·\u{001b}G┌──┐\u{001b}C·\u{001b}G║").unwrap();
    writeln!(context, "          ║\u{001b}C·\u{001b}G└─┐│\u{001b}C·\u{001b}G└───┘\u{001b}C·\u{001b}G└┘\u{001b}C·\u{001b}G└───┘\u{001b}C·\u{001b}G│┌─┘\u{001b}C·\u{001b}G║").unwrap();
    writeln!(context, "          ║○\u{001b}C··\u{001b}G││\u{001b}C·······\u{001b}Y◄►\u{001b}C·······\u{001b}G││\u{001b}C··\u{001b}G○║").unwrap();
    writeln!(context, "          ╠═╗\u{001b}C·\u{001b}G││\u{001b}C·\u{001b}G┌┐\u{001b}C·\u{001b}G┌──────┐\u{001b}C·\u{001b}G┌┐\u{001b}C·\u{001b}G││\u{001b}C·\u{001b}G╔═╣").unwrap();
    writeln!(context, "          ╠═╝\u{001b}C·\u{001b}G└┘\u{001b}C·\u{001b}G││\u{001b}C·\u{001b}G└──┐┌──┘\u{001b}C·\u{001b}G││\u{001b}C·\u{001b}G└┘\u{001b}C·\u{001b}G╚═╣").unwrap();
    writeln!(context, "          ║\u{001b}C······\u{001b}G││\u{001b}C····\u{001b}G││\u{001b}C····\u{001b}G││\u{001b}C······\u{001b}G║").unwrap();
    writeln!(context, "          ║\u{001b}C·\u{001b}G┌────┘└──┐\u{001b}C·\u{001b}G││\u{001b}C·\u{001b}G┌──┘└────┐\u{001b}C·\u{001b}G║").unwrap();
    writeln!(context, "          ║\u{001b}C·\u{001b}G└────────┘\u{001b}C·\u{001b}G└┘\u{001b}C·\u{001b}G└────────┘\u{001b}C·\u{001b}G║").unwrap();
    writeln!(context, "          ║\u{001b}C··························\u{001b}G║").unwrap();
    writeln!(context, "          ╚══════════════════════════╝").unwrap();
    writeln!(context, "             ◄► ◄► ◄►").unwrap();
    write!(context, "\n\n\nPress a key...").unwrap();
    loop {
        asm::wfi();
        if let Some(_input) = context.read() {
            break;
        }
    }

    write!(context, "\u{001b}Z\u{001b}K\u{001b}w  \u{001b}RF\u{001b}Kile   \u{001b}RE\u{001b}Kdit  \u{001b}RV\u{001b}Kiew   \u{001b}RT\u{001b}Kools  \u{001b}RH\u{001b}Kelp               ").unwrap();
    write!(context, "\u{001b}B\u{001b}c████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████\u{001b}W\u{001b}b┌[Calc]──X┐    \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "████████████████████████████\u{001b}W\u{001b}b│\u{001b}B\u{001b}k         \u{001b}W\u{001b}b│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m┌[Calendar]───────────X┐\u{001b}B\u{001b}c█\u{001b}W\u{001b}b│\u{001b}B\u{001b}k         \u{001b}W\u{001b}b│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}B                  \u{001b}W\u{0000}\u{001b}B  \u{001b}W │\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b│\u{001b}k12345678\u{001b}B \u{001b}W\u{001b}b│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}B                  \u{001b}W\u{0000}\u{001b}B  \u{001b}W │\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b│\u{001b}B\u{001b}k         \u{001b}W\u{001b}b│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│MonTueWedThuFriSatSun │\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b│\u{001b}B\u{001b}c█████████\u{001b}W\u{001b}b│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C┌────────────────────┐\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b│7 8 9 /\u{001b}B\u{001b}c█\u{001b}W\u{001b}bC│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C│\u{001b}W  \u{001b}C│\u{001b}W01\u{001b}C│\u{001b}W02\u{001b}C│\u{001b}W03\u{001b}C│\u{001b}W04\u{001b}C│\u{001b}W05\u{001b}C│\u{001b}W06\u{001b}C│\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b│\u{001b}B\u{001b}c█████████\u{001b}W\u{001b}b│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C├────────────────────┤\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b│4 5 6 ( )│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C│\u{001b}W07\u{001b}C│\u{001b}W08\u{001b}C│\u{001b}W09\u{001b}C│\u{001b}W10\u{001b}C│\u{001b}W11\u{001b}C│\u{001b}W12\u{001b}C│\u{001b}W13\u{001b}C│\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b│\u{001b}B\u{001b}c█████████\u{001b}W\u{001b}b│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C├────────────────────┤\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b│1 2 3 - ²│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C│\u{001b}W14\u{001b}C│\u{001b}W15\u{001b}C│\u{001b}W16\u{001b}C│\u{001b}W17\u{001b}C│\u{001b}W18\u{001b}C│\u{001b}W19\u{001b}C│\u{001b}W20\u{001b}C│\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b│\u{001b}B\u{001b}c█████████\u{001b}W\u{001b}b│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C├────────────────────┤\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b│0 . % + =│\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C│\u{001b}W21\u{001b}C│\u{001b}W22\u{001b}C│\u{001b}W23\u{001b}C│\u{001b}W24\u{001b}C│\u{001b}W25\u{001b}C│\u{001b}W26\u{001b}C│\u{001b}W27\u{001b}C│\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b└─────────┘\u{001b}B\u{001b}k▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C├────────────────────┤\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b \u{001b}B\u{001b}k▒▒▒▒▒▒▒▒▒▒▒\u{001b}c██\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C│\u{001b}W28\u{001b}C│\u{001b}W29\u{001b}C│\u{001b}W30\u{001b}C│\u{001b}W31\u{001b}C│\u{001b}B     \u{001b}W\u{0000}\u{001b}B  \u{001b}C│\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████████████\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C└────────────────────┘\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████████████\u{001b}W\u{001b}b \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}C\u{0011}\u{001b}B \u{001b}C\u{0011}\u{0011}\u{001b}B  \u{001b}YMay 2018\u{001b}B   \u{001b}C\u{0010}\u{0010}\u{001b}B  \u{001b}C\u{0010}\u{001b}W│\u{001b}B\u{001b}k▒\u{001b}W\u{001b}b               \u{001b}B\u{001b}c█████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m│\u{001b}B                  \u{001b}W\u{0000}\u{001b}B  \u{001b}W │\u{001b}B\u{001b}k▒\u{001b}c████████████████████").unwrap();
    write!(context, "███\u{001b}W\u{001b}m└──────────────────────┘\u{001b}B\u{001b}k▒\u{001b}c████████████████████").unwrap();
    write!(context, "████\u{001b}k▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒\u{001b}c████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "████████████████████████████████████████████████").unwrap();
    write!(context, "\u{001b}K\u{001b}w▒\u{001b}W\u{001b}kCAPS\u{001b}K\u{001b}w▒\u{001b}W\u{001b}kINS\u{001b}K\u{001b}w▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒\u{001b}W\u{001b}k[Version v1.2]\u{001b}K\u{001b}w").unwrap();

    loop {
        asm::wfi();
        if let Some(_input) = context.read() {
            writeln!(context, "\u{001b}W\u{001b}b\u{001b}ZOk...").unwrap();
            break;
        }
    }

    write!(context, "\u{001b}Z\u{001b}Y\u{001b}k╔════════════════════════\u{001b}W[MonotronPaint]\u{001b}Y═══════╗").unwrap();
    write!(context, "║\u{001b}W┌[Font]──────────┐┌──────────────────────────┐\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│ ☺☻♥♦♣♠•◘○◙♂♀♪♫☼││\u{001b}Y\u{001b}b                          \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│►◄↕‼¶§▬↨↑↓→←∟↔▲▼││\u{001b}Y\u{001b}b                          \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│ !\"#$%&'()*+,-./││\u{001b}Y\u{001b}b                          \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│0123456789:;<=>?││\u{001b}Y\u{001b}b                          \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│@ABCDEFGHIJKLMNO││\u{001b}Y\u{001b}b                          \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│PQRSTUVWXYZ[\\]^_││\u{001b}Y\u{001b}b          ▓▓▓▓▓▓▓         \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│`abcdefghijklmno││\u{001b}Y\u{001b}b        ▓▓▓▓▓▓▓▓▓▓▓       \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│pqrstuvwxyz{{|}}~ ││\u{001b}Y\u{001b}b       ▓▓▓▓▓▓▓▓▓▓▓▓▓      \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│ÇüéâäàåçêëèïîìÄÅ││\u{001b}Y\u{001b}b     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│ÉæÆôöòûùÿÖÜø£Ø×ƒ││\u{001b}Y\u{001b}b     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│áíóúñÑªº¿®¬½¼¡«»││\u{001b}Y\u{001b}b    ▓▓▓\u{001b}K\u{001b}k▓▓▓\u{001b}Y\u{001b}b▓▓▓▓▓▓\u{001b}K\u{001b}k▓▓▓\u{001b}Y\u{001b}b▓▓▓▓   \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│░▒▓│┤ÁÂÀ©╣║╗╝¢¥┐││\u{001b}Y\u{001b}b   ▓▓▓\u{001b}K\u{001b}k▓▓▓▓▓\u{001b}Y\u{001b}b▓▓▓▓\u{001b}K\u{001b}k▓▓▓▓▓\u{001b}Y\u{001b}b▓▓▓▓  \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│└┴┬├─┼ãÃ╚╔╩╦╠═╬¤││\u{001b}Y\u{001b}b   ▓▓▓\u{001b}K\u{001b}k▓▓▓▓▓\u{001b}Y\u{001b}b▓▓▓▓\u{001b}K\u{001b}k▓▓▓▓▓\u{001b}Y\u{001b}b▓▓▓▓  \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│ðÐÊËÈıÍÎÏ┘┌█▄¦Ì▀││\u{001b}Y\u{001b}b  ▓▓▓▓\u{001b}K\u{001b}k▓▓▓▓▓\u{001b}Y\u{001b}b▓▓▓▓\u{001b}K\u{001b}k▓▓▓▓▓\u{001b}Y\u{001b}b▓▓▓▓▓ \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│ÓßÔÒõÕµþÞÚÛÙýÝ¯´││\u{001b}Y\u{001b}b  ▓▓▓▓▓\u{001b}K\u{001b}k▓▓▓\u{001b}Y\u{001b}b▓▓▓▓▓▓\u{001b}K\u{001b}k▓▓▓\u{001b}Y\u{001b}b▓▓▓▓▓▓ \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│-±‗¾¶§÷¸°¨·¹³²■\u{00a0}││\u{001b}Y\u{001b}b  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W└────────────────┘│\u{001b}Y\u{001b}b  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W┌[Colour]┐┌[Menu]┐│\u{001b}Y\u{001b}b  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│\u{001b}R█\u{001b}G█\u{001b}B█\u{001b}Y█\u{001b}M█\u{001b}C█\u{001b}K█\u{001b}W█││L\u{001b}Mo\u{001b}Wad\u{001b}K  \u{001b}W││\u{001b}Y\u{001b}b  ▓▓▓▓▓\u{001b}K\u{001b}k▓\u{001b}Y\u{001b}b▓▓▓▓▓▓▓▓▓▓▓▓\u{001b}K\u{001b}k▓\u{001b}Y\u{001b}b▓▓▓▓ \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W└────────┘│\u{001b}MS\u{001b}Wave\u{001b}K  \u{001b}W││\u{001b}Y\u{001b}b  ▓▓▓▓▓\u{001b}K\u{001b}k▓▓\u{001b}Y\u{001b}b▓▓▓▓▓▓▓▓▓▓\u{001b}K\u{001b}k▓▓\u{001b}Y\u{001b}b▓▓▓▓ \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W┌[Tools]─┐│\u{001b}MP\u{001b}Wrint\u{001b}K \u{001b}W││\u{001b}Y\u{001b}b   ▓▓▓▓▓\u{001b}K\u{001b}k▓▓\u{001b}Y\u{001b}b▓▓▓▓▓▓▓▓▓\u{001b}K\u{001b}k▓\u{001b}Y\u{001b}b▓▓▓▓  \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│\u{001b}MR\u{001b}Wect\u{001b}K    \u{001b}W││\u{001b}ME\u{001b}Wxport││\u{001b}Y\u{001b}b   ▓▓▓▓▓▓\u{001b}K\u{001b}k▓\u{001b}Y\u{001b}b▓▓▓▓▓▓▓\u{001b}K\u{001b}k▓▓▓\u{001b}Y\u{001b}b▓▓▓▓  \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│\u{001b}MC\u{001b}Well\u{001b}K    \u{001b}W││C\u{001b}Ml\u{001b}Wear\u{001b}K \u{001b}W││\u{001b}Y\u{001b}b    ▓▓▓▓▓▓\u{001b}K\u{001b}k▓▓▓▓▓▓▓▓\u{001b}Y\u{001b}b▓▓▓▓▓   \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│\u{001b}ML\u{001b}Wine\u{001b}K    \u{001b}W││\u{001b}K      \u{001b}W││\u{001b}Y\u{001b}b     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│\u{001b}MF\u{001b}Will\u{001b}K    \u{001b}W│└──────┘│\u{001b}Y\u{001b}b     ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓    \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│\u{001b}M\u{001b}cT\u{001b}Kext    \u{001b}W\u{001b}k│┌[Layr]┐│\u{001b}Y\u{001b}b       ▓▓▓▓▓▓▓▓▓▓▓▓▓      \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│\u{001b}MC\u{001b}Wopy\u{001b}K    \u{001b}W││\u{001b}K      \u{001b}W││\u{001b}Y\u{001b}b        ▓▓▓▓▓▓▓▓▓▓▓       \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│\u{001b}MP\u{001b}Waste\u{001b}K   \u{001b}W││1  \u{001b}GH\u{001b}RL\u{001b}WX││\u{001b}Y\u{001b}b          ▓▓▓▓▓▓▓         \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W└────────┘│2  \u{001b}GH\u{001b}RL\u{001b}WX││\u{001b}Y\u{001b}b                          \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W┌[Info]──┐│3  \u{001b}RH\u{001b}GL\u{001b}WX││\u{001b}Y\u{001b}b                          \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│N:Test\u{001b}K  \u{001b}W││4  \u{001b}GHL\u{001b}WX││\u{001b}Y\u{001b}b                          \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W│S:48x36 ││\u{001b}K      \u{001b}W││\u{001b}Y\u{001b}b                          \u{001b}W\u{001b}k│\u{001b}Y║").unwrap();
    write!(context, "║\u{001b}W└────────┘└──────┘└──────────────────────────┘\u{001b}Y║").unwrap();
    write!(context, "╚══════════════════════════════════════════════").unwrap();

    loop {
        asm::wfi();
        if let Some(_input) = context.read() {
            break;
        }
    }

    let mode2_buffer = unsafe { &mut super::APPLICATION_RAM[0..384 * 288 / 8] };

    unsafe {
        FRAMEBUFFER.clear();
    }

    for (src, dest) in rust_logo::RUST_LOGO_DATA
        .iter()
        .zip(mode2_buffer.iter_mut())
    {
        // Our source is an X-Bitmap, which puts the pixels in LSB-first order.
        // We need MSB first order for Monotron.
        *dest = flip_byte(*src);
    }

    // Attach a graphical buffer at a scan-line. It is interpreted as
    // being a grid 48 bytes wide and as long as given. Each line
    // is output twice. We've attached it to the first scan-line.
    unsafe {
        FRAMEBUFFER.mode2(mode2_buffer, 0);
    }
    let mut start = 0;
    let mut up = true;

    let mut next_frame = unsafe { FRAMEBUFFER.frame() } + 1;
    loop {
        asm::wfi();
        let this_frame = unsafe { FRAMEBUFFER.frame() };
        if this_frame == next_frame {
            if up {
                start += 1;
                if start == 288 {
                    up = false;
                }
            } else {
                start -= 1;
                if start == 0 {
                    up = true;
                }
            }
            unsafe {
                FRAMEBUFFER.mode2_shift(start);
            }
            next_frame = this_frame + 1;
        }
        if let Some(_input) = context.read() {
            break;
        }
    }

    unsafe {
        FRAMEBUFFER.mode2_release();
    }

    write!(context, "\u{001b}Z\u{001b}W\u{001b}k╔══════════════════════════════════════════════╗").unwrap();
    write!(context, "║\u{001b}R█████\u{001b}K \u{001b}R\u{001b}y█████\u{001b}K\u{001b}k \u{001b}Y██  █\u{001b}K \u{001b}G█████\u{001b}K \u{001b}G\u{001b}y█\u{001b}k█\u{001b}y█\u{001b}k██\u{001b}K \u{001b}B████\u{001b}K \u{001b}B█████\u{001b}K \u{001b}M██  █\u{001b}W║").unwrap();
    write!(context, "║\u{001b}R▓\u{001b}K \u{001b}R▓\u{001b}K \u{001b}R▓\u{001b}K \u{001b}R\u{001b}y▓\u{001b}K\u{001b}k   \u{001b}R\u{001b}y▓\u{001b}K\u{001b}k \u{001b}Y▓\u{001b}K \u{001b}Y▓ ▓\u{001b}K \u{001b}G▓\u{001b}K   \u{001b}G▓\u{001b}K \u{001b}G \u{001b}K \u{001b}G\u{001b}y▓\u{001b}K\u{001b}k \u{001b}G \u{001b}K \u{001b}B\u{001b}g▓\u{001b}K\u{001b}k  \u{001b}B\u{001b}g▓\u{001b}K\u{001b}k \u{001b}B▓\u{001b}K   \u{001b}B▓\u{001b}K \u{001b}M▓\u{001b}K \u{001b}M▓ ▓\u{001b}W║").unwrap();
    write!(context, "║\u{001b}R▒\u{001b}K \u{001b}R▒\u{001b}K \u{001b}R▒\u{001b}K \u{001b}R\u{001b}y▒\u{001b}K\u{001b}k   \u{001b}R\u{001b}y▒\u{001b}K\u{001b}k \u{001b}Y▒\u{001b}K  \u{001b}Y▒▒\u{001b}K \u{001b}G▒\u{001b}K   \u{001b}G▒\u{001b}K \u{001b}G \u{001b}K \u{001b}G\u{001b}y▒\u{001b}K\u{001b}k \u{001b}G \u{001b}K \u{001b}B\u{001b}g▒\u{001b}K\u{001b}k \u{001b}B\u{001b}g▒\u{001b}k \u{001b}K \u{001b}B▒\u{001b}K   \u{001b}B▒\u{001b}K \u{001b}M▒\u{001b}K \u{001b}M ▒▒\u{001b}W║").unwrap();
    write!(context, "║\u{001b}R░ ░\u{001b}K \u{001b}R░\u{001b}K \u{001b}R\u{001b}y░░░░░\u{001b}K\u{001b}k \u{001b}Y░   ░\u{001b}K \u{001b}G░░░░░\u{001b}K \u{001b}G  \u{001b}y░\u{001b}k  \u{001b}K \u{001b}B\u{001b}g░\u{001b}k  \u{001b}g░\u{001b}K\u{001b}k \u{001b}B░░░░░\u{001b}K \u{001b}M░   ░\u{001b}W║").unwrap();
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
                    unsafe {
                        FRAMEBUFFER
                            .set_attr_at(pos, fb::Attr::new(fb::Colour::Blue, fb::Colour::Black));
                    }
                }
            }
            Some(101) => {
                for col in 13..=21 {
                    pos.col = fb::Col(col);
                    unsafe {
                        FRAMEBUFFER
                            .set_attr_at(pos, fb::Attr::new(fb::Colour::White, fb::Colour::Black));
                    }
                }
            }
            Some(104) => {
                for col in 13..=21 {
                    pos.col = fb::Col(col);
                    unsafe {
                        FRAMEBUFFER
                            .set_attr_at(pos, fb::Attr::new(fb::Colour::Red, fb::Colour::Black));
                    }
                }
            }
            _ => {}
        }
        if let Some(_input) = context.read() {
            writeln!(context, "\u{001b}W\u{001b}k\u{001b}ZOk...").unwrap();
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
            buffer: [0u32; Self::FLAME_BUFFER_LEN],
        }
    }

    /// Draws a flame effect.
    /// Based on https://gist.github.com/msimpson/1096950.
    fn draw_fire(&mut self, fb: &mut fb::FrameBuffer<super::VideoHardware>) {
        use fb::Char;
        const CHARS: [Char; 10] = [
            Char::Space,
            Char::FullStop,
            Char::Colon,
            Char::CircumflexAccent,
            Char::Asterisk,
            Char::LatinSmallLetterX,
            Char::LatinSmallLetterS,
            Char::LatinCapitalLetterS,
            Char::NumberSign,
            Char::DollarSign,
        ];
        // Seed the fire on the last line
        for _i in 0..5 {
            let idx =
                (Self::WIDTH * (Self::HEIGHT - 1)) + self.random_up_to(Self::WIDTH as u32) as usize;
            self.buffer[idx] = 65;
        }
        // Cascade the flames
        for i in 0..Self::SIZE {
            self.buffer[i] = (self.buffer[i]
                + self.buffer[i + 1]
                + self.buffer[i + Self::WIDTH]
                + self.buffer[i + Self::WIDTH + 1]) / 4;
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
            let pos = fb::Position::new(
                fb::Row(((i / Self::WIDTH) as u8) + 16),
                fb::Col((i % Self::WIDTH) as u8),
            );
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
pub(crate) fn test_animation<'a>(_menu: &ui::Menu, _item: &ui::Item, input: &str, context: &mut Context) {
    let mut old_frame = 0;
    let mut pos = fb::Position::origin();
    let mut left = true;
    let mut down = true;
    let width = unsafe { FRAMEBUFFER.get_width() };
    let height = unsafe { FRAMEBUFFER.get_height() };
    let input = input.trim_left_matches("animate ");
    let num_chars = input.chars().count();
    let attr =
        unsafe { FRAMEBUFFER.set_attr(fb::Attr::new(fb::Colour::Black, fb::Colour::Yellow)) };
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

fn flip_byte(mut b: u8) -> u8 {
    b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
    b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
    (b & 0xAA) >> 1 | (b & 0x55) << 1
}

