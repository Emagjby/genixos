use crate::arch::x86_64::port::write_u8;

const PIT_CHANNEL_PORT: u16 = 0x40;
const PIT_COMMAND_PORT: u16 = 0x43;

const PIT_FREQUENCY: u32 = 1_193_182;

pub struct Pit {
    channel_port: u16,
    command_port: u16,
}

impl Pit {
    pub const fn new() -> Self {
        Self {
            channel_port: PIT_CHANNEL_PORT,
            command_port: PIT_COMMAND_PORT,
        }
    }

    pub fn configure_periodic(&self, frequency_hz: u32) {
        assert!(frequency_hz > 0, "frequency must be greater than 0");

        let divisor = PIT_FREQUENCY / frequency_hz;
        assert!(
            (2..=65535).contains(&divisor),
            "divisor must be between 2 and 65535"
        );

        unsafe {
            write_u8(self.command_port, 0x34);

            write_u8(self.channel_port, (divisor & 0xFF) as u8);
            write_u8(self.channel_port, ((divisor >> 8) & 0xFF) as u8);
        }
    }
}
