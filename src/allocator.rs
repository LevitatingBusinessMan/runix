use core::ptr::addr_of;

use x86_64::VirtAddr;

pub fn init() {
    let mbi = crate::MBI.get().expect("Allocator could net get MBI");
    let mbi_end = unsafe { (addr_of!(**mbi) as *const u8).offset(mbi.total_size as isize) };
    let first_page = unsafe { mbi_end.offset(mbi_end.align_offset(0x1000) as isize) };
    //let first_page = VirtAddr::new(first_page as u64);
    println!("First page: {first_page:#x?}");
}
