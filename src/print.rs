//! print macros
use core::fmt::{self, Debug};

use spin::Mutex;

/// A locked console
pub static CONSOLE: Mutex<Option<Console>> = Mutex::new(Option::None);

/// Locked access to qemu debugcon device
pub static QEMU_DEBUG: Mutex<QemuDebug> = Mutex::new(QemuDebug());

/// initialize a main console for printing
/// using the first found framebuffer
pub fn init_console() {
    let fb = crate::FRAMEBUFFER_REQUEST.response().unwrap().framebuffers()[0];
    let mut console = Console::new(fb);
    console.clear();
    *CONSOLE.lock() = Some(console);
}

pub fn print_err(err: &'static str) {
    use fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        // if let Some(console) = &CONSOLE.lock() {
            
        // }
        // CONSOLE.lock().
        // PRINTER.lock().print_chars(err, Color::Yellow, Color::Red);
        // PRINTER.lock().write_char(' ').unwrap();
    });
}

// Helper function for the `print` macro to prevent deadlocks
pub fn print_args(args: fmt::Arguments) {
    use fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        if let Some(ref mut console) = &mut *CONSOLE.lock() {
            let _ = console.write_fmt(args);
        } else {
            panic!()
        }
        let qemu = &mut *QEMU_DEBUG.lock();
        let _ = qemu.write_fmt(args);
    });
}

use crate::{console::Console, qemu::QemuDebug};
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => (print!("\n"));
    ($($arg:tt)*) => (eprint!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! wprintln {
    () => (print!("\n"));
    ($($arg:tt)*) => (wprint!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! exprintln {
    () => (print!("\n"));
    ($($arg:tt)*) => (exprint!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::print::print_args(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! wprint {
    ($($arg:tt)*) => {{
        $crate::print::print_err("WARNING");
        $crate::print::print_args(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        $crate::print::print_err("ERR");
        $crate::print::print_args(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! exprint {
    ($($arg:tt)*) => {{
        $crate::print::print_err("EXCEPTION");
        $crate::print::print_args(format_args!($($arg)*));
    }};
}

pub use println;
