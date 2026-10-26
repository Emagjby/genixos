use crate::arch::x86_64::port::{io_wait, read_u8, write_u8};

const MASTER_PIC_C: u16 = 0x20;
const MASTER_PIC_D: u16 = MASTER_PIC_C + 1;
const MASTER_PIC_V: u8 = 0x20;

const SLAVE_PIC_C: u16 = 0xA0;
const SLAVE_PIC_D: u16 = SLAVE_PIC_C + 1;
const SLAVE_PIC_V: u8 = 0x28;

const EOI: u8 = 0x20;

const ICW1_ICW4: u8 = 0x01;
const ICW1_SINGLE: u8 = 0x02;
const ICW1_INTERVAL4: u8 = 0x04;
const ICW1_LEVEL: u8 = 0x08;
const ICW1_INIT: u8 = 0x10;
const ICW4_8086: u8 = 0x01;
const ICW4_AUTO: u8 = 0x02;
const ICW4_BUF_SLAVE: u8 = 0x08;
const ICW4_BUF_MASTER: u8 = 0x0C;
const ICW4_SFNM: u8 = 0x10;
const CASCADE_IRQ: u8 = 2;

pub struct ChainedPics {
    master: Pic,
    slave: Pic,
}

impl ChainedPics {
    pub const fn new() -> Self {
        Self {
            master: Pic::new(MASTER_PIC_C, MASTER_PIC_D, MASTER_PIC_V),
            slave: Pic::new(SLAVE_PIC_C, SLAVE_PIC_D, SLAVE_PIC_V),
        }
    }

    pub fn initialize(&self) {
        unsafe {
            self.master.send_command(ICW1_INIT | ICW1_ICW4);
            io_wait();
            self.slave.send_command(ICW1_INIT | ICW1_ICW4);
            io_wait();
            self.master.send_data(MASTER_PIC_V);
            io_wait();
            self.slave.send_data(SLAVE_PIC_V);
            io_wait();
            self.master.send_data(1 << CASCADE_IRQ);
            io_wait();
            self.slave.send_data(CASCADE_IRQ);
            io_wait();

            self.master.send_data(ICW4_8086);
            io_wait();
            self.slave.send_data(ICW4_8086);
            io_wait();

            self.mask_all();
        }
    }

    pub fn mask_all(&self) {
        self.master.write_mask(0xFF);
        self.slave.write_mask(0xFF);
    }

    pub fn mask_irq(&self, irq: u8) {
        assert!(irq < 16);

        if irq < 8 {
            let mask = self.master.read_mask();
            self.master.write_mask(mask | (1u8 << irq));
        } else {
            let mask = self.slave.read_mask();
            self.slave.write_mask(mask | (1u8 << (irq - 8)));
        }
    }

    pub fn unmask_irq(&self, irq: u8) {
        assert!(irq < 16);

        if irq < 8 {
            let mask = self.master.read_mask();
            self.master.write_mask(mask & !(1u8 << irq));
        } else {
            let mask = self.slave.read_mask();
            self.slave.write_mask(mask & !(1u8 << (irq - 8)));
        }
    }

    pub fn send_eoi(&self, irq: u8) {
        assert!(irq < 16);

        if irq >= 8 {
            self.slave.send_command(EOI);
        }

        self.master.send_command(EOI);
    }
}

pub struct Pic {
    pub command_port: u16,
    pub data_port: u16,
    pub vector_offset: u8,
}

impl Pic {
    pub const fn new(cp: u16, dp: u16, vo: u8) -> Self {
        Self {
            command_port: cp,
            data_port: dp,
            vector_offset: vo,
        }
    }

    pub fn read_mask(&self) -> u8 {
        unsafe { read_u8(self.data_port) }
    }
    pub fn write_mask(&self, mask: u8) {
        self.send_data(mask);
    }

    pub fn send_command(&self, command: u8) {
        unsafe {
            write_u8(self.command_port, command);
        }
    }
    pub fn send_data(&self, data: u8) {
        unsafe {
            write_u8(self.data_port, data);
        }
    }
}
