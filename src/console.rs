//! drawing the console

use core::{ascii::Char, fmt, slice};

use crate::{fonts::{self, Font, UNSCII_16}, gfx::{self, Color, FramebufferGfxExtension}, limine, multiboot};

pub struct  Console {
    fb: &'static limine::Framebuffer,
    /// width in characters
    pub width: u32,
    /// height in characters
    pub height: u32,
    font: &'static Font,
    cursor: (u32, u32),
    color_fg: gfx::Color,
    color_bg: gfx::Color,
}

impl Console {
    pub const DEFAULT_FONT: &Font = &UNSCII_16;

    pub fn new(fb: &'static limine::Framebuffer) -> Self {
        let font = Self::DEFAULT_FONT;
        let width = fb.pitch as u32 / 8;
        let height = fb.height / font.height as u64;
        Self {
            fb,
            cursor: (0, 0),
            width: width as u32,
            height: height as u32,
            font,
            color_fg: gfx::Color::BLACK,
            color_bg: gfx::Color::WHITE,
        }
    }
    
    pub fn set_cursor(&mut self, cursor: (u32, u32)) {
        self.cursor = cursor;
    }
    
    fn print_char(&mut self, c: char) {
        // handle control characters
        match c {
            '\n' => {
                self.break_line();
                return;
            },
            _ => {}
        }
        
        let glyph = self.font.get(c);
        let y = self.font.height as usize * self.cursor.1 as usize;
        let x = 8 * self.cursor.0 as usize;
        let bytes_per_pixel = (self.fb.bpp / 8) as usize;
        let fbb = self.fb.buffer();
        let pitch = self.fb.pitch as usize;
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..8 {
                if (bits >> (7 - col)) & 1 == 1 {
                    let offset = (y + row) * pitch + x * bytes_per_pixel + col * bytes_per_pixel;
                    for i in 0..bytes_per_pixel {
                        fbb[offset + i as usize] = 0x00; // black
                    }
                }
            }
        }
        self.move_forward();
    }
    
    fn move_forward(&mut self) {
        if self.cursor.0 < self.width {
            self.cursor.0 += 1;
        } else { // wrap
            self.break_line();
        }
    }
    
    fn break_line(&mut self) {
        self.cursor.0 = 0;
        self.cursor.1 += 1;
        if self.cursor.1 >= self.height {
            self.roll();
            self.cursor.1 -= 1;
        }
    }
    
    pub fn clear(&mut self) {
        self.fb.clear_grey(0xFF);
        self.cursor = (0, 0);
    }
    
    /// move the cursor back a position
    pub fn backspace(&mut self) {
        self.cursor.0 = self.cursor.0.saturating_sub(1);
        self.clear_cell();
    }
    
    pub fn clear_cell(&mut self) {
        let y = self.font.height as usize * self.cursor.1 as usize;
        let x = 8 * self.cursor.0 as usize;
        let bytes_per_pixel = (self.fb.bpp / 8) as usize;
        let fbb = self.fb.buffer();
        let pitch = self.fb.pitch as usize;
        for row in 0..self.font.height as usize {
            // this can be done more efficiently
            for col in 0..8 {
                let offset = (y + row) * pitch + x * bytes_per_pixel + col * bytes_per_pixel;
                for i in 0..bytes_per_pixel {
                    fbb[offset + i as usize] = 0xFF; // white
                }
            }
        }
    }
    
    /// move all rows up 1 cell/row
    pub fn roll(&mut self) {
        let fbb = self.fb.buffer();
        let row_size = self.fb.pitch as usize * self.font.height as usize;
        for i in 1..self.height as usize {
            let offset = i * row_size;
            let (top, bot) = fbb.split_at_mut(offset);
            top[offset-row_size..].copy_from_slice(&bot[..row_size]);
        }
        // clear last row
        fbb[row_size*(self.height as usize - 1)..].fill(0xFF);
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.print_char(c);
        }
        Ok(())
    }
}
