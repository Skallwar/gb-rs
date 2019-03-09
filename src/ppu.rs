pub struct Ppu {
    vram: Vec<u8>,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: vec![0; 0x9FFF - 0x8000],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.vram[addr as usize] = data;
    }
}
