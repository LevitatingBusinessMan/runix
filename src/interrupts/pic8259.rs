//! Programmable Interrupt Controller
//! https://wiki.osdev.org/PIC
//! https://en.wikipedia.org/wiki/Intel_8259
//! These chips have 8 in and 8 out lines

// Also see https://docs.rs/pic8259/latest/src/pic8259/lib.rs.html#1-186

/// Interrupt index for master PIC
pub(super) const PIC1_OFFSET: u8 = 0x20;
/// Interrupt index for slave PIC
pub(super) const PIC2_OFFSET: u8 = 0x28; 

use x86_64::instructions::port::{Port, PortWriteOnly};
use crate::io_wait;

struct PIC {
    command: PortWriteOnly<u8>,
    data: Port<u8>,
}

static mut PIC1: PIC = PIC {command: PortWriteOnly::new(0x20), data: Port::new(0x21)};
static mut PIC2: PIC = PIC {command: PortWriteOnly::new(0xa0), data: Port::new(0xa1)};

pub fn send_eoi(irq: u8) {
    if irq >= 8 {
        unsafe {PIC2.command.write(0x20)};
    }
    unsafe {PIC1.command.write(0x20)};
}

/// Reinitiliaze the PIC to use an offset above 0x20
pub fn init_pic() {

    // Older moderboards might require some processing timme between the writes to the PICS
    unsafe {
        // Save masks
        let pic1_mask = PIC1.data.read();
        let pic2_mask = PIC2.data.read();

        // Tell the PICs to initialize
        PIC1.command.write(0x11);
        io_wait!();
        PIC2.command.write(0x11);
        io_wait!();

        // Tell the PICs what offsets to use
        PIC1.data.write(PIC1_OFFSET);
        io_wait!();
        PIC2.data.write(PIC2_OFFSET);
        io_wait!();

        // Tell master to use line 4
        PIC1.data.write(4);
        io_wait!();
        // Tell slave to use line 2
        PIC2.data.write(2);
        io_wait!();

        // Configure the PICS to use 8086 mode
        PIC1.data.write(1);
        io_wait!();
        PIC2.data.write(1);
        io_wait!();

        // Restore masks
        PIC1.data.write(pic1_mask);
        PIC2.data.write(pic2_mask);
    }
}
