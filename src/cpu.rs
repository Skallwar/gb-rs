use std::path;

use crate::mmu::Mmu;

pub struct Cpu {
    regs: Registers,

    mmu: Mmu,
}

#[allow(non_snake_case)]
struct Registers {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    H: u8,
    L: u8,
    F: u8,
    SP: u16,
    PC: u16,
}

impl Cpu {
    pub fn new(path: &path::Path) -> Self {
        Cpu {
            regs: Registers::new(),

            mmu: Mmu::new(path),
        }
    }
}

#[allow(non_snake_case)]
impl Registers {
    fn new() -> Self {
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
            PC: 0x0100,
        }
    }

    fn A(self) -> u8 {
        self.A
    }

    fn B(self) -> u8 {
        self.B
    }

    fn C(self) -> u8 {
        self.C
    }

    fn D(self) -> u8 {
        self.D
    }

    fn E(self) -> u8 {
        self.E
    }

    fn H(self) -> u8 {
        self.H
    }

    fn L(self) -> u8 {
        self.L
    }

    fn F(self) -> u8 {
        self.F
    }

    fn AF(self) -> u16 {
        (self.A as u16) << 8
    }

    fn BC(self) -> u16 {
        (self.B as u16) << 8 + self.C
    }

    fn DE(self) -> u16 {
        (self.D as u16) << 8 + self.E
    }

    fn HL(self) -> u16 {
        (self.H as u16) << 8 + self.L
    }

    fn SP(self) -> u16 {
        self.SP
    }

    fn PC(self) -> u16 {
        self.PC
    }

    fn set_A(mut self, data: u8) {
        self.A = data;
    }

    fn set_B(mut self, data: u8) {
        self.B = data;
    }

    fn set_C(mut self, data: u8) {
        self.C = data;
    }

    fn set_D(mut self, data: u8) {
        self.D = data;
    }

    fn set_E(mut self, data: u8) {
        self.E = data;
    }

    fn set_H(mut self, data: u8) {
        self.H = data;
    }

    fn set_L(mut self, data: u8) {
        self.L = data;
    }

    fn set_AF(mut self, data: u16) {
        self.A = (data >> 8) as u8;
    }

    fn set_BC(mut self, data: u16) {
        self.B = (data >> 8) as u8;
        self.C = (data & 0x00FF) as u8;
    }

    fn set_DE(mut self, data: u16) {
        self.D = (data >> 8) as u8;
        self.E = (data & 0x00FF) as u8;
    }

    fn set_HL(mut self, data: u16) {
        self.H = (data >> 8) as u8;
        self.L = (data & 0x00FF) as u8;
    }

    fn set_SP(mut self, data: u16) {
        self.SP = data;
    }

    fn set_PC(mut self, data: u16) {
        self.PC = data;
    }

    fn flag_Z(self) -> bool {
        if self.F & 0b10000000 != 0 {
            true
        } else {
            false
        }
    }

    fn flag_N(self) -> bool {
        if self.F & 0b01000000 != 0 {
            true
        } else {
            false
        }
    }

    fn flag_H(self) -> bool {
        if self.F & 0b00100000 != 0 {
            true
        } else {
            false
        }
    }

    fn flag_C(self) -> bool {
        if self.F & 0b00010000 != 0 {
            true
        } else {
            false
        }
    }
}
