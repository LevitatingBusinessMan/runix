use crate::limine;

pub struct Color(u32);

impl Color {
    pub const WHITE: Color =  Color::rgb(0xFF, 0xFF, 0xFF);
    pub const BLACK: Color =  Color::rgb(0x00, 0x00, 0x00);
    pub const PANIC_RED: Color =  Color(0x00DC143C);
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Self((r as u32) << 16 | (g as u32) << 8 | b as u32 | 0xFF000000)
    }
}

pub trait FramebufferGfxExtension {
    fn clear_grey(&self, scale: u8);
    fn clear(&self, color: Color);
}

impl FramebufferGfxExtension for limine::Framebuffer {
    fn clear_grey(&self, scale: u8) {
        self.buffer().fill(scale);
    }
    fn clear(&self, color: Color) {
        // NOTE I still have to consider just assuming 32bpp
        let buffer: &mut [u32] = bytemuck::cast_slice_mut(self.buffer());
        buffer.fill(color.0);
    }
}
