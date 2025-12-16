//! drawing the console

use core::slice;

use crate::{fonts, multiboot};

pub struct  Console {
    fb: limine::framebuffer::Framebuffer<'static>
}
impl Console {
    pub fn new(fb: limine::framebuffer::Framebuffer<'static>) -> Self {
        Self {
            fb
        }
    }
    pub fn test(&self) {
        let buffer_length = self.fb.height() * self.fb.pitch();
        let fbb = unsafe { slice::from_raw_parts_mut(self.fb.addr(), buffer_length as usize) };
        let bytes_per_pixel = self.fb.bpp() / 8;
        let pitch = self.fb.pitch();
        for (char_column, c) in "Hello World".chars().enumerate() {
            let glyph = fonts::hex::UNSCII_FANTASY_8[c as usize];
            for (bit_row, bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if (bits >> (7 - col)) & 1 == 1 {
                        let offset = bit_row as usize * pitch as usize + col as usize * bytes_per_pixel as usize + char_column * 8 * bytes_per_pixel as usize;
                        for i in 0..bytes_per_pixel {
                            fbb[offset + i as usize] = 0xFF; // White
                        }
                    }
                }
            }
        }
    }
}
