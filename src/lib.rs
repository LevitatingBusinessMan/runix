#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(panic_info_message)]

use core::{arch::asm, f32::consts::E};

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

    let elf_sections_tag = boot_info.elf_sections()
    .expect("Elf-sections tag required");

    // println!("kernel sections:");
    // for section in elf_sections_tag.clone() {
    //     println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
    //         section.start_address(), section.size(), section.flags());
    // }

    let kernel_start = elf_sections_tag.clone().map(|s| s.start_address()).min().unwrap();
    let kernel_end = elf_sections_tag.clone().map(|s| s.start_address() + s.size()).max().unwrap();
    
    let multiboot_start = multiboot_information_pointer as u32;
    let multiboot_end = multiboot_start + (boot_info.total_size() as u32);

    println!("Kernel    at {:#7x} - {:#7x}", kernel_start, kernel_end);
    println!("Multiboot at {:#7x} - {:#7x}", multiboot_start, multiboot_end);
    
    println!();
    println!("Memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: {:#13x}, length:{:#13x}", area.start_address(), area.size());
    }

    loop{}
}
