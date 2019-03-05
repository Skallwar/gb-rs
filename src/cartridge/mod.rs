use std::fs;
use std::path;

mod mbc0;

pub trait Cartridge {
    fn read_rom(&self, addr: u16) -> u8;
    fn read_ram(&self, addr: u16) -> u8;

    fn write_rom(&mut self, addr: u16, data: u8);
    fn write_ram(&self, addr: u16, data: u8);
}

pub fn new(path: &path::Path) -> Box<Cartridge> {
    let rom = fs::read(path).unwrap();

    match rom[0x147] {
        0x00 => return Box::new(mbc0::Mbc0::new(rom)),
        _ => panic!("Not a Gameboy rom file !"),
    }
}
