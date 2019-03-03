use std::error::Error;
use std::fs;
use std::path;

pub trait Cartridge {
    // fn new(data: Vec<u8>) -> Self;

    // fn read_rom(self, addr: u16, len: usize) -> Vec<u8>;
    // fn read_ram(self, addr: u16, len: usize) -> Vec<u8>;

    // fn write_rom(self, addr: u16, data: Vec<u8>);
    // fn write_ram(self, addr: u16, data: Vec<u8>);
}

// enum CartridgeType {
//     ROM = 0x00,
//     MBC1 = 0x01,
//     MBC1RAM = 0x02,
//     MBC1RAMBAT = 0x03,
//     MBC2 = 0x05,
//     MBC2RAM = 0x06,
//     ROMRAM = 0x08,
//     ROMRAMBAT = 0x09,
//     MMM01 = 0x0B,
//     MMM01RAM = 0x0C,
//     MMM01RAMBAT = 0x0D,
//     MBC3TIMERBAT = 0x0F,
//     MBC3TIMERRAMBAT = 0x10,
//     MBC3 = 0x11,
//     MBC3RAM = 0x12,
//     MBC3RAMBAT = 0x13,
//     MBC4 = 0x15,
//     MBC4RAM = 0x16,
//     MBC4RAMBAT = 0x17,
//     MBC5 = 0x19,
//     MBC5RAM = 0x1A,
//     MBC5RAMBAT = 0x1B,
//     MBC5RUMBLE = 0x1C,
//     MBC5RUMBLERAM = 0x1D,
//     MBC5RUMBLERAMBAT = 0x1E,
// }

struct mbc0 {}

impl Cartridge for mbc0 {}

pub fn new(path: &path::Path) -> Box<Cartridge> {
    let rom = fs::read(path).unwrap();

    match rom[0x147] {
        0x00 => panic!("Find MBC0"),
        _ => panic!(),
    }

    Box::new(mbc0 {})
}
