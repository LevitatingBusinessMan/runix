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

#[no_mangle]
// https://en.wikipedia.org/wiki/VGA_text_mode
pub extern fn runix(mbi_pointer: *const multiboot::MultibootInformation) -> ! {
    vga::fill(Color::White);
    vga::print_at(vga::BUFFER_WIDTH/2 - WELCOME_STRING.len()/2, 12, WELCOME_STRING.as_bytes(), Color::Black, Color::White);

    let mbi = multiboot::MultibootInformation::load(mbi_pointer);

    println!("Multiboot: {:#7x?} - {:#7x?}", mbi_pointer, mbi_pointer as *const () as usize + mbi.total_size as usize);

    let cmdline = mbi.boot_command_line().unwrap();

    println!("Command line: {}", &cmdline.string.to_str().unwrap());

    loop{}
}
