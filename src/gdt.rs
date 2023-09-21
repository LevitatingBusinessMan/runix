use x86_64::registers::segmentation::Segment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

use spin::Once;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
static TSS: Once<TaskStateSegment> = Once::new();

fn init_tss() {
    let mut tss = TaskStateSegment::new();
    const DOUBLE_FAULT_STACK_SIZE: usize = 4096 * 16;
    static mut DOUBLE_FAULT_STACK: [u8; DOUBLE_FAULT_STACK_SIZE] = [0; DOUBLE_FAULT_STACK_SIZE];    
    let stack_start = VirtAddr::from_ptr(unsafe {&DOUBLE_FAULT_STACK});
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] =  stack_start + DOUBLE_FAULT_STACK_SIZE;
    TSS.call_once(|| tss);
}

static GDT: Once<GlobalDescriptorTable> = Once::new();

pub fn init_gdt() {
    if !TSS.is_completed() { init_tss() }
    let mut gdt = GlobalDescriptorTable::new();
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(TSS.get().expect("TSS not yet initialized")));
    GDT.call_once(|| gdt);
    GDT.get().unwrap().load();

    // Enable our new GDT
    unsafe {
        use x86_64::instructions::segmentation::CS;
        use x86_64::instructions::tables::load_tss;
        CS::set_reg(code_selector);
        load_tss(tss_selector);
    }
}

