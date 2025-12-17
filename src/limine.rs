//! limine boot protocol

use core::{ffi::{CStr, c_char}, mem::{MaybeUninit, transmute}, ptr, slice};

use x86_64::registers::segmentation::CS;
#[repr(C, align(8))]
pub struct BaseRevision {
    _magic: [u64; 2],
    revision: u64
}

impl BaseRevision {
    /// the revision Runix uses
    pub const REVISION: u64 = 4;

    pub const fn new() -> Self {
        Self {
            _magic: [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc],
            revision: Self::REVISION,
        }
    }
}

// force that raw pointer to be Sync
unsafe impl Sync for BaseRequest {}

#[repr(C, align(8))]
struct BaseRequest {
    id: [u64; 4],
    revision: u64,
    response: *const (),
}
impl BaseRequest {
    pub const COMMON_MAGIC: [u64; 2] = [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b];
}

impl BaseRequest {
    pub const fn new(id: [u64; 2], revision: u64) -> BaseRequest {
        Self {
            id: [Self::COMMON_MAGIC[0], Self::COMMON_MAGIC[1], id[0], id[1]],
            revision,
            response: ptr::null(),
        }
    }
}

#[repr(C, align(8))]
pub struct BootloaderInfoRequest {
    base: BaseRequest,
}

#[repr(C, align(8))]
pub struct BootloaderInfoReponse {
    revision: u64,
    name: *const c_char,
    version: *const c_char,
}

impl BootloaderInfoRequest {
    pub const fn new() -> Self {
        Self {
            base: BaseRequest::new([0xf55038d8e2a1202f, 0x279426fcf5f59740], 0),
        }
    }
    pub fn response(&self) -> Option<&'static BootloaderInfoReponse> {
        unsafe { (self.base.response as *const BootloaderInfoReponse).as_ref() }
    }
}

impl BootloaderInfoReponse {
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.name) }
    }
    pub fn version(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.version) }
    }
}

#[repr(C, align(8))]
pub struct FramebufferRequest {
    base: BaseRequest,
}

#[repr(C)]
pub struct FramebufferResponse {
    revision: u64,
    framebuffer_count: u64,
    framebuffers: *const *const Framebuffer,
}

impl FramebufferResponse {
    pub fn framebuffers(&self) -> &[&'static Framebuffer] {
        unsafe {
            slice::from_raw_parts(transmute(self.framebuffers), self.framebuffer_count as usize)
        }
    }
}

impl FramebufferRequest {
    pub const fn new() -> Self {
        Self {
            base: BaseRequest::new([0x9d5827dcd881dd75, 0xa3148604f6fab11b], 1),
        }
    }
    pub fn response(&self) -> Option<&'static FramebufferResponse> {
        unsafe { (self.base.response as *const FramebufferResponse).as_ref() }
    }
}

#[repr(C)]
pub struct Framebuffer {
    pub address: *mut u8,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
    pub unused: [u8; 7],
    pub edid_size: u64,
    pub edid: *mut u8,

    /* Response revision 1 */
    pub mode_count: MaybeUninit<u64>,
    pub modes: MaybeUninit<*const [VideoMode]>,
}

impl Framebuffer {
    /// get internal buffer as slice
    pub fn buffer(&self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.address, (self.height * self.pitch) as usize) }
    }
}

#[repr(C)]
pub struct VideoMode {
    pub pitch: u64,
    pub width: u64,
    pub height: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
}
