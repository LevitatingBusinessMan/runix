//! This module is for handling interrupts

/* Why I decided to use x86-interrupt:
 * https://os.phil-opp.com/returning-from-exceptions/#a-naked-wrapper-function
 */
// https://github.com/rust-lang/rust/pull/39832

use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};
use x86_64::set_general_handler;
use spin::Once;
use crate::gdt;

/// Statically allocated IDT
// Make sure you have enough stack size for this
static IDT: Once<InterruptDescriptorTable> = Once::new();

pub fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();
    set_general_handler!(&mut idt, generic_handler);
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    let double_fault_entry = idt.double_fault.set_handler_fn(double_fault_handler);
    unsafe {
        // register the double fault handler with a clean stack
        double_fault_entry.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    IDT.call_once(|| idt);
    IDT.get().unwrap().load();
}

fn generic_handler(stack_frame: InterruptStackFrame, index: u8, err_code : Option<u64>) {
    panic!("Unimplemented interrupt {:#x} (err: {:x?})\n{:?}", index, err_code, stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, err_code : u64) -> ! {
    panic!("DOUBLE FAULT {:#x} \n{:?}", err_code, stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    exprintln!("HARDWARE BREAKPOINT\n{:?}", stack_frame);
}
