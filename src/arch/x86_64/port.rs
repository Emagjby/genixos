/// Reads one byte from an x86 I/0 port.
///
/// # Safety
///
/// The caller must ensure that the CPU has permission to access the
/// specified port and that reading it is appropriate for the device.
/// Some hardware registers have side effects when read.
pub unsafe fn read_u8(port: u16) -> u8 {
    let mut out: u8;

    // SAFETY:
    // The caller guarantees valid port access.
    // DX contains the 16-bit port address, and AL receives the byte.
    unsafe {
        core::arch::asm!(
            "in al, dx",
            in("dx") port,
            out("al") out,
        )
    }

    out
}

/// Writes one byte to an x86 I/O port.
///
/// # Safety
///
/// The caller must ensure that the CPU has permission to access the
/// specified port and that writing this value is appropriate for
/// the target hardware.
pub unsafe fn write_u8(port: u16, value: u8) {
    // SAFETY:
    // the caller guarantees valid port access.
    // DX contains the 16-bit port address, and AL contains the byte.
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") value
        );
    }
}

pub unsafe fn io_wait() {
    unsafe {
        write_u8(0x80, 0);
    }
}
