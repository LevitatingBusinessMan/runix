//! This module is for handling interrupts

/* Why I decided to use x86-interrupt:
 * https://os.phil-opp.com/returning-from-exceptions/#a-naked-wrapper-function
 */
// https://github.com/rust-lang/rust/pull/39832

use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable, ExceptionVector};
use x86_64::set_general_handler;
use spin::Once;
use crate::gdt;
pub mod pic8259;

/// Statically allocated IDT
// Make sure you have enough stack size for this
static IDT: Once<InterruptDescriptorTable> = Once::new();

pub fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();
    set_general_handler!(&mut idt, generic_interrupt_handler);
    set_general_handler!(&mut idt, generic_exception_handler, 0..0x20);
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    let double_fault_entry = idt.double_fault.set_handler_fn(double_fault_handler);
    unsafe {
        // register the double fault handler with a clean stack
        double_fault_entry.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    IDT.call_once(|| idt);
    IDT.get().unwrap().load();
}

/// inits both the IDT and PIC
pub fn init() {
    init_idt();
    pic8259::init_pic();
    crate::io_wait!();
    x86_64::instructions::interrupts::enable();
}

// There is an enum with exception numbers:
// https://docs.rs/x86_64/latest/src/x86_64/structures/idt.rs.html#1137-1206

fn generic_exception_handler(stack_frame: InterruptStackFrame, index: u8, err_code : Option<u64>) {
    wprintln!("Unimplemented exception {:#x} (ex: {:?}) (err: {:x?})\n{:?}", index, exception_get_name(index).unwrap(), err_code, stack_frame);
}

fn generic_interrupt_handler(_stack_frame: InterruptStackFrame, index: u8, _err_code : Option<u64>) {
    pic8259::send_eoi(index);
    wprintln!("Unimplemented interrupt {:#x}", index);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, err_code : u64) -> ! {
    panic!("DOUBLE FAULT {:#x} \n{:?}", err_code, stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    exprintln!("HARDWARE BREAKPOINT\n{:?}", stack_frame);
}

/// Get the exception form exception vector as an enum
fn exception_get_name(code: u8) -> Option<ExceptionVector> {
    match code {
        0..0x20 => Some(match code {
            x if x == ExceptionVector::Division as u8 => ExceptionVector::Division,
            x if x == ExceptionVector::Debug as u8 => ExceptionVector::Debug,
            x if x == ExceptionVector::NonMaskableInterrupt as u8 => ExceptionVector::NonMaskableInterrupt,
            x if x == ExceptionVector::Breakpoint as u8 => ExceptionVector::Breakpoint,
            x if x == ExceptionVector::Overflow as u8 => ExceptionVector::BoundRange,
            x if x == ExceptionVector::InvalidOpcode as u8 => ExceptionVector::InvalidOpcode,
            x if x == ExceptionVector::DeviceNotAvailable as u8 => ExceptionVector::DeviceNotAvailable,
            x if x == ExceptionVector::Double as u8 => ExceptionVector::Double,
            x if x == ExceptionVector::InvalidTss as u8 => ExceptionVector::InvalidTss,
            x if x == ExceptionVector::SegmentNotPresent as u8 => ExceptionVector::SegmentNotPresent,
            x if x == ExceptionVector::Stack as u8 => ExceptionVector::Stack,
            x if x == ExceptionVector::GeneralProtection as u8 => ExceptionVector::GeneralProtection,
            x if x == ExceptionVector::Page as u8 => ExceptionVector::Page,
            x if x == ExceptionVector::X87FloatingPoint as u8 => ExceptionVector::X87FloatingPoint,
            x if x == ExceptionVector::AlignmentCheck as u8 => ExceptionVector::AlignmentCheck,
            x if x == ExceptionVector::MachineCheck as u8 => ExceptionVector::MachineCheck,
            x if x == ExceptionVector::SimdFloatingPoint as u8 => ExceptionVector::SimdFloatingPoint,
            x if x == ExceptionVector::Virtualization as u8 => ExceptionVector::Virtualization,
            x if x == ExceptionVector::ControlProtection as u8 => ExceptionVector::ControlProtection,
            x if x == ExceptionVector::HypervisorInjection as u8 => ExceptionVector::HypervisorInjection,
            x if x == ExceptionVector::VmmCommunication as u8 => ExceptionVector::VmmCommunication,
            x if x == ExceptionVector::Security as u8 => ExceptionVector::Security,
            _ => unreachable!()
        }),
        _ => None
    }
}
