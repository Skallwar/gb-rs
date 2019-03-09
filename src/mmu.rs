use std::path;

use crate::cartridge;
use crate::cartridge::Cartridge;

pub struct Mmu {
    cartridge: Box<Cartridge>,

    ram: Vec<u8>,
    hram: Vec<u8>,
}

impl Mmu {
    pub fn new(path: &path::Path) -> Self {
        Mmu {
            cartridge: cartridge::new(path),

            ram: vec![0; 0xDFFF - 0xC000],
            hram: vec![0; 0xFFFE - 0xFF80],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x3FFF => self.cartridge.read_rom(addr),
            _ => panic!("Read at 0x{:X} not implemented", addr),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            //RAM
            0xC000...0xDFFF => self.ram[addr as usize - 0xC000 - 1] = data,
            //HRAM
            0xFF80...0xFFFE => self.hram[addr as usize - 0xFF80 - 1] = data,

            _ => panic!("Write at 0x{:X} not implemented", addr),
        }
    }
}
