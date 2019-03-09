#[allow(non_snake_case)]
pub struct Registers {
    pub A: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,
    pub F: u8,
    pub SP: u16,
    pub PC: u16,
}

pub enum FlagsMasks {
    Z = 0b10000000,
    N = 0b01000000,
    H = 0b00100000,
    C = 0b00010000,
}

#[allow(non_snake_case)]
impl Registers {
    pub fn new() -> Self {
        Registers {
            A: 0x01,
            B: 0x00,
            C: 0x13,
            D: 0x00,
            E: 0xD8,
            H: 0x01,
            L: 0x4D,
            F: 0xB0,
            SP: 0xFFFE,
            PC: 0x0000,
        }
    }

    //Getters
    pub fn AF(&self) -> u16 {
        //AF returns only A
        (self.A as u16) << 8
    }

    pub fn BC(&self) -> u16 {
        ((self.B as u16) << 8) + self.C as u16
    }

    pub fn DE(&self) -> u16 {
        ((self.D as u16) << 8) + self.E as u16
    }

    pub fn HL(&self) -> u16 {
        ((self.H as u16) << 8) + self.L as u16
    }

    //Setters
    pub fn set_AF(&mut self, data: u16) {
        //AF contains only A
        self.A = (data >> 8) as u8;
    }

    pub fn set_BC(&mut self, data: u16) {
        self.B = (data >> 8) as u8;
        self.C = (data & 0x00FF) as u8;
    }

    pub fn set_DE(&mut self, data: u16) {
        self.D = (data >> 8) as u8;
        self.E = (data & 0x00FF) as u8;
    }

    pub fn set_HL(&mut self, data: u16) {
        self.H = (data >> 8) as u8;
        self.L = (data & 0x00FF) as u8;
    }

    pub fn inc_PC(&mut self) {
        self.PC += 1;
    }

    //Flags
    pub fn get_flag(&self, mask: FlagsMasks) -> bool {
        (self.F & mask as u8) > 0
    }

    pub fn set_flag(&mut self, mask: FlagsMasks, active: bool) {
        if active {
            self.F |= mask as u8;
        } else {
            self.F &= !(mask as u8);
        }
    }
}
