//! Structures for the multiboot information

use core::ptr;
use core::ptr::addr_of;
use core::ffi::CStr;

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

#[repr(u32)]
pub enum TagType {
    BootCommandLine = 1,
    BootLoaderName = 2,
    ElfSymbol = 9,
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

impl MultibootInformation {
    pub fn load(ptr: *const MultibootInformation) -> &'static Self {
        let total_size = unsafe {*(ptr as *const u32)};
        let mbi: &MultibootInformation = unsafe {&*ptr::from_raw_parts(ptr as *const (), total_size as usize)};
        mbi
    }

    // Iterate over the tags
    //pub fn tags(&self) ->

    pub fn boot_command_line(&self) -> Result<&'static BootCommandLine, ()> {
        let mut i: usize = 0;
        loop {
            if i >= self.total_size as usize - 8 {
                return Err(())
            }

            // Align to an 8th byte
            if i != 0 {
                i += 8 - (i % 8);
            }

            let type_ = u32::from_le_bytes(self.tags[i..i+4].try_into().unwrap());
            let size = u32::from_le_bytes(self.tags[i+4..i+8].try_into().unwrap()) as usize;
           
            if type_ == 1 {
                let addr = addr_of!(self.tags) as *const () as usize + (i + 8);
                let tag: &BootCommandLine = unsafe {
                    &*ptr::from_raw_parts(addr as *const (), size-8)
                };
                return Ok(tag);
            }

            i += size;
        }
    }

}
