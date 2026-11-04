use crate::arch::x86_64::port::read_u8;

const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64;

pub struct Ps2Keyboard;

impl Ps2Keyboard {
    pub const fn new() -> Self {
        Self
    }

    pub fn read_status(&self) -> u8 {
        unsafe { read_u8(STATUS_PORT) }
    }

    pub fn read_scancode(&self) -> Option<u8> {
        if self.read_status() & 0x01 == 0 {
            return None;
        }

        unsafe { Some(read_u8(DATA_PORT)) }
    }
}
