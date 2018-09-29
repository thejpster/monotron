use super::{Context, Input, FRAMEBUFFER};
use asm;
use fb::AsciiConsole;

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

pub(crate) extern "C" fn putchar(_raw_ctx: *mut Context, ch: u8) -> i32 {
    unsafe { FRAMEBUFFER.write_character(ch).unwrap() };
    ch as i32
}

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

pub(crate) extern "C" fn kbhit(raw_ctx: *mut Context) -> i32 {
    let ctx = unsafe {
        &mut *raw_ctx
    };
    ctx.has_char() as i32
}
