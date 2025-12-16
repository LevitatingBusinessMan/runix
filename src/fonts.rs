pub mod hex;

/// A bitmap font
#[derive(Debug)]
pub struct Font {
    /// how many rows
    height: u8,
    /// pointer to the flattened array of glyphs
    inner: &'static [u8],
    /// how a char should be mapped to an index into the array
    mapping: fn(char) -> usize,
}

impl Font {
    /// get a glyph for a char
    pub fn get(&self, c: char) -> &'static [u8] {
        let index = (self.mapping)(c) * self.height as usize;
        &self.inner[index..index + self.height as usize]
    }
}
