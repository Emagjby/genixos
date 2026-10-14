pub mod macros;
pub mod vga;

static CONSOLE: Console = Console::new();

use core::fmt::Write;

use crate::{
    console::vga::{VgaColor, VgaWriter},
    sync::spinlock::SpinLock,
};

pub struct Console {
    writer: SpinLock<VgaWriter>,
}

impl Console {
    pub const fn new() -> Self {
        Self {
            writer: SpinLock::new(VgaWriter::new()),
        }
    }
}

pub fn _set_color(bg: VgaColor, fg: VgaColor) {
    let mut guard = CONSOLE.writer.lock();
    guard.set_color(bg, fg);
}

pub fn _reset_color() {
    _set_color(VgaColor::Black, VgaColor::White);
}

pub fn clear() {
    let mut guard = CONSOLE.writer.lock();
    guard.clear();
}

pub fn set_cursor(x: usize, y: usize) {
    let mut guard = CONSOLE.writer.lock();
    guard.set_cursor(x, y);
}

pub fn with_console(f: impl FnOnce(&mut VgaWriter)) {
    let mut guard = CONSOLE.writer.lock();
    f(&mut guard);
}

pub fn try_with_console(f: impl FnOnce(&mut VgaWriter)) -> Option<()> {
    let mut guard = CONSOLE.writer.try_lock()?;
    f(&mut guard);
    Some(())
}

pub fn _print(args: core::fmt::Arguments) {
    let mut guard = CONSOLE.writer.lock();
    let _ = guard.write_fmt(args);
}
