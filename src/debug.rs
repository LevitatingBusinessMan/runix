//! debug helpers and printers
use core::slice;

use crate::limine::MemMapType;

pub mod byte_unit {
    //! https://physics.nist.gov/cuu/Units/binary.html

    pub enum Unit {
        Byte,
        KiB,
        MiB,
        GiB,
    }

    pub struct UnitValue {
        pub value: u128,
        pub unit: Unit,
    }
    
    impl Display for UnitValue {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let unit = match self.unit {
                Unit::Byte => "B",
                Unit::KiB => "KiB",
                Unit::MiB => "MiB",
                Unit::GiB => "GiB",
            };
            // WARNING this allocates
            f.pad(&alloc::format!("{}{}", self.value, unit))
        }
    }
    
    use core::fmt::{Debug, Display};

    use alloc::format;
    
    /// how many digits as base 10
    fn digit_count(n: u128) -> usize {
        if n == 0 { return 1; }
        (n.ilog10() + 1) as usize
    }
    
    pub fn to_unit(b: u128) -> UnitValue {
        if let Some(gib) = b.div_exact(2u128.pow(30)) {
            UnitValue { value: gib,  unit: Unit::GiB }
        } else if let Some(mib) = b.div_exact(2u128.pow(20)) {
            UnitValue { value: mib,  unit: Unit::MiB }
        } else if let Some(kib) = b.div_exact(2u128.pow(10)) {
            UnitValue { value: kib,  unit: Unit::KiB }
        } else {
            UnitValue { value: b,  unit: Unit::Byte }
        }
    }
}


pub fn dump(addr: *const i8, len: usize) {
    let longs = unsafe { slice::from_raw_parts(addr, len * 8) };
    for ls in longs.chunks_exact(8) {
        for l in ls {
            print!("{:02x}", l);
        }
        println!();
    }
}

// pub fn print_elfsections() {
//     let mbi = crate::MBI.get().unwrap();
//     let elf = mbi.elf_symbols().next().unwrap();
//     for s in elf.sections() {
//         let name =  s.get_name(elf).unwrap_or_default();
//         let type_ = s.type_;
//         let addr = s.addr;
//         let size = s.size;
//         let flags = s.flags;
//         println!("{name:16.16} type: {type_:#02x}, addr: {addr:#x}, size: {size:#x}, flags: {flags:#x}");
//     }
// }

// pub fn print_memoryareas() {
//     let memory_map = crate::MBI.get().unwrap().memory_map().unwrap();
//     for entry in &memory_map.entries {
//         println!("    base: {:#14x}   size: {:#14x} (type {:#x})", entry.base_addr, entry.length, entry.type_)
//     }
// }

pub fn print_limine_memory_map() {
    println!("{:>16} {:>16} {:<12} {:<8} {}", "START", "END", "SIZE", "AS UNIT", "TYPE"); 
    for entry in crate::MEMMAP_REQUEST.response().unwrap().entries() {
        println!("{:>16x?} {:>16x?} {:<12x?} {:<8} {:?}", 
            entry.base, 
            (entry.base + entry.length), 
            entry.length,
            byte_unit::to_unit(entry.length as u128),
            entry.r#type
        );
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
