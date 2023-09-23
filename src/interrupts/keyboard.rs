//! Handles they keyboard interrupt

use x86_64::instructions::port::PortReadOnly;
use x86_64::structures::idt::InterruptStackFrame;
use spin::RwLock;

use super::pic8259;
use super::Keyboard;

const PS2: PortReadOnly<u8> = PortReadOnly::new(0x60);

struct KeyBuffer {
    buffer: [char; 256],
    len: usize,
}

static KEYBUFFER: RwLock<KeyBuffer> = RwLock::new(KeyBuffer {buffer: ['\0'; 256], len: 0});

// I should not be able to receive keyboard interrupts during
// the handling of a keyboard interrupt. So a deadlock should not occur.
#[allow(const_item_mutation)]
pub(super) extern "x86-interrupt" fn handler(_stack_frame: InterruptStackFrame) {
    let scancode = unsafe {PS2.read()};
    let c = translate_key(scancode);
    print!("{}", c);
    let mut kb = KEYBUFFER.write();
    let len = kb.len;
    kb.buffer[len] = c;
    kb.len += 1;
    pic8259::send_eoi(Keyboard as u8);
}

pub fn translate_key(scancode: u8) -> char {
    // todo
    return scancode as char;
}
