//! A shell-like interface to debug the kernel with

use core::ptr::{addr_of, slice_from_raw_parts};

use crate::{debug, keyboard, pci, vga};

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
                    if index > 0 {
                        index -= 1;
                        vga::PRINTER.lock().col -= 1;
                        print!(" "); // clear character
                        vga::PRINTER.lock().col -= 1;
                    }
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
            println!("registers");
            println!("mbi");
            println!("stackoverflow");
            println!("pagefault");
            println!("scanpci");
            println!("mbitags");
            println!("clean");
        },
        b"sections" => debug::print_elfsections(),
        b"memory" => debug::print_memoryareas(),
        b"registers" => debug::print_registers(),
        b"mbi" => {
            let mbi = crate::MBI.get().unwrap();
            println!("Multiboot at: {:#7x?} - {:#7x?}", addr_of!(**mbi) as *const (), addr_of!(**mbi) as *const () as usize + mbi.total_size as usize);
        },
        b"stackoverflow" => {
            debug::stack_overflow();
        },
        b"pagefault" => {
            debug::page_fault();
        },
        b"scanpci" => {
            pci::scanner::brute_force();
        },
        b"mbitags" => {
            let mbi = crate::MBI.get().unwrap();
            for tag in mbi.tags() {
                println!("{tag:?}");
            }
        },
        b"clear" => {
            vga::clear();
            vga::PRINTER.lock().col = 0;
            vga::PRINTER.lock().row = 0;
        },
        _ => {
            println!("Unknown command");
        }
    }
}
