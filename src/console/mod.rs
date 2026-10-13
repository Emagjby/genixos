static CONSOLE: Console = Console::new();

use core::fmt::Write;

use crate::{console::vga::VgaWriter, sync::spinlock::SpinLock};

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
