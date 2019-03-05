use crate::mmu::cartridge::Cartridge;

pub struct Mbc0 {
    rom: Vec<u8>,
}

impl Mbc0 {
    pub fn new(data: Vec<u8>) -> Self {
        let rom = Rom { rom: data };
        Mbc0 { rom: rom }
    }
}

impl Cartridge for Mbc0 {
    fn read_rom(self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn read_ram(self, addr: u16) -> u8 {
        panic!("No RAM on MBC0 cartridge !");
    }

    fn write_rom(mut self, addr: u16, data: u8) {
        self.rom[addr as usize] = data;
    }

    fn write_ram(self, addr: u16) -> u8 {
        panic!("No RAM on MBC0 cartridge !");
    }
}
