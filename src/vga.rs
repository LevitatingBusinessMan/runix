// https://en.wikipedia.org/wiki/VGA_text_mode
// https://en.wikipedia.org/wiki/Code_page_437

// https://wiki.osdev.org/Printing_to_Screen#Color_Table
// https://doc.rust-lang.org/core/mem/struct.Discriminant.html
// I gave all these discriminators, but it's just 0 to 15

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;
const BUFFER: *mut VGABuffer = 0xb8000 as *mut VGABuffer;

#[repr(C)]
#[derive(Clone, Copy)]
struct ScreenCharacter {
    character: u8,
    color: ColorAttribute,
}

impl ScreenCharacter {
    pub fn new(color: ColorAttribute, character: u8) -> Self {
        ScreenCharacter { color, character }
    }
}

#[derive(Clone, Copy)]
struct ColorAttribute(u8);

impl From<(Color, Color)> for ColorAttribute {
    fn from((fg,bg): (Color, Color)) -> Self {
        ColorAttribute ((bg as u8) << 4 | (fg as u8))
    }
}

struct VGABuffer([[ScreenCharacter; BUFFER_WIDTH]; BUFFER_HEIGHT]);

pub struct Printer {
    column: usize,
    row: usize,
}

impl Printer {
    pub fn new() -> Self {
        Printer { row: 0, column: 0 }
    }
    pub fn print_byte(&mut self, character: u8, fg: Color, bg: Color) {
        if character == '\n' as u8 {
            self.row += 1;
            self.column = 0;
            return;
        }

        let sc = ScreenCharacter {
            color:(fg,bg).into(),
            character,
        };
        // Write the word
        unsafe {(*BUFFER).0[self.row][self.column] = sc};
        self.column += 1;
    }
    pub fn print_chars(&mut self, characters: &str, fg: Color, bg: Color) {
        for c in characters.chars() {
            self.print_byte(c as u8, fg, bg);
        }
    }
}

/// Fill screen with solid color
pub fn fill(bg: Color) {
    let sc = ScreenCharacter {
        color: (Color::White,bg).into(),
        character: 0x20,
    };
    for i in 0..BUFFER_HEIGHT {
        for j in 0..BUFFER_WIDTH {
            unsafe {(*BUFFER).0[i][j] = sc};
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Color {
    Black       =  0b000,
    Blue        =  0b001,
    Green       =  0b010,
    Cyan        =  0b011,
    Red         =  0b100,
    Magenta     =  0b101,
    Brown       =  0b110,
    Gray        =  0b111,
    DarkGray    = 0b1000,
    LightBlue   = 0b1001,
    LightGreen  = 0b1010,
    LightCyan   = 0b1011,
    LightRed    = 0b1100,
    LightMagenta= 0b1101,
    Yellow      = 0b1110,
    White       = 0b1111,
}
