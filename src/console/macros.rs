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
        $crate::print!("{}\n", format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! cprint {
    ($fg:ident, $bg:ident; $($arg:tt)*) => {{
        use core::fmt::Write;

        $crate::console::with_console(|writer| {
            writer.set_color(
                $crate::console::vga::VgaColor::$bg,
                $crate::console::vga::VgaColor::$fg,
            );
            let _ = writer.write_fmt(format_args!($($arg)*));
            writer.set_color(
                $crate::console::vga::VgaColor::Black,
                $crate::console::vga::VgaColor::White
            );
        })
    }};
    ($fg:ident; $($arg:tt)*) => {{
        use core::fmt::Write;

        $crate::console::with_console(|writer| {
            writer.set_color(
                $crate::console::vga::VgaColor::Black,
                $crate::console::vga::VgaColor::$fg,
            );
            let _ = writer.write_fmt(format_args!($($arg)*));
            writer.set_color(
                $crate::console::vga::VgaColor::Black,
                $crate::console::vga::VgaColor::White
            );
        })
    }};
}

#[macro_export]
macro_rules! cprintln {
    ($fg:ident, $bg:ident; $($args:tt)*) => {{
        $crate::cprint!($fg, $bg; "{}\n", format_args!($($args)*));
    }};
    ($fg:ident; $($args:tt)*) => {{
        $crate::cprint!($fg; "{}\n", format_args!($($args)*));
    }};
}
