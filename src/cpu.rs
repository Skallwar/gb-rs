use std::path;

use crate::mmu::Mmu;

pub struct Cpu {
    regs: Registers,
    cycles: u8,

    mmu: Mmu,
}

#[allow(non_snake_case)]
pub struct Registers {
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
            cycles: 0,

            mmu: Mmu::new(path),
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.cycles != 0 {
                self.cycles -= 1;
            }

            let instr = self.mmu.read(self.regs.PC);
            self.regs.inc_PC();
            self.cycles = self.exec_instr(instr);
        }
    }

    fn exec_instr(&mut self, instr: u8) -> u8 {
        let addr = self.regs.PC - 1;
        match instr {
            0x00 => {
                println!(
                    "Addr:0x{:04X}\tOp:0x{:X}\tTime:{}\t{}",
                    addr, instr, 4, "JMP"
                );
                4
            }

            0xAF => {
                self.xor(self.regs.A);

                println!(
                    "Addr:0x{:04X}\tOp:0x{:X}\tTime:{}\t{} {}",
                    addr, instr, 4, "XOR", "A"
                );
                4
            }

            0xC3 => {
                self.regs.PC = self.get_imu16();
                println!(
                    "Addr:{:04X}\tOp:{}\tTime:{}\t{} 0x{:04X}",
                    addr, instr, 16, "JMP", self.regs.PC
                );

                16
            }

            _ => {
                self.panic_dump(instr);

                0
            }
        }
    }

    fn get_imu8(&mut self) -> u8 {
        let val: u8 = self.mmu.read(self.regs.PC);
        self.regs.inc_PC();
        val
    }

    fn get_imu16(&mut self) -> u16 {
        let mut val: u16 = self.get_imu8() as u16;
        val |= (self.mmu.read(self.regs.PC) as u16) << 8;
        val
    }

    fn panic_dump(&self, instr: u8) {
        println!();
        println!(
            "Addr: 0x{:04X}\t Opcode 0x{:X} not implemented",
            self.regs.PC - 1,
            instr
        );
        println!("Register dump:");
        println!("-AF: 0x{:04X}", self.regs.AF());
        println!("-BC: 0x{:04X}", self.regs.BC());
        println!("-DE: 0x{:04X}", self.regs.DE());
        println!("-HL: 0x{:04X}", self.regs.HL());
        println!("-SP: 0x{:04X}", self.regs.SP);
        println!("-PC: 0x{:04X}", self.regs.PC);
        println!();
        panic!();
    }
}

//ALU
impl Cpu {
    fn xor(&mut self, reg: u8) {
        self.regs.A = reg ^ self.regs.A;

        if self.regs.A == 0 {
            self.regs.set_flag_Z(true);
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

    //Getters
    fn AF(&self) -> u16 {
        //AF returns only A
        (self.A as u16) << 8
    }

    fn BC(&self) -> u16 {
        ((self.B as u16) << 8) + self.C as u16
    }

    fn DE(&self) -> u16 {
        ((self.D as u16) << 8) + self.E as u16
    }

    fn HL(&self) -> u16 {
        ((self.H as u16) << 8) + self.L as u16
    }

    //Setters
    fn set_AF(&mut self, data: u16) {
        //AF contains only A
        self.A = (data >> 8) as u8;
    }

    fn set_BC(&mut self, data: u16) {
        self.B = (data >> 8) as u8;
        self.C = (data & 0x00FF) as u8;
    }

    fn set_DE(&mut self, data: u16) {
        self.D = (data >> 8) as u8;
        self.E = (data & 0x00FF) as u8;
    }

    fn set_HL(&mut self, data: u16) {
        self.H = (data >> 8) as u8;
        self.L = (data & 0x00FF) as u8;
    }

    //Flags
    fn flag_Z(&self) -> bool {
        if self.F & 0b10000000 != 0 {
            true
        } else {
            false
        }
    }

    fn flag_N(&self) -> bool {
        if self.F & 0b01000000 != 0 {
            true
        } else {
            false
        }
    }

    fn flag_H(&self) -> bool {
        if self.F & 0b00100000 != 0 {
            true
        } else {
            false
        }
    }

    fn flag_C(&self) -> bool {
        if self.F & 0b00010000 != 0 {
            true
        } else {
            false
        }
    }

    fn set_flag_Z(&mut self, val: bool) {
        if val {
            self.F |= 0b10000000;
        } else {
            self.F &= 0b01111111;
        }
    }

    fn set_flag_N(&mut self, val: bool) {
        if val {
            self.F |= 0b01000000;
        } else {
            self.F &= 0b10111111;
        }
    }

    fn set_flag_H(&mut self, val: bool) {
        if val {
            self.F |= 0b00100000;
        } else {
            self.F &= 0b11011111;
        }
    }

    fn set_flag_C(&mut self, val: bool) {
        if val {
            self.F |= 0b00010000;
        } else {
            self.F &= 0b11101111;
        }
    }
    fn inc_PC(&mut self) {
        self.PC += 1;
    }
}
