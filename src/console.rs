//! drawing the console

use core::slice;

use crate::{fonts::{self, Font}, multiboot};

pub struct  Console {
    fb: limine::framebuffer::Framebuffer<'static>,
    /// width in characters
    width: u32,
    /// height in characters
    height: u32,
    font: &'static Font,
    cursor: (u32, u32),
}

impl Console {
    pub fn new(fb: limine::framebuffer::Framebuffer<'static>) -> Self {
        Self {
            fb,
            cursor: (0, 0),
            width: 0,
            height: 0,
            font: &fonts::hex::UNSCII_FANTASY_8,
        }
    }
    pub fn test(&self) {
        let buffer_length = self.fb.height() * self.fb.pitch();
        let fbb = unsafe { slice::from_raw_parts_mut(self.fb.addr(), buffer_length as usize) };
        let bytes_per_pixel = self.fb.bpp() / 8;
        let pitch = self.fb.pitch();
        for (char_column, c) in "Panic!".chars().enumerate() {
            let glyph = self.font.get(c);
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
