use core::slice;

pub fn dump(addr: *const i8, len: usize) {
    let longs = unsafe { slice::from_raw_parts(addr, len * 8) };
    for ls in longs.chunks_exact(8) {
        for l in ls {
            print!("{:02x}", l);
        }
        println!();
    }
}

pub fn print_elfsections() {
    let mbi = crate::MBI.get().unwrap();
    let elf = mbi.elf_symbols().next().unwrap();
    for s in elf.sections() {
        let name =  s.get_name(elf).unwrap_or_default();
        let type_ = s.type_;
        let addr = s.addr;
        let size = s.size;
        let flags = s.flags;
        println!("{name:16.16} type: {type_:#02x}, addr: {addr:#x}, size: {size:#x}, flags: {flags:#x}");
    }
}

pub fn print_memoryareas() {
    let memory_map = crate::MBI.get().unwrap().memory_map().unwrap();
    for entry in &memory_map.entries {
        println!("    base: {:#14x}   size: {:#14x} (type {:#x})", entry.base_addr, entry.length, entry.type_)
    }
}

#[inline(always)]
pub fn print_registers() {
    let rip = x86_64::registers::read_rip();
    let (PML4T, _flags) = x86_64::registers::control::Cr3::read();
    let rflags = x86_64::registers::rflags::read();
    println!("PML4T at {:#x?}", PML4T);
    println!("RFLAGS: {:?}", rflags);
    println!("RIP: {:x?}", rip);
}

/**
 * Overflows the stack for testing purposes
 */
#[allow(dead_code)]
#[allow(unconditional_recursion)]
pub fn stack_overflow() {
    stack_overflow()
}

/**
 * Probably causes a page fault
 */
#[allow(dead_code)]
pub fn page_fault() {
    let ptr = 0xdeadc0de as *mut u8;
    unsafe { *ptr = 69; }
}

