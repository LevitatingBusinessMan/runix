//! Handles the keyboard interrupt

use x86_64::instructions::port::PortReadOnly;
use x86_64::structures::idt::InterruptStackFrame;
use spin::RwLock;

use self::ps2::KeyCode;

use super::pic8259;
use super::Keyboard;

const PS2: PortReadOnly<u8> = PortReadOnly::new(0x60);

struct KeyBuffer {
    buffer: [KeyCode; 256],
    len: usize,
}

static KEYBUFFER: RwLock<KeyBuffer> = RwLock::new(KeyBuffer {buffer: [KeyCode::Unknown; 256], len: 0});

// I should not be able to receive keyboard interrupts during
// the handling of a keyboard interrupt. So a deadlock should not occur.
#[allow(const_item_mutation)]
pub(super) extern "x86-interrupt" fn handler(_stack_frame: InterruptStackFrame) {
    let scancode = unsafe {PS2.read()};
    if let Some(keyevent) = ps2::decode_scancode(scancode) {
        if keyevent.state == ps2::State::Press {
            let mut kb = KEYBUFFER.write();
            let len = kb.len;
            kb.buffer[len] = keyevent.key;
            kb.len += 1;
            if kb.len >= kb.buffer.len() {
                kb.len = 0;
            }
        }
    }
    pic8259::send_eoi(Keyboard as u8);
}

/// For reading keystrokes
pub struct KeyReader {
    /// Index into the global keybuffer
    index: usize,
}

impl KeyReader {
    pub fn new() -> Self {
        Self {index: 0}
    }
    /// Try to get a key immediately
    pub fn try_key(&mut self) -> Option<KeyCode> {
        let kb = KEYBUFFER.read();
        if kb.len > self.index {
            self.index += 1;
            if self.index >= kb.buffer.len() {
                self.index = 0;
            }
            Some(kb.buffer[self.index-1])
        } else {
            None
        }
    }
    /// Waits for a key using a hlt loop
    pub fn get_key(&mut self) -> KeyCode {
        loop {
            if let Some(key) = self.try_key() { return key; }
            x86_64::instructions::hlt();
        }
    }
}

pub mod ps2 {
    use bit_field::BitField;
    use num_enum::TryFromPrimitive;

    pub struct KeyEvent {
        pub state: State,
        pub key: KeyCode,
    }

    pub fn decode_scancode(scancode: ScanCode) -> Option<KeyEvent> {
        let state = State::from(scancode);
        let key = KeyCode::try_from(*scancode.clone().set_bit(7, false)).unwrap_or(KeyCode::Unknown);
        if key == KeyCode::Unknown {
            wprintln!("Unknown scancode {:#x} {:#x}", scancode, scancode.clone().set_bit(7, false));
        }
        Some(KeyEvent {state, key})
    }

    //pub struct ScanCode (pub u8);
    pub type ScanCode = u8;

    #[derive(PartialEq)]
    pub enum State {
        Press,
        Release
    }

    impl From<ScanCode> for State {
        fn from(value: ScanCode) -> Self {
            match value.get_bit(7) {
                true => Self::Release,
                false => Self::Press,
            }
        }
    }

    /// Using scancode set 1
    #[derive(Clone, Copy, TryFromPrimitive, PartialEq, Debug)]
    #[repr(u8)]
    pub enum KeyCode {
        Escape = 0x1,
        One = 0x2,
        Two = 0x3,
        Three = 0x4,
        Four = 0x5,
        Five = 0x6,
        Six = 0x7,
        Seven = 0x8,
        Eight = 0x9,
        Nine = 0xA,
        Zero = 0xB,
        Minus = 0xC,
        Equals = 0xD,
        Backspace = 0xE,
        Tab = 0xF,
        Q = 0x10,
        W = 0x11,
        E = 0x12,
        R = 0x13,
        T = 0x14,
        Y = 0x15,
        U = 0x16,
        I = 0x17,
        O = 0x18,
        P = 0x19,
        LeftBracket = 0x1A,
        RightBracket = 0x1B,
        Enter = 0x1C,
        LeftControl = 0x1D,
        A = 0x1E,
        S = 0x1F,
        D = 0x20,
        F = 0x21,
        G = 0x22,
        H = 0x23,
        J = 0x24,
        K = 0x25,
        L = 0x26,
        Semicolon = 0x27,
        Apostrophe = 0x28,
        BackTick = 0x29,
        LeftShift = 0x2A,
        BackSlash = 0x2B,
        Z = 0x2C,
        X = 0x2D,
        C = 0x2E,
        V = 0x2F,
        B = 0x30,
        N = 0x31,
        M = 0x32,
        Comma = 0x33,
        Period = 0x34,
        ForwardSlash = 0x35,
        RightShift = 0x36,
        NumPadStar = 0x37,
        LeftAlt = 0x38,
        Space = 0x39,
        CapsLock = 0x3A,
        F1 = 0x3B,
        F2 = 0x3C,
        F3 = 0x3D,
        F4 = 0x3E,
        F5 = 0x3F,
        F6 = 0x40,
        F7 = 0x41,
        F8 = 0x42,
        F9 = 0x43,
        F10 = 0x44,
        NumLock = 0x45,
        ScrollLock = 0x46,
        NumPad7 = 0x47,
        NumPad8 = 0x48,
        NumPad9 = 0x49,
        NumPadMinus = 0x4A,
        NumPad4 = 0x4B,
        NumPad5 = 0x4C,
        NumPad6 = 0x4D,
        NumPadPlus = 0x4E,
        NumPad1 = 0x4F,
        NumPad2 = 0x50,
        NumPad3 = 0x51,
        NumPad0 = 0x52,
        NumPadPeriod = 0x53,
        F11 = 0x57,
        F12 = 0x58,
        Unknown,
    }

