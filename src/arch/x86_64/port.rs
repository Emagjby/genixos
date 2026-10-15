pub unsafe fn read_u8(port: u16) -> u8 {
    let mut out: u8;

    unsafe {
        core::arch::asm!(
            "in al, dx",
            in("dx") port,
            out("al") out,
        )
    }

    out
}

pub unsafe fn write_u8(port: u16, value: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") value
        );
    }
}
