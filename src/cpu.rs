use std::path;

use crate::mmu::Mmu;
use crate::regs::*;

pub struct Cpu {
    regs: Registers,
    mode_flags: ModeChangeFlags,

    interrupts: bool,
    cycles: u8,

    mmu: Mmu,
}

struct ModeChangeFlags {
    change_intrpt_mode_on_next_instr: bool,
}

impl Cpu {
    pub fn new(path: &path::Path) -> Self {
        Cpu {
            regs: Registers::new(),
            mode_flags: ModeChangeFlags::new(),

            interrupts: true,
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

        let cycles = match instr {
            //NOP
            0x00 => {
                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{}",
                    addr, instr, 4, "NOP"
                );
                4
            }

            //INC B
            0x04 => {
                self.regs.B = self.inc_u8(self.regs.B);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {}",
                    addr, instr, 4, "INC", "B"
                );
                4
            }

            //DEC B
            0x05 => {
                self.regs.B = self.dec_u8(self.regs.B);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {}",
                    addr, instr, 4, "DEC", "B"
                );
                4
            }

            //LD B u8
            0x06 => {
                let data = self.get_imu8();
                self.regs.B = data;

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:X}",
                    addr, instr, 8, "LD", "B,", data
                );
                8
            }

            //INC C
            0x0C => {
                self.regs.C = self.inc_u8(self.regs.C);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {}",
                    addr, instr, 4, "INC", "C"
                );
                4
            }

            //DEC D
            0x0D => {
                self.regs.C = self.dec_u8(self.regs.C);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {}",
                    addr, instr, 4, "DEC", "C"
                );
                4
            }

            //LD C u8
            0x0E => {
                let data = self.get_imu8();
                self.regs.C = data;

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:X}",
                    addr, instr, 8, "LD", "C,", data
                );
                8
            }

            //LD A 0xFF00 + u8
            0xF0 => {
                let offset = self.get_imu8();
                self.regs.A = self.mmu.read(0xFF00 + offset as u16);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:X}",
                    addr, instr, 12, "LDH", "A,", offset
                );
                12
            }

            //LD DE imu16
            0x11 => {
                let word = self.get_imu16();
                self.regs.set_DE(word);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:X}",
                    addr, instr, 12, "LD", "DE,", word
                );
                12
            }

            //INC HL
            0x13 => {
                let DE = self.regs.DE() + 1;
                self.regs.set_DE(DE);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {}",
                    addr, instr, 8, "INC", "DE"
                );
                8
            }

            //RLA
            0x17 => {
                self.regs.A = self.rl(self.regs.A);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{}",
                    addr, instr, 4, "RLA"
                );
                4
            }

            //JR
            0x18 => {
                let offset = self.get_imu8() as i8;
                let jmp_addr = ((self.regs.PC as i32) + offset as i32) as u16;
                self.regs.PC = jmp_addr;

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:04X}",
                    addr, instr, 8, "JR", "NZ", jmp_addr
                );
                8
            }

            //LD A DE
            0x1A => {
                self.regs.A = self.mmu.read(self.regs.DE());

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} {}",
                    addr, instr, 8, "LD", "A,", "DE"
                );
                8
            }

            //LD E u8
            0x1E => {
                let data = self.get_imu8();
                self.regs.E = data;

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:X}",
                    addr, instr, 8, "LD", "E,", data
                );
                8
            }

            //JR NZ
            0x20 => {
                let offset = self.get_imu8() as i8;
                let jmp_addr = ((self.regs.PC as i32) + offset as i32) as u16;
                if !self.regs.get_flag(FlagsMasks::Z) {
                    self.regs.PC = jmp_addr;
                }

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:04X}",
                    addr, instr, 8, "JR", "NZ", jmp_addr
                );
                8
            }

            //LD HL u16
            0x21 => {
                let data = self.get_imu16();
                self.regs.set_HL(data);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:X}",
                    addr, instr, 12, "LD", "HL,", data
                );
                12
            }

            //LD (HL+) A
            0x22 => {
                let addr_write = self.regs.HL();
                self.mmu.write(addr_write, self.regs.A);
                self.regs.set_HL(addr_write + 1);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} {}",
                    addr, instr, 8, "LD", "(HL+),", "A"
                );
                8
            }

            //INC HL
            0x23 => {
                let HL = self.regs.HL() + 1;
                self.regs.set_HL(HL);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {}",
                    addr, instr, 8, "INC", "HL"
                );
                8
            }

            //JR Z, u16
            0x28 => {
                let offset = self.get_imu8() as i8;
                let jmp_addr = ((self.regs.PC as i32) + offset as i32) as u16;

                if self.regs.get_flag(FlagsMasks::Z) {
                    self.regs.PC = jmp_addr;
                }

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:X}",
                    addr, instr, 8, "JR", "Z", offset
                );
                8
            }

            //LD L imu8
            0x2E => {
                self.regs.L = self.get_imu8();

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:X}",
                    addr, instr, 8, "LD", "L,", self.regs.L
                );
                8
            }

            //LD SP u16
            0x31 => {
                self.regs.SP = self.get_imu16();

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:X}",
                    addr, instr, 12, "LD", "SP,", self.regs.SP
                );
                12
            }

            //LDD HL u16
            0x32 => {
                let addr_write = self.regs.HL();
                self.mmu.write(addr_write, self.regs.A);
                self.regs.set_HL(addr_write - 1);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} 0x{:04X}, {}",
                    addr, instr, 8, "LDD", addr_write, "A"
                );
                8
            }

            //DEC A
            0x3D => {
                self.regs.A = self.dec_u8(self.regs.A);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {}",
                    addr, instr, 4, "DEC", "A"
                );
                4
            }

            //LD A imu8
            0x3E => {
                self.regs.A = self.get_imu8();

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} 0x{:X}",
                    addr, instr, 8, "LD", "A,", self.regs.A
                );
                8
            }

            //LD C A
            0x4F => {
                self.regs.C = self.regs.A;

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} {}",
                    addr, instr, 4, "LD", "C,", "A"
                );
                4
            }

            //LD D A
            0x57 => {
                self.regs.D = self.regs.A;

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} {}",
                    addr, instr, 4, "LD", "D,", "A"
                );
                4
            }

            //LD H A
            0x67 => {
                self.regs.H = self.regs.A;

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} {}",
                    addr, instr, 4, "LD", "H,", "A"
                );
                4
            }

            //LD (HL), A
            0x77 => {
                self.mmu.write(self.regs.HL(), self.regs.A);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} {}",
                    addr, instr, 8, "LD", "(HL),", "A"
                );
                8
            }

            //LD A C
            0x7B => {
                self.regs.A = self.regs.E;

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} {}",
                    addr, instr, 4, "LD", "A,", "E"
                );
                4
            }

            //XOR A
            0xAF => {
                self.xor(self.regs.A);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {}",
                    addr, instr, 4, "XOR", "A"
                );
                4
            }

            //POP BC
            0xC1 => {
                let word = self.stack_pop_u16();
                self.regs.set_BC(word);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {}",
                    addr, instr, 12, "POP", "BC"
                );
                12
            }

            //JMP u16
            0xC3 => {
                self.regs.PC = self.get_imu16();

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} 0x{:04X}",
                    addr, instr, 16, "JMP", self.regs.PC
                );
                16
            }

            //PUSH BC
            0xC5 => {
                self.stack_push_u16(self.regs.BC());
                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {}",
                    addr, instr, 16, "PUSH", "BC"
                );
                16
            }

            //RET
            0xC9 => {
                self.ret();

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{}",
                    addr, instr, 8, "RET"
                );
                8
            }

            //CB Prefix
            0xCB => {
                let instr = self.get_imu8();
                self.exec_prefix_instr(instr) + 4
            }

            //Call
            0xCD => {
                self.call();

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} 0x{:04X}",
                    addr, instr, 12, "CALL", self.regs.PC
                );
                12
            }

            //RST 0x18
            0xDF => {
                self.rst(0x18);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} 0x{:X}",
                    addr, instr, 32, "RST", 0x18
                );
                32
            }

            //LD (n), A
            0xE0 => {
                let byte = self.get_imu8();
                self.mmu.write(0xFF00 + byte as u16, self.regs.A);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} ({} 0x{:X}), {}",
                    addr, instr, 12, "LD", "0xFF00 + ", byte, "A"
                );
                12
            }

            //LD (C), A
            0xE2 => {
                self.mmu.write(0xFF00 + self.regs.C as u16, self.regs.A);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {} {}",
                    addr, instr, 8, "LD", "(0xFF00 + C),", "A"
                );
                8
            }

            //LD (imu16) A
            0xEA => {
                let addr_write = self.get_imu16();
                self.mmu.write(addr_write, self.regs.A);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} (0x{:X}), {}",
                    addr, instr, 16, "LD", addr_write, "A"
                );
                16
            }
            //DI
            0xF3 => {
                //Mode flag set at end of func

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{}",
                    addr, instr, 4, "DI"
                );
                4
            }

            //CP A #
            0xFE => {
                let byte = self.get_imu8();
                self.sub(self.regs.A, byte);

                println!(
                    "Addr:0x{:04X}\t\tOp:0x{:X}\t\tTime:{}\t\t{} {}, 0x{:X}",
                    addr, instr, 8, "CP", "A", byte
                );
                8
            }

            //RST 0x36
            0xFF => {
                self.rst(0x36);

                println!(
                    "Addr:0x{:04X}\t\tOp:{}\t\tTime:{}\t\t{} 0x{:X}",
                    addr, instr, 32, "RST", 0x36
                );
                32
            }

            _ => {
                self.panic_dump(instr, false);

                0
            }
        };

        if instr == 0xF3 {
            self.mode_flags.change_intrpt_mode_on_next_instr = true;
        } else if self.mode_flags.change_intrpt_mode_on_next_instr {
            self.interrupts = !self.interrupts;
        }

        cycles
    }

    fn exec_prefix_instr(&mut self, instr: u8) -> u8 {
        let addr = self.regs.PC - 2;
        match instr {
            //RL C
            0x11 => {
                self.regs.C = self.rl(self.regs.C);
                println!(
                    "Addr:0x{:04X}\t\tOp:CB {}\tTime:{}\t\t{} {}",
                    addr, instr, 8, "RL", "C"
                );
                8
            }

            //BIT 7,h
            0x7C => {
                self.bit(self.regs.H, 7);

                println!(
                    "Addr:0x{:04X}\t\tOp:CB {:X}\tTime:{}\t\t{} {}, {}",
                    addr,
                    instr,
                    8 + 4,
                    "BIT",
                    7,
                    "H"
                );
                8
            }
            _ => {
                self.panic_dump(instr, true);

                0
            }
        }
    }

    fn stack_push_u16(&mut self, data: u16) {
        self.regs.SP -= 2;
        self.mmu.write_u16(self.regs.SP, data);
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let word = self.mmu.read_u16(self.regs.SP);
        self.regs.SP += 2;

        word
    }

    fn get_imu8(&mut self) -> u8 {
        let val: u8 = self.mmu.read(self.regs.PC);
        self.regs.inc_PC();
        val
    }

    fn get_imu16(&mut self) -> u16 {
        let mut val: u16 = self.get_imu8() as u16;
        val |= (self.get_imu8() as u16) << 8;
        val
    }
}

