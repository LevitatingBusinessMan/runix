#![no_std]
#![no_main]
#![feature(const_mut_refs)]
#![feature(panic_info_message)]

use vga::Color;
mod panic;
mod vga;

const VGA: u32 = 0xb8000;
const VGA_SIZE: u32 = 25 * 80;

static WELCOME_STRING :&'static str = "Welcome to Runix!";

#[no_mangle]
// https://en.wikipedia.org/wiki/VGA_text_mode
pub extern fn _start() -> ! {

    vga::fill(Color::White);
    vga::print_at(vga::BUFFER_WIDTH/2 - WELCOME_STRING.len()/2, 12, WELCOME_STRING.as_bytes(), Color::Black, Color::White);

    loop{}
}
