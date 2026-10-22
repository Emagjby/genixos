use crate::{arch::x86_64::port, sync::spinlock::SpinLock};

const COM1_ADDRESS: u16 = 0x3F8;

static SERIAL1: SpinLock<SerialPort> = SpinLock::new(SerialPort::new(COM1_ADDRESS));

pub struct SerialPort {
    base_port: u16,
}

impl SerialPort {
    pub const fn new(base_port: u16) -> Self {
        Self { base_port }
    }

    pub fn init(&self) {
        // SAFETY:
        // base_port must identify an accessible 16550-compatible UART.
        // The register offsets and intialization sequence follow the
        // UART's hardware interface. Initialization must not race with
        // other accesses to the same device.
        unsafe {
            // disable interrupts
            port::write_u8(self.base_port + 1, 0x00);

            // baud rate
            port::write_u8(self.base_port + 3, 0x80); // DLAB on
            port::write_u8(self.base_port, 0x03); // Low byte
            port::write_u8(self.base_port + 1, 0x00); // High byte
            port::write_u8(self.base_port + 3, 0x03); // DLAB off & 8N1

            // fifo
            port::write_u8(self.base_port + 2, 0xC7);

            // modem control signals
            port::write_u8(self.base_port + 4, 0x0B);
        }
    }

    pub fn write_byte(&self, byte: u8) {
        // SAFETY:
        // The UART must be initialized and accessible.
        // Register offset +5 contains the line status register.
        // Bit 5 indicates transmitter readiness.
        // The transmit register is accessed at offset +0 with DLAB cleared.
        // Accesses must be synchronized with other users of this UART.
        unsafe {
            loop {
                if (port::read_u8(self.base_port + 5) & 0x20) != 0 {
                    break port::write_u8(self.base_port, byte);
                }
            }
        }
    }
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        for byte in text.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

pub fn with_serial1(f: impl FnOnce(&mut SerialPort)) {
    let mut guard = SERIAL1.lock();
    f(&mut guard);
}

pub fn try_with_serial1(f: impl FnOnce(&mut SerialPort)) -> Option<()> {
    let mut guard = SERIAL1.try_lock()?;
    f(&mut guard);
    Some(())
}

#[macro_export]
macro_rules! serialprint {
    ($($arg:tt)*) => {{
        use core::fmt::Write;

        $crate::drivers::serial::with_serial1(|s| {
            let _ = s.write_fmt(format_args!($($arg)*));
        });
    }};
}

#[macro_export]
macro_rules! serialprintln {
    () => {
        $crate::serialprint!("\n");
    };
    ($($arg:tt)*) => {
        $crate::serialprint!("{}\n", format_args!($($arg)*));
    }
}
