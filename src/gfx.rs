use crate::limine;

pub struct Color(u8, u8, u8);

impl Color {
    pub const WHITE: Color =  Color(0xFF, 0xFF, 0xFF);
    pub const BLACK: Color =  Color(0x00, 0x00, 0x00);
}

pub trait FramebufferGfxExtension {
    fn clear_grey(&self, scale: u8);
}

impl FramebufferGfxExtension for limine::Framebuffer {
    fn clear_grey(&self, scale: u8) {
        self.buffer().fill(scale);
    }
}
