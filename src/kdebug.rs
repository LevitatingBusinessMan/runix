//! A shell-like interface to debug the kernel with

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
            println!("elfsections");
            println!("mbi");
        },
        b"elfsections" => {
            let mbi = crate::MBI.get().unwrap();
            let elf = mbi.elf_symbols().next().unwrap();
            let addr = core::ptr::addr_of!(elf.num);

            println!("{:#x?}", elf);
            for i in 0..elf.num {
                let x = elf.section_headers[i as usize].addr;
                println!("{:x?}", x);
            }

            // for section in elf.sections() {
            //     let x = section.addr;
            //     //println!("{:x?}", x);
            // }
        },
        _ => {
            println!("Unknown command");
        }
    }
}
