use std::fs;
use std::path;

mod mbc0;

enum CartridgeTypes {
    MBC0 = 0x00,
}

const CARTRIDGE_TYPE_BYTE: usize = 0x147;

pub trait Cartridge {
    fn read_rom(&self, addr: u16) -> u8;
    fn read_ram(&self, addr: u16) -> u8;

    fn write_rom(&mut self, addr: u16, data: u8);
    fn write_ram(&self, addr: u16, data: u8);
}

pub fn new(path: &path::Path) -> Box<Cartridge> {
    let rom = fs::read(path).unwrap();

    match get_cartridge_type(rom[CARTRIDGE_TYPE_BYTE]) {
        CartridgeTypes::MBC0 => Box::new(mbc0::Mbc0::new(rom)),
    }
}

fn get_cartridge_type(type_byte: u8) -> CartridgeTypes {
    match type_byte {
        0x00 => CartridgeTypes::MBC0,
        _ => panic!("Not a Gameboy rom file !"),
    }
}
