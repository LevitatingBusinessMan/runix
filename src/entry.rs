#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(panic_info_message)]
#![feature(ptr_metadata)]
#![feature(const_maybe_uninit_zeroed)]

use core::{cell::{OnceCell, UnsafeCell}, mem::MaybeUninit, sync::atomic::AtomicPtr};

use static_cell::StaticCell;
use vga::Color;
mod panic;
#[macro_use]
mod vga;
mod multiboot;
mod conf;

const VGA: u32 = 0xb8000;
const VGA_SIZE: u32 = 25 * 80;

static WELCOME_STRING :&'static str = "Welcome to Runix!";

use multiboot::BootInformation;

pub static MBI: spin::Once<&'static BootInformation> = spin::Once::new();

#[no_mangle]
// https://en.wikipedia.org/wiki/VGA_text_mode
pub extern fn runix(mbi_pointer: *const BootInformation) -> ! {
    vga::clear();

    let mbi = BootInformation::load(mbi_pointer);

    MBI.call_once(|| mbi);

    conf::parse(mbi.boot_command_line().expect("Could not get cmdline").to_str().unwrap());

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
        let cmdline = mbi.boot_command_line().unwrap();
    
        println!("Booted from: {}", &bootloader_name.to_str().unwrap());
    
        println!("Memory areas: ");
        let memory_map = mbi.memory_map().unwrap();
        for entry in &memory_map.entries {
            println!("    base: {:#14x}   size: {:#14x} (type {:#x})", entry.base_addr, entry.length, entry.type_)
        }
    }

    loop{}
}