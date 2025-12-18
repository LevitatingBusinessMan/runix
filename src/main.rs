#![no_std]
#![no_main]
#![feature(ptr_metadata)]
#![feature(abi_x86_interrupt)]
#![feature(const_trait_impl)]
#![feature(ascii_char)]
#![feature(ptr_as_ref_unchecked)]
#![feature(cstr_display)]
#![feature(exact_div)]

#[macro_use]
pub mod print;
mod panic;
// #[macro_use]
// mod vga;
#[deprecated]
mod multiboot;
// mod conf;
mod interrupts;
mod gdt;
mod kdebug;
mod debug;
// mod allocator;
mod pci;
mod fonts;
mod console;
mod limine;
mod gfx;
mod qemu;
mod allocator;

use core::ptr::addr_of;

pub use interrupts::keyboard;

#[used]
#[link_section = ".requests"]
pub static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
static BOOTLOADER_INFO_REQUEST: limine::BootloaderInfoRequest = limine::BootloaderInfoRequest::new();

#[used]
#[link_section = ".requests"]
static MEMMAP_REQUEST: limine::MemMapRequest = limine::MemMapRequest::new();

#[used]
#[link_section = ".requests"]
static HHDM_REQUEST: limine::HhdmRequest = limine::HhdmRequest::new();

#[used]
#[link_section = ".requests"]
static EXECUTABLE_ADDRESS_REQUEST: limine::ExecutableAddressRequest = limine::ExecutableAddressRequest::new();

#[used]
#[link_section = ".requests_start_marker"]
static LIMINE_REQUESTS_START_MARKER: limine::RequestsStartMarker = limine::RequestsStartMarker::new();

#[used]
#[link_section = ".requests_end_marker"]
static LIMINE_REQUESTS_END_MARKER: limine::RequestsEndMarker = limine::RequestsEndMarker::new();

extern "C" {
    /// address of the end of the kernel
    static kernel_end: u8;
}
// #[used]
// #[unsafe(link_section = ".requests_start_marker")]
// static _LIMINE_REQUESTS_START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

// #[used]
// #[unsafe(link_section = ".requests_end_marker")]
// static _LIMINE_REQUESTS_END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn runix() -> ! {
    print::init_console();
    //gdt::init_gdt();
    interrupts::init();

    let bootloader_info = BOOTLOADER_INFO_REQUEST.response().unwrap();
        
    println!("Welcome to Runix");
    println!("Booted via {} {}", bootloader_info.name().display(), bootloader_info.version().display());
    
    kdebug::kdebug();
    
}

#[macro_export]
macro_rules! hbreak {
    () => {
        x86_64::instructions::interrupts::int3();
    };
}

/// Wait a very small amount of time (1 to 4 microseconds, generally).
/// Useful for implementing a small delay for PIC remapping on old hardware or generally as a simple but imprecise wait.
#[macro_export]
macro_rules! io_wait {
    () => { x86_64::instructions::port::Port::new(0x80).write(0 as u8) };
}

#[macro_export]
macro_rules! hlt_loop {
    () => {
        loop {
            x86_64::instructions::hlt();
        }
    };
}
