//! A shell-like interface to debug the kernel with

use core::ptr::{addr_of, slice_from_raw_parts};

use crate::{console, debug, hbreak, keyboard, limine::MemMapType, pci, print};

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
                        print::CONSOLE.lock().as_mut().unwrap().backspace();
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
            // println!("sections");
            // println!("memory");
            println!("registers");
            // println!("mbi");
            println!("stackoverflow");
            println!("pagefault");
            println!("scanpci");
            // println!("mbitags");
            println!("memmap");
            println!("hbreak");
            println!("usable");
            println!("executable");
            println!("hhdm");
            println!("clear");
        },
        // b"sections" => debug::print_elfsections(),
        // b"memory" => debug::print_memoryareas(),
        b"registers" => debug::print_registers(),
        // b"mbi" => {
        //     let mbi = crate::MBI.get().unwrap();
        //     println!("Multiboot at: {:#7x?} - {:#7x?}", addr_of!(**mbi) as *const (), addr_of!(**mbi) as *const () as usize + mbi.total_size as usize);
        // },
        b"stackoverflow" => {
            debug::stack_overflow();
        },
        b"pagefault" => {
            debug::page_fault();
        },
        b"scanpci" => {
            pci::scanner::brute_force();
        },
        // b"mbitags" => {
        //     let mbi = crate::MBI.get().unwrap();
        //     for tag in mbi.tags() {
        //         println!("{tag:?}");
        //     }
        // },
        b"memmap" => {
            debug::print_limine_memory_map();
        },
        b"usable" => {
            println!("{:>16} {:>16} {}", "START", "END", "SIZE of usable memory regions"); 
            for entry in crate::MEMMAP_REQUEST.response().unwrap().entries() {
                if matches!(entry.r#type, MemMapType::Usable) {
                    println!("{:>16x?} {:>16x?} {:x?}", entry.base, (entry.base + entry.length), entry.length)
                }
            }
        },
        b"executable" => {
            println!("{:x?}", crate::EXECUTABLE_ADDRESS_REQUEST.response().unwrap());  
        },
        b"hhdm" => {
          println!("{:x?}", crate::HHDM_REQUEST.response().unwrap())  
        },
        b"hbreak" => {
            hbreak!();
        },
        b"clear" => {
            print::CONSOLE.lock().as_mut().unwrap().clear();
        },
        _ => {
            println!("Unknown command");
        }
    }
}
