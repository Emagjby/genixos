static CONSOLE: Console = Console::new();

use core::fmt::Write;

use crate::{
    console::vga::{VgaColor, VgaWriter},
    sync::spinlock::SpinLock,
};

pub mod vga;

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

pub fn set_color(bg: VgaColor, fg: VgaColor) {
    let mut guard = CONSOLE.writer.lock();
    guard.set_color(bg, fg);
}

pub fn reset_color() {
    set_color(VgaColor::Black, VgaColor::White);
}

pub fn clear() {
    let mut guard = CONSOLE.writer.lock();
    guard.clear();
}

pub fn set_cursor(x: usize, y: usize) {
    let mut guard = CONSOLE.writer.lock();
    guard.set_cursor(x, y);
}

pub fn try_with_console(f: impl FnOnce(&mut VgaWriter)) -> Option<()> {
    let mut guard = CONSOLE.writer.try_lock()?;
    f(&mut guard);
    Some(())
}

pub fn _print(args: core::fmt::Arguments) {
    let mut guard = CONSOLE.writer.lock();
    guard.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    }
}
