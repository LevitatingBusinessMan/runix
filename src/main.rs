#![no_std]
#![no_main]
mod panic;

#[no_mangle]
pub extern fn _start() -> ! {
    loop {}
}
