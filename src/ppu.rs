pub struct Ppu {
    vram: Vec<u8>,

    pub LCDC_Control: u8,
    // LCDC_Status: u8,
    pub SCY: u8,
    // SCX: u8,
    pub LY: u8,
    // LYC: u8,
    // WY: u8,
    // WX: u8,
    pub BG_ColorPalette: u8,
}

enum Colors {
    BLACK = 0x00000000,
    WHITE = 0xFFFFFFFF,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: vec![0; 0x9FFF - 0x8000 + 1],

            LCDC_Control: 0,
            SCY: 0,
            LY: 0,

            BG_ColorPalette: 0,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.vram[addr as usize] = data;
    }

    fn find_color(&self, color: u8) -> Colors {
        match color {
            0b00 => Colors::WHITE,
            _ => Colors::BLACK,
        }
    }

    fn bg_map_get_tile_number(&self, x: u8, y: u8) -> u8 {
        self.vram[(0x9800 + (y * 0x20 + x) as u16) as usize]
    }

    fn tile_addr(&self, num: u8) -> u16 {
        num as u16 * 0x10
    }
}

//Debug
impl Ppu {
    pub fn tile_print(&self, num: u8) {
        let addr = self.tile_addr(num);

        for i in (addr..addr + 0x10).step_by(2) {
            let lsb = self.vram[i as usize];
            let msb = self.vram[(i + 1) as usize];

            for j in 0..8 {
                //Color in 2 bits format
                // lsb & (1 << (7 - x))
                let msb_color = ((msb & (1 << 7 - j) > 0) as u8) << 1;
                let lsb_color = (lsb & (1 << 7 - j) > 0) as u8;
                let color_num = msb_color + lsb_color;
                let pix_color = self.find_color(color_num) as u32;

                if pix_color == Colors::BLACK as u32 {
                    print!("1");
                } else {
                    print!(" ");
                }
            }

            println!("");
        }
    }
}
