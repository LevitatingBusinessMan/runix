//! This module is for handling interrupts

/* Why I decided to use x86-interrupt:
 * https://os.phil-opp.com/returning-from-exceptions/#a-naked-wrapper-function
 */
// https://github.com/rust-lang/rust/pull/39832

use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable, ExceptionVector, ExceptionVector::*, PageFaultErrorCode};
use x86_64::set_general_handler;
use spin::Once;
use crate::gdt;
pub mod pic8259;
pub mod keyboard;

/// Statically allocated IDT
// Make sure you have enough stack size for this
static IDT: Once<InterruptDescriptorTable> = Once::new();

use InterruptIndex::*;

pub fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();
    set_general_handler!(&mut idt, generic_interrupt_handler);
    set_general_handler!(&mut idt, generic_exception_handler, 0..0x20);
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    let double_fault_entry = idt.double_fault.set_handler_fn(double_fault_handler);
    unsafe {
        // register the double fault handler with a clean stack
        double_fault_entry.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt[Timer as usize].set_handler_fn(timer);
    idt[Keyboard as usize].set_handler_fn(keyboard::handler);
    IDT.call_once(|| idt);
    IDT.get().unwrap().load();
}

/// inits both the IDT and PIC
pub fn init() {
    init_idt();
    pic8259::init_pic();
    x86_64::instructions::interrupts::enable();
}

#[repr(u8)]
enum InterruptIndex {
    Timer = pic8259::PIC1_OFFSET,
    Keyboard
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

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    let addr = x86_64::registers::control::Cr2::read();
    panic!("PAGE FAULT {:#?} at {:#x}\n{:?}", error_code, addr, stack_frame);
}

extern "x86-interrupt" fn timer(_stack_frame: InterruptStackFrame) {
    pic8259::send_eoi(Timer as u8);
}

/// Get the exception form exception vector as an enum
fn exception_get_name(code: u8) -> Option<ExceptionVector> {
    match code {
        0..0x20 => Some(match code {
            x if x == Division as u8 => Division,
            x if x == Debug as u8 => Debug,
            x if x == NonMaskableInterrupt as u8 => NonMaskableInterrupt,
            x if x == Breakpoint as u8 => Breakpoint,
            x if x == Overflow as u8 => BoundRange,
            x if x == InvalidOpcode as u8 => InvalidOpcode,
            x if x == DeviceNotAvailable as u8 => DeviceNotAvailable,
            x if x == Double as u8 => Double,
            x if x == InvalidTss as u8 => InvalidTss,
            x if x == SegmentNotPresent as u8 => SegmentNotPresent,
            x if x == Stack as u8 => Stack,
            x if x == GeneralProtection as u8 => GeneralProtection,
            x if x == Page as u8 => Page,
            x if x == X87FloatingPoint as u8 => X87FloatingPoint,
            x if x == AlignmentCheck as u8 => AlignmentCheck,
            x if x == MachineCheck as u8 => MachineCheck,
            x if x == SimdFloatingPoint as u8 => SimdFloatingPoint,
            x if x == Virtualization as u8 => Virtualization,
            x if x == ControlProtection as u8 => ControlProtection,
            x if x == HypervisorInjection as u8 => HypervisorInjection,
            x if x == VmmCommunication as u8 => VmmCommunication,
            x if x == Security as u8 => Security,
            _ => unreachable!()
        }),
        _ => None
    }
}
