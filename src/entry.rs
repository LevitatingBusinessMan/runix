#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(ptr_metadata)]
#![feature(abi_x86_interrupt)]
#![feature(const_trait_impl)]
#![feature(ascii_char)]

mod panic;
#[macro_use]
mod vga;
mod multiboot;
mod conf;
mod interrupts;
mod gdt;
mod kdebug;
mod debug;
mod allocator;

static WELCOME_STRING :&'static str = "Welcome to Runix!";

use interrupts::keyboard;
use vga::Color;
use multiboot::BootInformation;
use spin::Once;

pub static MBI: Once<&'static BootInformation> = Once::new();

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

#[no_mangle]
// https://en.wikipedia.org/wiki/VGA_text_mode
#[allow(improper_ctypes_definitions)]
pub extern fn runix(mbi_pointer: *const BootInformation) -> ! {
    gdt::init_gdt();
    interrupts::init();
    vga::clear();

    let mbi = BootInformation::load(mbi_pointer);

    MBI.call_once(|| mbi);

    conf::parse(mbi.boot_command_line().expect("Could not get cmdline").to_str().unwrap());

    for tag in mbi.tags() {
        if let multiboot::Tag::Unknown(type_, data) = tag {
            wprintln!(" Unknown multiboot tag: type {} size: {:#x}", type_, data.len());
        }
    }

    if conf::CONFIG.get().unwrap().welcome {
        vga::print_at(vga::BUFFER_WIDTH/2 - WELCOME_STRING.len()/2, 12, WELCOME_STRING.as_bytes(), Color::Black, Color::White);
    }

    if conf::CONFIG.get().unwrap().print_info {
        let bootloader_name = mbi.bootloader_name().unwrap();    
        println!("Booted from: {}", &bootloader_name.to_str().unwrap());
      
        let elf = mbi.elf_symbols().next().expect("No ELF symbols");
        println!(
            "Kernel at: {:#x?} - {:#x?}",
            elf.sections().into_iter().skip(1).map(|s| s.addr).min().unwrap(),
            elf.sections().into_iter().map(|s| s.addr + s.size).max().unwrap()
        );
    
        println!("Multiboot at: {:#7x?} - {:#7x?}", mbi_pointer, mbi_pointer as *const () as usize + mbi.total_size as usize);  

        // let (PML4T, flags) = x86_64::registers::control::Cr3::read();
        // println!("PML4T at {:#x?}", PML4T);
    }

    allocator::init();

    kdebug::kdebug();

}
