use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::structures::tss::TaskStateSegment;

use spin::Lazy;

const DOUBLE_FAULT_IST_INDEX: usize = 0;
//static TSS: TaskStateSegment = TaskStateSegment::new();
static TSS: Lazy<TaskStateSegment> = Lazy(|| {
    let mut tss = TaskStateSegment::new();
    //tss[DOUBLE_FAULT_IST_INDEX] = 
    tss
});
