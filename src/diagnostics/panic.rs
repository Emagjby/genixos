use core::{
    fmt::{Arguments, Write},
    panic::PanicInfo,
};

use crate::console::{self, vga::VgaColor};

pub fn report_panic(info: &PanicInfo) {
    let args: Arguments = format_args!(
        r#"========================================
GENIX KERNEL PANIC
========================================

Message: {}
Location: {:?}

SYSTEM HALTED
"#,
        info.message(),
        info.location(),
    );

    console::try_with_console(|vga_writer| {
        vga_writer.clear().ok();
        vga_writer.set_color(VgaColor::Black, VgaColor::Red);
        let _ = vga_writer.write_fmt(args);
    });
}
