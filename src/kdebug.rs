//! A shell-like interface to debug the kernel with

use core::ptr::slice_from_raw_parts;

use crate::{debug, keyboard, vga};

pub fn kdebug() -> ! {
    let mut kr = keyboard::KeyReader::new();
    loop {
        print!("kdebug> ");
        let mut command: [u8; 32] = [0; 32];
        let mut index = 0;
        loop {
            let key = kr.get_key();
            if let Ok(c) = <keyboard::ps2::KeyCode as TryInto<char>>::try_into(key) {
                if c != '\n' {
                    if index < 32 {
                        command[index] = c as u8;
                        index += 1;
                        print!("{}", c);
                    }
                } else {
                    let cmd = &command[..index];
                    println!();
                    handle_cmd(cmd);
                    break;
                }
            } else {
                if key == keyboard::ps2::KeyCode::Backspace {
                    index -= 1;
                    vga::PRINTER.lock().col -= 1;
                }
            }
        }
    }
}

fn handle_cmd(cmd: &[u8]) {
    match cmd {
        b"help" => {
            println!("List of commands:");
            println!("sections");
            println!("memory");
            println!("registers")
        },
        b"sections" => debug::print_elfsections(),
        b"memory" => debug::print_memoryareas(),
        b"registers" => debug::print_registers(),
        _ => {
            println!("Unknown command");
        }
    }
}