//ALU
impl Cpu {
    fn xor(&mut self, reg: u8) {
        self.regs.A = reg ^ self.regs.A;

        if self.regs.A == 0 {
            self.regs.set_flag(FlagsMasks::Z, true);
        }
    }

    fn dec_u8(&mut self, reg: u8) -> u8 {
        let res = reg.overflowing_sub(1).0;

        self.regs.set_flag(FlagsMasks::N, true);
        if res == 0 {
            self.regs.set_flag(FlagsMasks::Z, true);
        } else {
            self.regs.set_flag(FlagsMasks::Z, false);
        }

        if res & 0xF0 == 0 {
            self.regs.set_flag(FlagsMasks::H, true);
        } else {
            self.regs.set_flag(FlagsMasks::H, false);
        }

        res
    }

    fn inc_u8(&mut self, reg: u8) -> u8 {
        let res = reg.overflowing_add(1).0;

        self.regs.set_flag(FlagsMasks::N, false);
        if res == 0 {
            self.regs.set_flag(FlagsMasks::Z, true);
        }

        if res & 0xF8 == 0 {
            self.regs.set_flag(FlagsMasks::H, true);
        } else {
            self.regs.set_flag(FlagsMasks::H, false);
        }

        res
    }

    fn bit(&mut self, regs: u8, nb_bit: u8) {
        let mask = 1 << nb_bit;
        let bit = (regs & mask) > 0;

        self.regs.set_flag(FlagsMasks::N, false);
        self.regs.set_flag(FlagsMasks::H, true);
        if bit {
            self.regs.set_flag(FlagsMasks::Z, false);
        } else {
            self.regs.set_flag(FlagsMasks::Z, true);
        }
    }

