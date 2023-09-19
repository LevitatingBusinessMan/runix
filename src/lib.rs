#![no_std]
#![no_main]
mod panic;

const VGA: u32 = 0xb8000;
const VGA_SIZE: u32 = 25 * 80;

#[no_mangle]
// https://en.wikipedia.org/wiki/VGA_text_mode
pub extern fn _start() -> ! {

    for i in 0..(VGA_SIZE) {
        unsafe { *((VGA + i * 2) as *mut _) = 0xf020 as u16}
    }

    // ATTENTION: we have a very small stack and no guard page
    let hello = b"Hello World!";
    let mut hello_colored = [0xf0; 24];
    
    // Fill in the text for every other byte
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    unsafe { *((VGA + 1988) as *mut _)  = hello_colored };

    loop{}
}
