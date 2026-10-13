static STARTUP_MESSAGE: &str = "Loading GenixOS...\n";

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const BYTES_PER_ROW: usize = VGA_WIDTH * 2;
const LAST_ROW_BYTE_OFFSET: usize = (VGA_HEIGHT - 1) * BYTES_PER_ROW;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub struct VgaWriter {
    column: usize,
    row: usize,
    fg: VgaColor,
    bg: VgaColor,
    buffer: *mut u8,
}

#[allow(dead_code)]
impl VgaWriter {
    pub fn new() -> Self {
        Self {
            column: 0,
            row: 0,
            fg: VgaColor::White,
            bg: VgaColor::Black,
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
            0x20..=0x7E => {
                self.write_to_buffer(self.get_pos(), byte);
                self.iterate_pos()
            }
            _ => {
                self.write_to_buffer(self.get_pos(), b'?');
                self.iterate_pos()
            }
        }
    }

    pub fn set_color(&mut self, bg: VgaColor, fg: VgaColor) {
        self.bg = bg;
        self.fg = fg;
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        if x >= VGA_WIDTH || y >= VGA_HEIGHT {
            return; // TODO: error
        }

        self.column = x;
        self.row = y;
    }

    fn get_color(&self) -> u8 {
        ((self.bg as u8) << 4) | (self.fg as u8)
    }

    fn write_to_buffer(&mut self, pos: usize, byte: u8) {
        unsafe {
            core::ptr::write_volatile(self.buffer.add(pos * 2), byte);
            core::ptr::write_volatile(self.buffer.add(pos * 2 + 1), self.get_color());
        }
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

        self.clear_row(VGA_HEIGHT - 1);

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

    pub fn clear(&mut self) {
        for i in 0..VGA_HEIGHT {
            self.clear_row(i);
        }

        self.set_cursor(0, 0);
    }

    pub fn clear_row(&mut self, row: usize) {
        if row >= VGA_HEIGHT {
            return; // TODO: error
        }

        for i in 0..VGA_WIDTH {
            self.clear_point(row * VGA_WIDTH + i);
        }
    }

    pub fn clear_point(&mut self, pos: usize) {
        if pos >= VGA_WIDTH * VGA_HEIGHT {
            return; // TODO: error
        }

        self.write_to_buffer(pos, b' ');
    }
}

impl core::fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s);
        Ok(())
    }
}

fn char_to_u8(input: char) -> u8 {
    match input {
        ' '..='~' => input as u8,
        '\n' => b'\n',
        _ => b'?',
    }
}
