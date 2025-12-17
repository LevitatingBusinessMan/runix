#![no_std]
#![no_main]
#![feature(ptr_metadata)]
#![feature(abi_x86_interrupt)]
#![feature(const_trait_impl)]
#![feature(ascii_char)]
#![feature(ptr_as_ref_unchecked)]
#![feature(cstr_display)]

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
    
    //vga::clear();

    // let mbi = BootInformation::load(mbi_pointer);

    // MBI.call_once(|| mbi);

    // conf::parse(mbi.boot_command_line().expect("Could not get cmdline").to_str().unwrap());
        
    // for tag in mbi.tags() {
    //     if let multiboot::Tag::Unknown(type_, data) = tag {
    //         wprintln!(" Unknown multiboot tag: type {} size: {:#x}", type_, data.len());
    //     }
    // }

    // if conf::CONFIG.get().unwrap().welcome {
    //     vga::print_at(vga::BUFFER_WIDTH/2 - WELCOME_STRING.len()/2, 12, WELCOME_STRING.as_bytes(), Color::Black, Color::White);
    // }

    // if conf::CONFIG.get().unwrap().print_info {
    //     let bootloader_name = mbi.bootloader_name().unwrap();
    //     println!("Booted from: {}", &bootloader_name.to_str().unwrap());

    //     let elf = mbi.elf_symbols().next().expect("No ELF symbols");
    //     println!(
    //         "Kernel at: {:#x?} - {:#x?}",
    //         elf.sections().into_iter().skip(1).map(|s| s.addr).min().unwrap(),
    //         elf.sections().into_iter().map(|s| s.addr + s.size).max().unwrap()
    //     );

    //     println!("Multiboot at: {:#7x?} - {:#7x?}", mbi_pointer, mbi_pointer as *const () as usize + mbi.total_size as usize);

    //     // let (PML4T, flags) = x86_64::registers::control::Cr3::read();
    //     // println!("PML4T at {:#x?}", PML4T);
    // }

    // allocator::init();
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
