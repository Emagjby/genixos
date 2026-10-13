static STARTUP_MESSAGE: &str = "Loading GenixOS...\n";

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const BYTES_PER_ROW: usize = VGA_WIDTH * 2;
const LAST_ROW_BYTE_OFFSET: usize = (VGA_HEIGHT - 1) * BYTES_PER_ROW;

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
        for byte in input.chars() {
            Self::write_byte(self, char_to_u8(byte));
        }
    }

    pub fn write_startup_message(&mut self) {
        Self::write(self, STARTUP_MESSAGE);
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(),
            0x20..=0x7E => self.write_to_buffer(byte),
            _ => self.write_to_buffer(b'?'),
        }
    }

    fn write_to_buffer(&mut self, byte: u8) {
        let pos = self.get_pos();

        unsafe {
            core::ptr::write_volatile(self.buffer.add(pos * 2), byte);
            core::ptr::write_volatile(self.buffer.add(pos * 2 + 1), 0xF);
        }

        self.iterate_pos();
    }

    fn get_pos(&self) -> usize {
        self.column + self.row * VGA_WIDTH
    }

    fn iterate_pos(&mut self) {
        self.column += 1;

        if self.column >= VGA_WIDTH {
            self.newline();
        }
    }

    fn scroll(&mut self) {
        for i in 0..LAST_ROW_BYTE_OFFSET {
            unsafe {
                core::ptr::write_volatile(
                    self.buffer.add(i),
                    core::ptr::read_volatile(self.buffer.add(VGA_WIDTH * 2 + i)),
                );
            }
        }

        for i in 0..VGA_WIDTH {
            unsafe {
                core::ptr::write_volatile(self.buffer.add(LAST_ROW_BYTE_OFFSET + i * 2), 0x20);
                core::ptr::write_volatile(self.buffer.add(LAST_ROW_BYTE_OFFSET + i * 2 + 1), 0xF);
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

fn char_to_u8(input: char) -> u8 {
    match input {
        ' '..='~' => input as u8,
        '\n' => b'\n',
        _ => b'?',
    }
}
