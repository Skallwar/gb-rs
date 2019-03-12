use std::path;

use crate::cartridge;
use crate::cartridge::Cartridge;
use crate::ppu::Ppu;

pub struct Mmu {
    cartridge: Box<Cartridge>,
    ppu: Ppu,

    dmg: Vec<u8>,
    ram: Vec<u8>,
    ioports: Vec<u8>,
    hram: Vec<u8>,

    is_dmg: bool,
}

impl Mmu {
    pub fn new(path: &path::Path) -> Self {
        Mmu {
            cartridge: cartridge::new(path),
            ppu: Ppu::new(),

            ram: vec![0; 0xDFFF - 0xC000 + 1],
            ioports: vec![0; 0xFF7E - 0xFF00 + 1],
            hram: vec![0; 0xFFFE - 0xFF80 + 1],
            dmg: vec![
                0x31, 0xfe, 0xff, 0xaf, 0x21, 0xff, 0x9f, 0x32, 0xcb, 0x7c, 0x20, 0xfb, 0x21, 0x26,
                0xff, 0xe, 0x11, 0x3e, 0x80, 0x32, 0xe2, 0xc, 0x3e, 0xf3, 0xe2, 0x32, 0x3e, 0x77,
                0x77, 0x3e, 0xfc, 0xe0, 0x47, 0x11, 0x4, 0x1, 0x21, 0x10, 0x80, 0x1a, 0xcd, 0x95,
                0x0, 0xcd, 0x96, 0x0, 0x13, 0x7b, 0xfe, 0x34, 0x20, 0xf3, 0x11, 0xd8, 0x0, 0x6,
                0x8, 0x1a, 0x13, 0x22, 0x23, 0x5, 0x20, 0xf9, 0x3e, 0x19, 0xea, 0x10, 0x99, 0x21,
                0x2f, 0x99, 0xe, 0xc, 0x3d, 0x28, 0x8, 0x32, 0xd, 0x20, 0xf9, 0x2e, 0xf, 0x18,
                0xf3, 0x67, 0x3e, 0x64, 0x57, 0xe0, 0x42, 0x3e, 0x91, 0xe0, 0x40, 0x4, 0x1e, 0x2,
                0xe, 0xc, 0xf0, 0x44, 0xfe, 0x90, 0x20, 0xfa, 0xd, 0x20, 0xf7, 0x1d, 0x20, 0xf2,
                0xe, 0x13, 0x24, 0x7c, 0x1e, 0x83, 0xfe, 0x62, 0x28, 0x6, 0x1e, 0xc1, 0xfe, 0x64,
                0x20, 0x6, 0x7b, 0xe2, 0xc, 0x3e, 0x87, 0xe2, 0xf0, 0x42, 0x90, 0xe0, 0x42, 0x15,
                0x20, 0xd2, 0x5, 0x20, 0x4f, 0x16, 0x20, 0x18, 0xcb, 0x4f, 0x6, 0x4, 0xc5, 0xcb,
                0x11, 0x17, 0xc1, 0xcb, 0x11, 0x17, 0x5, 0x20, 0xf5, 0x22, 0x23, 0x22, 0x23, 0xc9,
                0xce, 0xed, 0x66, 0x66, 0xcc, 0xd, 0x0, 0xb, 0x3, 0x73, 0x0, 0x83, 0x0, 0xc, 0x0,
                0xd, 0x0, 0x8, 0x11, 0x1f, 0x88, 0x89, 0x0, 0xe, 0xdc, 0xcc, 0x6e, 0xe6, 0xdd,
                0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0xe, 0xec, 0xcc, 0xdd, 0xdc, 0x99,
                0x9f, 0xbb, 0xb9, 0x33, 0x3e, 0x3c, 0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c, 0x21,
                0x4, 0x1, 0x11, 0xa8, 0x0, 0x1a, 0x13, 0xbe, 0x20, 0xfe, 0x23, 0x7d, 0xfe, 0x34,
                0x20, 0xf5, 0x6, 0x19, 0x78, 0x86, 0x23, 0x5, 0x20, 0xfb, 0x86, 0x20, 0xfe, 0x3e,
                0x1, 0xe0, 0x50,
            ],

            is_dmg: true,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x00FF if self.is_dmg => self.dmg[addr as usize], //DMG
            0x0000...0x3FFF => self.cartridge.read_rom(addr),
            0xFF00...0xFF7F => self.ioports[addr as usize - 0xFF00],
            0xFF80...0xFFFE => self.hram[addr as usize - 0xFF80],

            // 0xFF00...0xFF7E => self.ram[addr as usize - 0xFF00],
            _ => panic!("Read at 0x{:X} not implemented", addr),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0xC000...0xDFFF => self.ram[addr as usize - 0xC000] = data,
            //VRAM
            0x8000...0x9FFF => self.ppu.write(addr - 0x8000, data),
            0xFF00...0xFF7F => self.ioports[addr as usize - 0xFF00] = data,
            0xFF80...0xFFFE => self.hram[addr as usize - 0xFF80] = data,

            _ => panic!("Write at 0x{:X} not implemented", addr),
        }
    }

    pub fn write_u16(&mut self, addr: u16, data: u16) {
        self.write(addr, (data & 0x00FF) as u8);
        self.write(addr + 1, (data >> 8) as u8);
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let mut word: u16 = self.read(addr) as u16;
        word += (self.read(addr + 1) as u16) << 8;

        word
    }
}
