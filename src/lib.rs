#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(panic_info_message)]
#![feature(ptr_metadata)]

use core::{arch::asm, f32::consts::E};
use core::ptr::addr_of;

use vga::Color;
mod panic;
#[macro_use]
mod vga;
mod multiboot;

const VGA: u32 = 0xb8000;
const VGA_SIZE: u32 = 25 * 80;

static WELCOME_STRING :&'static str = "Welcome to Runix!";

use multiboot::BootInformation;
//use multiboot2;

#[no_mangle]
// https://en.wikipedia.org/wiki/VGA_text_mode
pub extern fn runix(mbi_pointer: *const BootInformation) -> ! {
    vga::fill(Color::White);
    vga::print_at(vga::BUFFER_WIDTH/2 - WELCOME_STRING.len()/2, 12, WELCOME_STRING.as_bytes(), Color::Black, Color::White);

    let mbi = BootInformation::load(mbi_pointer);

    println!("Multiboot: {:#7x?} - {:#7x?}", mbi_pointer, mbi_pointer as *const () as usize + mbi.total_size as usize);

    // let cmdline = mbi.boot_command_line().unwrap();

    // println!("Command line: {}", &cmdline.to_str().unwrap());

    // println!("Memory areas: ");
    // let memory_map = mbi.memory_map().unwrap();
    // for entry in &memory_map.entries {
    //     println!("    base: {:#14x}   size: {:#14x} (type {:#x})", entry.base_addr, entry.length, entry.type_)
    // }

    // // let x = unsafe {
    // //     multiboot2::BootInformation::load(mbi_pointer as *const multiboot2::BootInformationHeader).unwrap()
    // // };

    let elfs = mbi.elf_symbols();

    for elf in elfs {
        println!("Got myself an elf")
    }

    loop{}
}
