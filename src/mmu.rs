use std::path;

use crate::cartridge;
use crate::cartridge::Cartridge;

pub struct Mmu {
    cartridge: Box<Cartridge>,
}

impl Mmu {
    pub fn new(path: &path::Path) -> Self {
        Mmu {
            cartridge: cartridge::new(path),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x3FFF => self.cartridge.read_rom(addr),
            _ => panic!("Read at 0x{:X} not implemented", addr),
        }
    }

    pub fn write(&self, addr: u16) -> u8 {
        match addr {
            _ => panic!("Write at 0x{:X} not implemented", addr),
        }
    }
}
