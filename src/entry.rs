#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(panic_info_message)]
#![feature(ptr_metadata)]
#![feature(const_maybe_uninit_zeroed)]
#![feature(abi_x86_interrupt)]
#![feature(ptr_from_ref)]

mod panic;
#[macro_use]
mod vga;
mod multiboot;
mod conf;
mod interrupts;
mod gdt;

static WELCOME_STRING :&'static str = "Welcome to Runix!";

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

#[no_mangle]
// https://en.wikipedia.org/wiki/VGA_text_mode
#[allow(improper_ctypes_definitions)]
pub extern fn runix(mbi_pointer: *const BootInformation) -> ! {
    gdt::init_gdt();
    interrupts::init_idt();
    vga::clear();

    //stack_overflow();

    let mbi = BootInformation::load(mbi_pointer);

    //MBI.call_once(|| mbi);

    conf::parse(mbi.boot_command_line().expect("Could not get cmdline").to_str().unwrap());

    for tag in mbi.tags() {
        if let multiboot::Tag::Unknown(type_, data) = tag {
            wprintln!(" Unknown multiboot tag: type {} size: {:#x}", type_, data.len());
        }
    }

    if conf::CONFIG.read().welcome {
        vga::print_at(vga::BUFFER_WIDTH/2 - WELCOME_STRING.len()/2, 12, WELCOME_STRING.as_bytes(), Color::Black, Color::White);
    }

    if conf::CONFIG.read().print_info {
        println!("Multiboot at: {:#7x?} - {:#7x?}", mbi_pointer, mbi_pointer as *const () as usize + mbi.total_size as usize);
        let elf = mbi.elf_symbols().next().expect("No ELF symbols");
    
        println!(
            "Kernel at: {:#x?} - {:#x?}",
            elf.sections().map(|s| s.addr).min().unwrap(),
            elf.sections().map(|s| s.addr + s.size).max().unwrap()
        );
    
        let bootloader_name = mbi.bootloader_name().unwrap();    
        println!("Booted from: {}", &bootloader_name.to_str().unwrap());
    
        println!("Memory areas: ");
        let memory_map = mbi.memory_map().unwrap();
        for entry in &memory_map.entries {
            println!("    base: {:#14x}   size: {:#14x} (type {:#x})", entry.base_addr, entry.length, entry.type_)
        }
    }

    //stack_overflow();

    loop{}
}

fn stack_overflow() {
    stack_overflow()
}
