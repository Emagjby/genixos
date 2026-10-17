// SAFETY:
// The bootloader must map virtual adress 0xB8000 to VGA text memory
// with write permissions. Genix currently relies on this mapping
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
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
}

#[allow(dead_code)]
impl VgaWriter {
    pub const fn new() -> Self {
        Self {
            column: 0,
            row: 0,
            fg: VgaColor::White,
            bg: VgaColor::Black,
        }
    }

    pub fn write(&mut self, input: &str) -> Result<(), ()> {
        for byte in input.chars() {
            Self::write_byte(self, char_to_u8(byte))?;
        }

        Ok(())
    }

    pub fn write_byte(&mut self, byte: u8) -> Result<(), ()> {
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

    pub fn set_cursor(&mut self, x: usize, y: usize) -> Result<(), ()> {
        if x >= VGA_WIDTH || y >= VGA_HEIGHT {
            return Err(());
        }

        self.column = x;
        self.row = y;
        Ok(())
    }

    fn get_color(&self) -> u8 {
        ((self.bg as u8) << 4) | (self.fg as u8)
    }

    fn write_to_buffer(&mut self, pos: usize, byte: u8) {
        // SAFETY:
        // The caller must ensure pos < VGA_WIDTH * VGA_HEIGHT.
        // Each VGA cell occupies two bytes, so both writes must remain within
        // the mapped VGA text buffer. The caller must also ensure that
        // accesses are synchronized.
        unsafe {
            core::ptr::write_volatile(VGA_BUFFER.add(pos * 2), byte);
            core::ptr::write_volatile(VGA_BUFFER.add(pos * 2 + 1), self.get_color());
        }
    }

    fn get_pos(&self) -> usize {
        self.column + self.row * VGA_WIDTH
    }

    fn iterate_pos(&mut self) -> Result<(), ()> {
        self.column += 1;

        if self.column >= VGA_WIDTH {
            self.newline()?;
        }

        Ok(())
    }

    fn scroll(&mut self) -> Result<(), ()> {
        for i in 0..LAST_ROW_BYTE_OFFSET {
            // SAFETY:
            // Source and destination offsets remain within the 80x25 VGA buffer.
            // The forward copy direction is safe for shifting rows upward.
            // Volatile operations preserve the required device-memory accesses.
            // Exclusive access must be guaranteed by the caller.
            unsafe {
                core::ptr::write_volatile(
                    VGA_BUFFER.add(i),
                    core::ptr::read_volatile(VGA_BUFFER.add(VGA_WIDTH * 2 + i)),
                );
            }
        }

        self.clear_row(VGA_HEIGHT - 1)?;

        self.column = 0;
        self.row -= 1;

        Ok(())
    }

    fn newline(&mut self) -> Result<(), ()> {
        self.column = 0;
        self.row += 1;

        if self.row >= VGA_HEIGHT {
            self.scroll()?;
        }

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), ()> {
        for i in 0..VGA_HEIGHT {
            self.clear_row(i)?;
        }

        self.set_cursor(0, 0)?;

        Ok(())
    }

    pub fn clear_row(&mut self, row: usize) -> Result<(), ()> {
        if row >= VGA_HEIGHT {
            return Err(());
        }

        for i in 0..VGA_WIDTH {
            self.clear_point(row * VGA_WIDTH + i)?;
        }

        Ok(())
    }

    pub fn clear_point(&mut self, pos: usize) -> Result<(), ()> {
        if pos >= VGA_WIDTH * VGA_HEIGHT {
            return Err(());
        }

        self.write_to_buffer(pos, b' ');

        Ok(())
    }
}

impl core::fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s).map_err(|_| core::fmt::Error)
    }
}

fn char_to_u8(input: char) -> u8 {
    match input {
        ' '..='~' => input as u8,
        '\n' => b'\n',
        _ => b'?',
    }
}
