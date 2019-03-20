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

const VIEWPORT_SIZE_X: usize = 160;
const VIEWPORT_SIZE_Y: usize = 144;

//Memory management
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
}

//Frame creation
impl Ppu {
    fn frame_gen(&mut self) -> Vec<u32> {
        let mut pixs = Vec::new();

        println!("SCX: {}, SCY: {}", 0, self.SCY);

        self.LY = 0;

        while self.LY < VIEWPORT_SIZE_Y as u8 {
            let mut pixs_line = self.get_line_pixs(self.LY);
            pixs.append(&mut pixs_line);

            self.LY += 1;
        }

        //Should be in V-Blank

        pixs
    }

    fn get_line_pixs(&self, line: u8) -> Vec<u32> {
        let mut pixs = Vec::with_capacity(VIEWPORT_SIZE_X);

        let bg_y = (self.SCY + line) / 8;
        for x in 0..VIEWPORT_SIZE_X {
            let col = x as u8; //TODO add SCX
            let bg_x = col / 8;

            let tile_nb = self.bg_map_get_tile_number(bg_x, bg_y);
            pixs.push(self.tile_get_pix(tile_nb, col % 8, line % 8));
        }

        pixs
    }

    fn tile_get_pix(&self, num: u8, x: u8, y: u8) -> u32 {
        let addr = self.tile_addr(num);
        let i = addr + y as u16 * 2;
        let lsb = self.vram[i as usize];
        let msb = self.vram[(i + 1) as usize];

        let msb_color = ((msb & (1 << 7 - x) > 0) as u8) << 1;
        let lsb_color = (lsb & (1 << 7 - x) > 0) as u8;
        let color_num = msb_color + lsb_color;

        self.pix_find_color(color_num) as u32
    }

    fn tile_addr(&self, num: u8) -> u16 {
        num as u16 * 0x10
    }

    fn pix_find_color(&self, color: u8) -> Colors {
        match color {
            0b00 => Colors::WHITE,
            _ => Colors::BLACK,
        }
    }

    fn bg_map_get_tile_number(&self, x: u8, y: u8) -> u8 {
        let index = 0x9800 - 0x8000 + (y as u16 * 0x20 + x as u16);
        self.vram[index as usize]
    }
}

//Debug
impl Ppu {
    pub fn tile_print(&self, num: u8) {
        for j in 0..8 {
            for i in 0..8 {
                let pix_color = self.tile_get_pix(num, i, j);
                if pix_color == Colors::BLACK as u32 {
                    print!("1");
                } else {
                    print!(" ");
                }
            }

            println!("");
        }
    }

    pub fn frame_print(&mut self) {
        let frame = self.frame_gen();

        for j in 0..VIEWPORT_SIZE_Y {
            for x in 0..VIEWPORT_SIZE_X {
                if frame[(j * VIEWPORT_SIZE_X + x) as usize] == 0 {
                    print!("1");
                } else {
                    print!(" ");
                }
            }

            println!();
        }
    }
}
