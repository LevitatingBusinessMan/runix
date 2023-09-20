//! Structures for the multiboot information

use core::ptr::{self, addr_of};

use crate::println;

#[repr(C)]
pub struct MultibootInformation {
    /// `total_size` contains the total size of boot information including this field and terminating tag in bytes.
    pub total_size: u32,
    /// Will be 0, and should be ignored
    _reserved: u32,
    /// Our tags
    pub tags: [u8],
}

#[repr(C)]
pub struct ElfSymbol {
    pub num: u16,
    pub entsize: u16,
    pub shndx: u16,
    pub reserved: u16,
    pub section_headers: [u8],
} 

impl MultibootInformation {
    pub fn load(ptr: *const MultibootInformation) -> &'static Self {
        let total_size = unsafe {*(ptr as *const u32)};
        let mbi: &MultibootInformation = unsafe {&*ptr::from_raw_parts(ptr as *const (), total_size as usize)};
        mbi
    }

    pub fn elf_symbols(&self) -> Result<&'static ElfSymbol, ()> {
        let mut i: usize = 0;
        loop {
            if i >= self.total_size as usize - 8 {
                return Err(())
            }
            let type_ = u32::from_le_bytes(self.tags[i..i+4].try_into().unwrap());
            println!("{}", type_);
            let size = u32::from_le_bytes(self.tags[i+4..i+8].try_into().unwrap()) as usize;
            println!("{}", size);

            if type_ == 9 {
                let tag: &ElfSymbol = unsafe {&*ptr::from_raw_parts((addr_of!(self.tags) as *const ()).offset(i.try_into().unwrap()), size)};
                return Ok(tag);
            }

            i += size + 8;
        }
    }

}
