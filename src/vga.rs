static STARTUP_MESSAGE: &[u8] = b"Loading GenixOS...";

pub fn write_startup_message() {
    let vga_buffer = 0xb8000 as *mut u8; // pointer to the VGA text buffer

    for (i, &byte) in STARTUP_MESSAGE.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // write the byte to the VGA buffer
            *vga_buffer.offset(i as isize * 2 + 1) = 0xF; // set the color to white on black
        }
    }
}
