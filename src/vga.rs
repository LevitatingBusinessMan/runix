// https://en.wikipedia.org/wiki/VGA_text_mode
// https://en.wikipedia.org/wiki/Code_page_437
use core::fmt;
use volatile::Volatile;
use spin::{Mutex, Lazy};

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        $crate::vga::PRINTER.lock().write_fmt(format_args!($($arg)*)).unwrap();
    }};
}

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

//static VGA: &'static mut VGABuffer = unsafe { &mut *(0xb8000 as *mut VGABuffer) };
const VGA: *mut VGABuffer = 0xb8000 as *mut VGABuffer;

pub static PRINTER: Lazy<Mutex<Printer>> = Lazy::new(||
    Mutex::new(Printer { column: 0, row: 0 })
);

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

type VGABuffer = [[Volatile<ScreenCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT];

/// Prints black on white from first row
/// To specify location and color see `ColoredPrinter`
pub struct Printer {
    column: usize,
    row: usize,
}

impl Printer {
    pub fn new() -> Self {
        Printer { row: 0, column: 0 }
    }
    pub fn print_byte(&mut self, character: u8, fg: Color, bg: Color) {
        //let mut x : 
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
        unsafe {(*VGA)[self.row][self.column].write(sc);};
        self.column += 1;
    }
    pub fn print_chars(&mut self, characters: &str, fg: Color, bg: Color) {
        for c in characters.chars() {
            self.print_byte(c as u8, fg, bg);
        }
    }
}

impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_chars(s, Color::Black, Color::White);
        fmt::Result::Ok(())
    }
}

/// For use with `fmt::Write` so you can write with a specified color
pub struct ColoredPrinter {
    printer: Printer,
    fg: Color,
    bg: Color,
}

impl ColoredPrinter {
    pub fn new(x: usize, y: usize, fg: Color, bg: Color) -> Self {
        ColoredPrinter { printer: Printer {row: y, column: x}, fg, bg}
    }
}

impl fmt::Write for ColoredPrinter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.printer.print_chars(s, self.fg, self.bg);
        fmt::Result::Ok(())
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
            unsafe {(*VGA)[i][j].write(sc)};
        }
    }
}

/// Print at a specific row and column
pub fn print_at(x: usize, y: usize, bytes: &[u8], fg: Color, bg: Color) {
    let x_init = x;
    let mut x = x_init;
    let mut y = y;
    for byte in bytes {
        if *byte == '\n' as u8 {
            y += 1;
            x = x_init;
            continue;
        }
        let sc = ScreenCharacter {
            color: (fg,bg).into(),
            character: *byte,
        };
        unsafe {(*VGA)[y][x].write(sc)};
        x += 1;
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
// https://wiki.osdev.org/Printing_to_Screen#Color_Table
// I gave all these discriminators, but it's just 0 to 15
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