    //TODO fix rotation, not shift
    fn rl(&mut self, reg: u8) -> u8 {
        self.regs.set_flag(FlagsMasks::C, reg & 0b10000000 > 0);
        self.regs.set_flag(FlagsMasks::N, false);
        self.regs.set_flag(FlagsMasks::H, false);

        let reg = reg << 1;
        if reg != 0 {
            self.regs.set_flag(FlagsMasks::Z, false);
        } else {
            self.regs.set_flag(FlagsMasks::Z, true);
        }

        reg
    }

    fn sub(&mut self, num1: u8, num2: u8) -> u8 {
        let res = num1.overflowing_sub(num2);

        self.regs.set_flag(FlagsMasks::N, true);
        if res.0 == 0 {
            self.regs.set_flag(FlagsMasks::Z, true);
        } else {
            self.regs.set_flag(FlagsMasks::Z, false);
        }

        if res.0 & 0xF0 == 0 {
            self.regs.set_flag(FlagsMasks::H, true);
        } else {
            self.regs.set_flag(FlagsMasks::H, false);
        }

        if res.1 {
            self.regs.set_flag(FlagsMasks::C, true);
        } else {
            self.regs.set_flag(FlagsMasks::C, false);
        }

        res.0
    }
}

//JMP
impl Cpu {
    fn rst(&mut self, offset: u8) {
        self.stack_push_u16(self.regs.PC);
        self.regs.PC = offset as u16;
    }

    fn ret(&mut self) {
        self.regs.PC = self.stack_pop_u16();
    }

    fn call(&mut self) {
        let addr = self.get_imu16();
        self.stack_push_u16(self.regs.PC + 2);
        self.regs.PC = addr;
    }
}

//Debug
impl Cpu {
    fn panic_dump(&self, instr: u8, prefix: bool) {
        println!();
        if prefix {
            println!(
                "Addr: 0x{:04X}\t\t Opcode CB {:02X} not implemented",
                self.regs.PC - 1,
                instr
            );
        } else {
            println!(
                "Addr: 0x{:04X}\t\t Opcode 0x{:02X} not implemented",
                self.regs.PC - 1,
                instr
            );
        }
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

impl ModeChangeFlags {
    fn new() -> Self {
        ModeChangeFlags {
            change_intrpt_mode_on_next_instr: false,
        }
    }
}
