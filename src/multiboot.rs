//! Structures for the multiboot information

use core::ptr;
use core::ptr::addr_of;
use core::ffi::CStr;
use core::mem::discriminant;

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
    BootCommandLine(&'static BootCommandLine) = 1,
    BootLoaderName = 2,
    ElfSymbol(&'static ElfSymbol) = 9,
    ImageLoadBase(&'static ImageLoadBase) = 21,
}

#[repr(C)]
pub struct BootCommandLine {
    /// `string` contains command line. terminated UTF-8 string. The command line is a normal C-style zero-
    pub string: CStr
}

impl core::fmt::Display for BootCommandLine {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.string.to_str().expect("Failed to convert C string"))
    }
}

impl core::fmt::Debug for BootCommandLine {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", &self.string)
    }
}

pub struct TagIter {
    mbi: &'static MultibootInformation,
    i: usize,
}

impl Iterator for TagIter {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.mbi.total_size as usize - 8 {
            return None
        }

        // Align to an 8th byte
        if self.i != 0 {
            self.i += 8 - (self.i % 8);
        }

        let type_ = u32::from_le_bytes(self.mbi.tags[self.i..self.i+4].try_into().unwrap());
        let size = u32::from_le_bytes(self.mbi.tags[self.i+4..self.i+8].try_into().unwrap()) as usize - 8;

        if type_ == 0 {
            return None
        }

        let addr = addr_of!(self.mbi.tags) as *const () as usize + (self.i + 8);
        self.i += size + 8;

        //println!("{}", Tag::BootCommandLine as u32);

        return match type_ {
            1 => {
                Some(Tag::BootCommandLine(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), size)
                }))
            },
            9 => {
                Some(Tag::ElfSymbol(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), size)
                }))
            },
            21 => {
                Some(Tag::ImageLoadBase(unsafe {
                    &*ptr::from_raw_parts(addr as *const (), ())
                }))
            },
            _ => panic!("Unknown type {type_} of size {size:#x} found in mbi")
        }
    }
}

impl MultibootInformation {
    pub fn load(ptr: *const MultibootInformation) -> &'static Self {
        let total_size = unsafe {*(ptr as *const u32)};
        let mbi: &MultibootInformation = unsafe {&*ptr::from_raw_parts(ptr as *const (), total_size as usize)};
        mbi
    }

    // Iterate over the tags
    pub fn tags(&'static self) -> TagIter {
        TagIter { mbi: &self, i: 0 }
    }

    pub fn boot_command_line(&'static self) -> Option<&'static BootCommandLine> {
        self.tags().find(|t| matches!(t, Tag::BootCommandLine(_))).map(|t| {
            if let Tag::BootCommandLine(bcl) = t {
                return bcl
            }
            unreachable!()
        })
    }

}
