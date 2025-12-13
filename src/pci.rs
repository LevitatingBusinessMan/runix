//!For working the PCI bus
//https://wiki.osdev.org/PCI
use x86_64::instructions::port::{PortWrite, PortRead};

// https://wiki.osdev.org/PCI#Configuration_Space_Access_Mechanism_#1
const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

pub fn config_read(bus: u8, device: u8, func: u8, offset: u8) -> u16 {
    let address = 0x80000000_u32 | (bus as u32) << 16 | (device as u32) << 11 | (func as u32) << 8 | (offset & 0xFC) as u32;
    unsafe { u32::write_to_port(CONFIG_ADDRESS, address) };
    // >> (offset & 2) * 8
    // should give us the first word of dword
    return unsafe { u32::read_from_port(CONFIG_DATA) >> (offset & 2) * 8 } as u16
}

pub mod scanner {
    use bit_field::BitField;

    use super::config_read;

    pub fn check_vendor(bus: u8, device: u8) -> Option<u16> {
        let vendor = config_read(bus, device, 0, 0);
        if vendor!= 0xFFFF {
            return Some(vendor);
        }
        return None;
    }

    fn check_bus(bus: u8) {
        for slot in 0..32 {
            check_device(bus, slot);
        }
    }

    fn check_device(bus: u8, slot: u8) {
        if let Some(vendor) = check_vendor(bus, slot) {
            let device = config_read(bus, slot, 0, 2);
            let header_type = config_read(bus, slot, 0, 0xe);
            if header_type.get_bit(7) {
                //check_functions
            }
            println!("PCI Bus {bus:#x} Vendor {vendor:#x} Device {device:#x} Header {header_type:#x}");
        }
    }

    pub fn brute_force() {
        for bus in 0..255 {
            check_bus(bus);
        }
    }
}
