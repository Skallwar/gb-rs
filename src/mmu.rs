use std::path;

use crate::cartridge as cartridge;
use crate::cartridge::Cartridge as Cartridge;

pub struct Mmu {
    cartridge: Box<Cartridge>,
}

impl Mmu {
    pub fn new(path: &path::Path) -> Self {
        Mmu {
            cartridge: cartridge::new(path),
        }
    }

    fn read(addr: u16) -> u8 {
        match addr {
            _ => panic!("Read at {} not implemented", addr),
        }
    }

    fn write(addr: u16) -> u8 {
        match addr {
            _ => panic!("Read at {} not implemented", addr),
        }
    }
}
