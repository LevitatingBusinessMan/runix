#![no_std]
#![no_main]
mod panic;
mod vga;

const VGA: u32 = 0xb8000;
const VGA_SIZE: u32 = 25 * 80;

#[no_mangle]
// https://en.wikipedia.org/wiki/VGA_text_mode
pub extern fn _start() -> ! {

    let mut printer = vga::Printer::new();

    vga::fill(vga::Color::White);
    printer.print_chars("Welcome to Runix!", vga::Color::Black, vga::Color::White);

    loop{}
}
