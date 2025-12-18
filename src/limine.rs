//! limine boot protocol

use core::{ffi::{CStr, c_char}, fmt::Debug, mem::{MaybeUninit, transmute}, ptr, slice};

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

unsafe impl Sync for Framebuffer {}

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

#[repr(C, align(8))]
pub struct MemMapRequest {
    base: BaseRequest,
}

#[repr(C)]
pub struct MemMapResponse {
    revision: u64,
    entry_count: u64,
    entries: *const *const MemMapEntry,
}

#[repr(u64)]
#[derive(Debug)]
pub enum MemMapType {
    Usable = 0,
    Reserved = 1,
    AcpiReclaimable = 2,
    AcpiNvs = 3,
    BadMemory = 4,
    BootloaderReclaimable = 5,
    ExecutablesAndModules = 6,
    Framebuffer = 7,
    AcpiTables = 8,
}

#[repr(C)]
#[derive(Debug)]
pub struct MemMapEntry {
    pub base: u64,
    pub length: u64,
    pub r#type: MemMapType,
}

impl MemMapRequest {
    pub const fn new() -> Self {
        Self {
            base: BaseRequest::new([0x67cf3d9d378a806f, 0xe304acdfc50c3c62], 0),
        }
    }
    pub fn response(&self) -> Option<&'static MemMapResponse> {
        unsafe { (self.base.response as *const MemMapResponse).as_ref() }
    }
}

impl MemMapResponse {
    pub fn entries(&self) -> &'static [&'static MemMapEntry] {
        unsafe { slice::from_raw_parts(transmute(self.entries), self.entry_count as usize) }
    }
}

#[repr(C, align(8))]
pub struct RequestsStartMarker([u64; 4]);
impl RequestsStartMarker {
    pub const fn new() -> Self {
        Self([0xf6b8f4b39de7d1ae, 0xfab91a6940fcb9cf, 0x785c6ed015d3e316, 0x181e920a7852b9d9])
    }
}

#[repr(C, align(8))]
pub struct RequestsEndMarker([u64; 2]);
impl RequestsEndMarker {
    pub const fn new() -> Self {
        Self([0xadc0e0531bb10d03, 0x9572709f31764c62])
    }
}

#[repr(C, align(8))]
pub struct ExecutableAddressRequest {
    base: BaseRequest,
}

#[derive(Debug)]
pub struct ExecutableAddressResponse {
    revision: u64,
    physical_base: u64,
    virtual_base: u64,
}

impl ExecutableAddressRequest {
    pub const fn new() -> Self {
        Self {
            base: BaseRequest::new([0x71ba76863cc55f63, 0xb2644a48c516a487], 0),
        }
    }
    pub fn response(&self) -> Option<&'static ExecutableAddressResponse> {
        unsafe { (self.base.response as *const ExecutableAddressResponse).as_ref() }
    }
}

#[repr(C, align(8))]
pub struct HhdmRequest {
    base: BaseRequest,
}

#[derive(Debug)]
pub struct HhdmResponse {
    revision: u64,
    offset: u64,
}

impl HhdmRequest {
    pub const fn new() -> Self {
        Self {
            base: BaseRequest::new([0x48dcf1cb8ad2b852, 0x63984e959a98244b], 0),
        }
    }
    pub fn response(&self) -> Option<&'static HhdmResponse> {
        unsafe { (self.base.response as *const HhdmResponse).as_ref() }
    }
}
