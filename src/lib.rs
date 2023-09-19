#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(panic_info_message)]

use core::arch::asm;

use vga::Color;
mod panic;
mod vga;

use multiboot2::{self, BootInformation, BootInformationHeader};

const VGA: u32 = 0xb8000;
const VGA_SIZE: u32 = 25 * 80;

static WELCOME_STRING :&'static str = "Welcome to Runix!";

#[no_mangle]
// https://en.wikipedia.org/wiki/VGA_text_mode
pub extern fn runix(multiboot_information_pointer: *const BootInformationHeader) -> ! {
    vga::fill(Color::White);
    vga::print_at(vga::BUFFER_WIDTH/2 - WELCOME_STRING.len()/2, 12, WELCOME_STRING.as_bytes(), Color::Black, Color::White);

    let boot_info = unsafe { BootInformation::load(multiboot_information_pointer).unwrap() };

    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");
    
    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}", area.start_address(), area.size());
    }

    loop{}
}
