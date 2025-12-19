use core::fmt;
use x86_64::instructions::port::PortWrite;

pub struct QemuDebug();

macro_rules! qemu {
    () => {
        
    };
}

impl fmt::Write for QemuDebug {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // for c in s.chars() {
        //     unsafe { u32::write_to_port(0xe9, c as u32) };
        // }
        for b in s.as_bytes() {
            unsafe { u8::write_to_port(0xe9, *b) };
        }
        Ok(())
    }
}
