//! Structures for the multiboot information

use core::ptr;
use core::ptr::addr_of;
use core::ffi::CStr;
use core::mem::{discriminant, size_of};

use crate::println;

#[repr(C)]
pub struct BootInformation {
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

#[repr(C)]
/// Image load base physical address.
/// This tag contains image load base physical address. It is provided only if image has
/// a relocatable header tag.
pub struct ImageLoadBase {
    pub load_base_addr: u32,
}

#[repr(u32)]
pub enum Tag {
    End = 0,
    BootCommandLine(&'static CStr) = 1,
    BootLoaderName(&'static CStr) = 2,
    MemoryMap(&'static MemoryMap) = 6,
    ElfSymbol(&'static ElfSymbol) = 9,
    APMTable(&'static APMTable) = 10,
    ImageLoadBase(&'static ImageLoadBase) = 21,
    //Unknown(&'static [u8]),
}

#[repr(C)]
pub struct MemoryMap {
    pub entry_size: u32,
    pub entry_version: u32,
    pub entries: [MemoryMapEntry],
}

#[repr(C)]
pub struct MemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub type_: u32,
    _reserved: u32,
}

pub struct APMTable {
    pub version: u16,
    pub cseg: u16,
    pub offset: u32,
    pub cseg_16: u16,
    pub dseg: u16,
    pub flags: u16,
    pub cseg_len: u16,
    pub cseg_16_len: u16,
    pub dseg_len: u16
}

pub struct TagIter {
    mbi: &'static BootInformation,
    i: usize,
}

impl Iterator for TagIter {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.mbi.total_size as usize - 8 {
            panic!("Have not found end tag")
        }

        // Align to an 8th byte
        if self.i % 8 != 0  {
            self.i += 8 - (self.i % 8);
        }

        let type_ = u32::from_le_bytes(self.mbi.tags[self.i..self.i+4].try_into().unwrap());
        let size = u32::from_le_bytes(self.mbi.tags[self.i+4..self.i+8].try_into().unwrap()) as usize - 8;

        if type_ == 0 {
            return None
        }

        let addr = addr_of!(self.mbi.tags) as *const () as usize + (self.i + 8);
        self.i += size + 8;

        /*
         * break src/multiboot.rs:102
         * x/2wx addr-8
         * x/2wx addr+size
         */

        return match type_ {
            1 => {
                Some(Tag::BootCommandLine(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), size)
                }))
            },
            2 => {
                Some(Tag::BootLoaderName(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), size)
                }))
            },
            6 => {
                let entries = (size - 8) / size_of::<MemoryMapEntry>();
                Some(Tag::MemoryMap(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), entries)
                }))
            },
            9 => {
                Some(Tag::ElfSymbol(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), size - 8)
                }))
            },
            10 => {
                Some(Tag::APMTable(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), ())
                }))
            },
            21 => {
                Some(Tag::ImageLoadBase(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), ())
                }))
            },
            _ => {
                panic!(
                    "Unknown tag with type {} and size {:#x} at {:#x}",
                    type_,
                    size,
                    addr_of!(self.mbi.tags) as *const () as usize + self.i
                );
                // Some(Tag::Unknown(unsafe {
                //     &*ptr::from_raw_parts(addr as *const (), size)
                // }))
            }
        }
    }
}

impl BootInformation {
    pub fn load(ptr: *const BootInformation) -> &'static Self {
        let total_size = unsafe {*(ptr as *const u32)};
        let mbi: &BootInformation = unsafe {&*ptr::from_raw_parts(ptr as *const (), total_size as usize)};
        mbi
    }

    /// Iterate over the tags
    pub fn tags(&'static self) -> TagIter {
        TagIter { mbi: &self, i: 0 }
    }

    pub fn boot_command_line(&'static self) -> Option<&'static CStr> {
        self.tags().find_map(|t| if let Tag::BootCommandLine(bcl) = t {Some(bcl)} else {None})
    }

    pub fn memory_map(&'static self) -> Option<&'static MemoryMap> {
        self.tags().find_map(|t| if let Tag::MemoryMap(mm) = t {Some(mm)} else {None})
    }

    pub fn elf_symbols(&'static self) -> impl Iterator<Item = &'static ElfSymbol> {
        self.tags().filter_map(|t| if let Tag::ElfSymbol(es) = t {Some(es)} else {None})
    }
}