    /// Get a character from the key if possible
    impl TryFrom<KeyCode> for char {
        type Error = ();

        fn try_from(value: KeyCode) -> Result<char, ()> {
            match value {
                KeyCode::One => Ok('1'),
                KeyCode::Two => Ok('2'),
                KeyCode::Three => Ok('3'),
                KeyCode::Four => Ok('4'),
                KeyCode::Five => Ok('5'),
                KeyCode::Six => Ok('6'),
                KeyCode::Seven => Ok('7'),
                KeyCode::Eight => Ok('8'),
                KeyCode::Nine => Ok('9'),
                KeyCode::Zero => Ok('0'),
                KeyCode::Minus => Ok('-'),
                KeyCode::Equals => Ok('='),
                //KeyCode::Tab => Ok('\t'),
                KeyCode::Q => Ok('q'),
                KeyCode::W => Ok('w'),
                KeyCode::E => Ok('e'),
                KeyCode::R => Ok('r'),
                KeyCode::T => Ok('t'),
                KeyCode::Y => Ok('y'),
                KeyCode::U => Ok('u'),
                KeyCode::I => Ok('i'),
                KeyCode::O => Ok('o'),
                KeyCode::P => Ok('p'),
                KeyCode::LeftBracket => Ok('['),
                KeyCode::RightBracket => Ok(']'),
                KeyCode::Enter => Ok('\n'),
                KeyCode::Space => Ok(' '),
                KeyCode::A => Ok('a'),
                KeyCode::S => Ok('s'),
                KeyCode::D => Ok('d'),
                KeyCode::F => Ok('f'),
                KeyCode::G => Ok('g'),
                KeyCode::H => Ok('h'),
                KeyCode::J => Ok('j'),
                KeyCode::K => Ok('k'),
                KeyCode::L => Ok('l'),
                KeyCode::Semicolon => Ok(';'),
                KeyCode::Apostrophe => Ok('\''),
                KeyCode::BackTick => Ok('`'),
                KeyCode::BackSlash => Ok('\\'),
                KeyCode::Z => Ok('z'),
                KeyCode::X => Ok('x'),
                KeyCode::C => Ok('c'),
                KeyCode::V => Ok('v'),
                KeyCode::B => Ok('b'),
                KeyCode::N => Ok('n'),
                KeyCode::M => Ok('m'),
                KeyCode::Comma => Ok(','),
                KeyCode::Period => Ok('.'),
                KeyCode::ForwardSlash => Ok('/'),
                KeyCode::NumPadStar => Ok('*'),
                KeyCode::NumPad7 => Ok('7'),
                KeyCode::NumPad8 => Ok('8'),
                KeyCode::NumPad9 => Ok('9'),
                KeyCode::NumPadMinus => Ok('-'),
                KeyCode::NumPad4 => Ok('4'),
                KeyCode::NumPad5 => Ok('5'),
                KeyCode::NumPad6 => Ok('6'),
                KeyCode::NumPadPlus => Ok('+'),
                KeyCode::NumPad1 => Ok('1'),
                KeyCode::NumPad2 => Ok('2'),
                KeyCode::NumPad3 => Ok('3'),
                KeyCode::NumPad0 => Ok('0'),
                KeyCode::NumPadPeriod => Ok('.'),
                _ => Err(())
            }
        }
    }
}
