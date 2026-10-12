static STARTUP_MESSAGE: &str = "Loading GenixOS...\n";

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

pub struct VgaWriter {
    column: usize,
    row: usize,
    buffer: *mut u8,
}

impl VgaWriter {
    pub fn new() -> Self {
        Self {
            column: 0,
            row: 0,
            buffer: 0xb8000 as *mut u8,
        }
    }

    pub fn write(&mut self, input: &str) {
        for byte in input.bytes() {
            Self::write_byte(self, byte);
        }
    }

    pub fn write_startup_message(&mut self) {
        Self::write(self, STARTUP_MESSAGE);
    }

    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.newline();
            return;
        }

        let pos = self.get_pos();

        unsafe {
            *self.buffer.offset(pos * 2) = byte;
            *self.buffer.offset(pos * 2 + 1) = 0xF;
        }

        self.iterate_pos();
    }

    fn get_pos(&self) -> isize {
        (self.column + self.row * VGA_WIDTH) as isize
    }

    fn iterate_pos(&mut self) {
        self.column += 1;

        if self.column >= VGA_WIDTH {
            self.newline();
        }
    }

    fn scroll(&mut self) {
        unsafe {
            core::ptr::copy(
                self.buffer.add(VGA_WIDTH * 2),
                self.buffer,
                (VGA_HEIGHT - 1) * VGA_WIDTH * 2,
            );
        }

        for i in ((VGA_HEIGHT - 1) * VGA_WIDTH)..(VGA_HEIGHT * VGA_WIDTH) {
            unsafe {
                *self.buffer.offset(i as isize * 2) = b' ';
                *self.buffer.offset(i as isize * 2 + 1) = 0xF;
            }
        }

        self.column = 0;
        self.row -= 1;
    }

    fn newline(&mut self) {
        self.column = 0;
        self.row += 1;

        if self.row >= VGA_HEIGHT {
            self.scroll();
        }
    }
}
