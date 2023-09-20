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

/// See man elf(5)
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
#[repr(C)]
pub struct ElfSymbol {
    pub num: u16,
    pub entsize: u16,
    pub shndx: u16,
    _reserved: u16,
    pub section_headers: [u8],
}

impl core::fmt::Debug for ElfSymbol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ElfSymbol")
        .field("num", &self.num)
        .field("entsize", &self.entsize)
        .field("shndx", &self.shndx)
        .finish()
    }
}

impl ElfSymbol {
    pub fn sections(&'static self) -> impl Iterator<Item = &'static ElfSection> {
        ElfSectionIter {symbol: &self, i: 0}
    }
}

pub struct ElfSectionIter {
    symbol: &'static ElfSymbol,
    i: usize,
}

// This is padded wrong or something (not at an 8th)
#[repr(packed)]
#[derive(Debug)]
/// See man elf(5)
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format#Section_header
pub struct ElfSection {
    /// An offset to a string in the .shstrtab section that represents the name of this section. 
    pub name: u32,
    /// Identifies the type of this header.
    pub type_: u32,
    /// Identifies the attributes of the section. 
    pub flags: usize,
    /// Virtual address of the section in memory, for sections that are loaded. 
    pub addr: usize,
    /// Offset of the section in the file image. 
    pub offset: usize,
    /// Size in bytes of the section in the file image. May be 0. 
    pub size: usize,
    /// Contains the section index of an associated section. This field is used for several purposes, depending on the type of section. 
    pub link: u32,
    /// Contains extra information about the section. This field is used for several purposes, depending on the type of section. 
    pub info: u32,
    /// Contains the required alignment of the section. This field must be a power of two. 
    pub addr_align: u64,
    /// Contains the size, in bytes, of each entry, for sections that contain fixed-size entries. Otherwise, this field contains zero. 
    pub entsize: usize,
}

impl Iterator for ElfSectionIter {
    type Item = &'static ElfSection;

    fn next(&mut self) -> Option<Self::Item> {
        // extra 4 because of weird padding
        // I think I can use shndx like this to correctly get to the first section?
        let offset = self.symbol.shndx as usize + 4;

        if self.i + 1 >= self.symbol.num as usize {
            return None;
        }
        
        let addr = addr_of!(self.symbol.section_headers) as *const () as usize + offset + self.i * core::mem::size_of::<ElfSection>() ;
        self.i += 1;
        unsafe {
            return Some(&*ptr::from_raw_parts(addr as *const (), ()));
        }
    }
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
    BasicMemInfo(&'static BasicMemInfo) = 4,
    BIOSBootDevice(&'static BIOSBootDevice) = 5,
    MemoryMap(&'static MemoryMap) = 6,
    FrameBufferInfo(&'static FrameBufferInfo) = 8,
    ElfSymbol(&'static ElfSymbol) = 9,
    APMTable(&'static APMTable) = 10,
    ACPIOldRSDP(&'static [u8]) = 14,
    ACPINewRSDP(&'static [u8]) = 15,
    NetworkInfo(&'static [u8]) = 16,
    ImageLoadBase(&'static ImageLoadBase) = 21,
    /// Any tag not explicitly added here. With type
    Unknown(u32, &'static [u8]),
}

#[repr(C)]
pub struct FrameBufferInfo {
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8,
    _reserved: u8,
    /// This still has to be expanded
    color_info: [u8],
}

#[repr(C)]
pub struct BasicMemInfo {
    pub mem_lower: u32,
    pub mem_upper: u32,
}

#[repr(C)]
pub struct BIOSBootDevice {
    pub biosdev: u32,
    pub partition: u32,
    pub sub_partition: u32,
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
            4 => {
                Some(Tag::BasicMemInfo(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), ())
                }))
            },
            5 => {
                Some(Tag::BIOSBootDevice(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), ())
                }))
            },
            6 => {
                let entries = (size - 8) / size_of::<MemoryMapEntry>();
                Some(Tag::MemoryMap(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), entries)
                }))
            },
            8 => {
                Some(Tag::FrameBufferInfo(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), size)
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
            14 => {
                Some(Tag::ACPIOldRSDP(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), size)
                }))
            },
            15 => {
                Some(Tag::ACPINewRSDP(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), size)
                }))
            },
            21 => {
                Some(Tag::ImageLoadBase(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), ())
                }))
            },
            _ => {
                // panic!(
                //     "Unknown tag with type {} and size {:#x} at {:#x}",
                //     type_,
                //     size,
                //     addr_of!(self.mbi.tags) as *const () as usize + self.i
                // );
                Some(Tag::Unknown(type_, unsafe {
                    &*ptr::from_raw_parts(addr as *const (), size)
                }))
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

    pub fn bootloader_name(&'static self) -> Option<&'static CStr> {
        self.tags().find_map(|t| if let Tag::BootLoaderName(bln) = t {Some(bln)} else {None})
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
