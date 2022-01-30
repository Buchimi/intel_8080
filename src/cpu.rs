use crate::registers::Regs;

// This allows us to print to terminal. 
#[derive(Debug)]
pub struct CPU {
    ram: [u8; 65536],
    pc: u16,
    sp: u16,
    regs: Regs
} 

impl CPU {
    pub fn init_cpu() -> Self {
        CPU {
            ram: [0; 65536],
            pc: 0,
            sp: 0,
            regs: Regs::init_regs()
        }
    }

    fn fetch(&mut self) -> u8 {
        let byte = self.ram[self.pc as usize];
        self.pc += 1;
        byte
    }

    pub fn tick(&mut self) {
        let inst = self.fetch();

        self.decode(inst);
    }

    fn decode(&mut self, inst: u8) {
        match inst {
            0x00 => {self.nop()},
            0x01 => {self.lxi("B")},
            0x02 => {self.stax("B")},
            0x03 => {self.inx("B")},
            0x04 => {self.inr("B")},
            0x05 => {self.dcr("B")},
            0x06 => {self.mvi("B")},
            0x07 => {self.rlc()},
            0x09 => {self.dad("B")},
            0x0A => {self.ldax("B")},
            0x0B => {self.dcx("B")},
            0x0C => {self.inr("C")},
            0x0D => {self.dcr("C")},
            0x0E => {self.mvi("C")},
            0x0F => {self.rrc()},
            
            0x11 => {self.lxi("D")},
            0x12 => {self.stax("D")},
            0x13 => {self.inx("D")},
            0x14 => {self.inr("D")},
            0x15 => {self.dcr("D")},
            0x16 => {self.mvi("D")},
            0x17 => {self.ral()},
            0x19 => {self.dad("D")},
            0x1A => {self.ldax("D")},
            0x1B => {self.dcx("D")},
            0x1C => {self.inr("E")},
            0x1D => {self.dcr("E")},
            0x1E => {self.mvi("E")},
            0x1F => {self.rar()},

            0x21 => {self.lxi("H")},
            0x22 => {self.shld()},
            0x23 => {self.inx("H")},
            0x24 => {self.inr("H")},
            0x25 => {self.dcr("H")},
            0x26 => {self.mvi("H")},
            0x27 => {self.daa()},
            0x29 => {self.dad("H")},
            0x2A => {self.lhld()},
            0x2B => {self.dcx("H")},
            0x2C => {self.inr("L")},
            0x2D => {self.dcr("L")},
            0x2E => {self.mvi("L")},
            0x2F => {self.cma()},

            0x31 => {self.lxi("SP")},
            0x32 => {self.sta()},
            0x33 => {self.inx("SP")},
            0x34 => {self.inr("M")},
            0x35 => {self.dcr("M")},
            0x36 => {self.mvi("M")},
            0x37 => {self.stc()},
            0x39 => {self.dad("SP")},
            0x3A => {self.lda()},
            0x3B => {self.dcx("SP")},
            0x3C => {self.inr("A")},
            0x3D => {self.dcr("A")},
            0x3E => {self.mvi("A")},
            0x3F => {self.cmc()},

            0x40 => {self.mov("B", "B")},
            0x41 => {self.mov("B", "C")},
            0x42 => {self.mov("B", "D")},
            0x43 => {self.mov("B", "E")},
            0x44 => {self.mov("B", "H")},
            0x45 => {self.mov("B", "L")},
            0x46 => {self.mov("B", "M")},
            0x47 => {self.mov("B", "A")},
            0x48 => {self.mov("C", "B")},
            0x49 => {self.mov("C", "C")},
            0x4A => {self.mov("C", "D")},
            0x4B => {self.mov("C", "E")},
            0x4C => {self.mov("C", "H")},
            0x4D => {self.mov("C", "L")},
            0x4E => {self.mov("C", "M")},
            0x4F => {self.mov("C", "A")},
           
            0x50 => {self.mov("D", "B")},
            0x51 => {self.mov("D", "C")},
            0x52 => {self.mov("D", "D")},
            0x53 => {self.mov("D", "E")},
            0x54 => {self.mov("D", "H")},
            0x55 => {self.mov("D", "L")},
            0x56 => {self.mov("D", "M")},
            0x57 => {self.mov("D", "A")},
            0x58 => {self.mov("E", "B")},
            0x59 => {self.mov("E", "C")},
            0x5A => {self.mov("E", "D")},
            0x5B => {self.mov("E", "E")},
            0x5C => {self.mov("E", "H")},
            0x5D => {self.mov("E", "L")},
            0x5E => {self.mov("E", "M")},
            0x5F => {self.mov("E", "A")},

            0x60 => {self.mov("H", "B")},
            0x61 => {self.mov("H", "C")},
            0x62 => {self.mov("H", "D")},
            0x63 => {self.mov("H", "E")},
            0x64 => {self.mov("H", "H")},
            0x65 => {self.mov("H", "L")},
            0x66 => {self.mov("H", "M")},
            0x67 => {self.mov("H", "A")},
            0x68 => {self.mov("L", "B")},
            0x69 => {self.mov("L", "C")},
            0x6A => {self.mov("L", "D")},
            0x6B => {self.mov("L", "E")},
            0x6C => {self.mov("L", "H")},
            0x6D => {self.mov("L", "L")},
            0x6E => {self.mov("L", "M")},
            0x6F => {self.mov("L", "A")},

            0x70 => {self.mov("M", "B")},
            0x71 => {self.mov("M", "C")},
            0x72 => {self.mov("M", "D")},
            0x73 => {self.mov("M", "E")},
            0x74 => {self.mov("M", "H")},
            0x75 => {self.mov("M", "L")},
            0x76 => {self.hlt()}
            0x77 => {self.mov("M", "A")},
            0x78 => {self.mov("A", "B")},
            0x79 => {self.mov("A", "C")},
            0x7A => {self.mov("A", "D")},
            0x7B => {self.mov("A", "E")},
            0x7C => {self.mov("A", "H")},
            0x7D => {self.mov("A", "L")},
            0x7E => {self.mov("A", "M")},
            0x7F => {self.mov("A", "A")},

            0x80 => {self.addc()},
            0x81 => {self.addc()},
            0x82 => {self.addc()},
            0x83 => {self.addc()},
            0x84 => {self.addc()},
            0x85 => {self.addc()},
            0x86 => {self.addc()},
            0x87 => {self.addc()},
            0x88 => {self.addc()},
            0x89 => {self.addc()},
            0x8A => {self.addc()},
            0x8B => {self.addc()},
            0x8C => {self.addc()},
            0x8D => {self.addc()},
            0x8E => {self.addc()},
            0x8F => {self.addc()},

            0x90 => {self.sub()},
            0x91 => {self.sub()},
            0x92 => {self.sub()},
            0x93 => {self.sub()},
            0x94 => {self.sub()},
            0x95 => {self.sub()},
            0x96 => {self.sub()},
            0x97 => {self.sub()},
            0x98 => {self.sbb()},
            0x99 => {self.sbb()},
            0x9A => {self.sbb()},
            0x9B => {self.sbb()},
            0x9C => {self.sbb()},
            0x9D => {self.sbb()},
            0x9E => {self.sbb()},
            0x9F => {self.sbb()},

            0xA0 => {self.ana()},
            0xA1 => {self.ana()},
            0xA2 => {self.ana()},
            0xA3 => {self.ana()},
            0xA4 => {self.ana()},
            0xA5 => {self.ana()},
            0xA6 => {self.ana()},
            0xA7 => {self.ana()},
            0xA8 => {self.xra()},
            0xA9 => {self.xra()},
            0xAA => {self.xra()},
            0xAB => {self.xra()},
            0xAC => {self.xra()},
            0xAD => {self.xra()},
            0xAE => {self.xra()},
            0xAF => {self.xra()},

            0xB0 => {self.ora()},
            0xB1 => {self.ora()},
            0xB2 => {self.ora()},
            0xB3 => {self.ora()},
            0xB4 => {self.ora()},
            0xB5 => {self.ora()},
            0xB6 => {self.ora()},
            0xB7 => {self.ora()},
            0xB8 => {self.cmp()},
            0xB9 => {self.cmp()},
            0xBA => {self.cmp()},
            0xBB => {self.cmp()},
            0xBC => {self.cmp()},
            0xBD => {self.cmp()},
            0xBE => {self.cmp()},
            0xBF => {self.cmp()},

            0xC0 => {self.rnz()},
            0xC1 => {self.pop()},
            0xC2 => {self.jnz()},
            0xC3 => {self.jmp()},
            0xC4 => {self.cnz()},
            0xC5 => {self.push()},
            0xC6 => {self.adi()},
            0xC7 => {self.rst()},
            0xC8 => {self.rz()},
            0xC9 => {self.ret()},
            0xCA => {self.jz()},
            0xCC => {self.cz()},
            0xCD => {self.call()},
            0xCE => {self.aci()},
            0xCF => {self.rst()},

            0xD0 => {self.rnc()},
            0xD1 => {self.pop()},
            0xD2 => {self.jnc()},
            0xD3 => {self.out()},
            0xD4 => {self.cnc()},
            0xD5 => {self.push()},
            0xD6 => {self.sui()},
            0xD7 => {self.rst()},
            0xD8 => {self.rc()},
            0xDA => {self.jc()},
            0xDB => {self.ind()},
            0xDC => {self.cc()},
            0xDE => {self.sbi()},
            0xDF => {self.rst()},

            0xE0 => {self.rpo()},
            0xE1 => {self.pop()},
            0xE2 => {self.jpo()},
            0xE3 => {self.xthl()},
            0xE4 => {self.cpo()},
            0xE5 => {self.push()},
            0xE6 => {self.ani()},
            0xE7 => {self.rst()},
            0xE8 => {self.rpe()},
            0xE9 => {self.pchl()},
            0xEA => {self.jpe()},
            0xEB => {self.xchg()},
            0xEC => {self.cpe()},
            0xEE => {self.xri()},
            0xEF => {self.rst()},

            0xF0 => {self.rp()},
            0xF1 => {self.pop()},
            0xF2 => {self.jp()},
            0xF3 => {self.di()},
            0xF4 => {self.cp()},
            0xF5 => {self.push()},
            0xF6 => {self.ori()},
            0xF7 => {self.rst()},
            0xF8 => {self.rm()},
            0xF9 => {self.sphl()},
            0xFA => {self.jm()},
            0xFB => {self.ei()},
            0xFC => {self.cm()},
            0xFE => {self.cpi()},
            0xFF => {self.rst()},

            _ => {}
        };
    }

    fn nop(&mut self) {

    }

    // Load the immediate 16 bit value to reg pair 
    // Reg C == byte 2
    // Reg B == Byte 3
    fn lxi(&mut self, reg: &str){
        match reg {
            "B" => {   
                self.regs.C = self.fetch();
                self.regs.B = self.fetch();
            },
            "D" => {
                self.regs.D = self.fetch();
                self.regs.E = self.fetch();
            },
            "H" => {
                self.regs.H = self.fetch();
                self.regs.L = self.fetch();
            },
            "SP" => {
                let lo = self.fetch();
                let hi = self.fetch();
                let result = (hi as u16) << 8 | lo as u16;
                self.sp = result;
            }
            _ => {}
        };
    }

    // Load value of reg A into location returned by reg pair
    fn stax(&mut self, reg: &str) {
        let location = if reg == "B" {
            self.regs.return_joined_regs("BC")
        } else if reg == "D" {
            self.regs.return_joined_regs("DE")
        }  else if reg == "H" {
            self.regs.return_joined_regs("HL")
        } else {
            0
        };

        self.ram[location as usize] = self.regs.A;
    }

    // Increment reg pair by 1
    fn inx(&mut self, reg: &str) {
        if reg != "SP" {
        let mut reg_pair = if reg == "B" {
            self.regs.return_joined_regs("BC")
        } else if reg == "D" {
            self.regs.return_joined_regs("DE")
        }  else if reg == "H" {
            self.regs.return_joined_regs("HL")
        } else {
            0
        };

        reg_pair += 1;

        // Convert register pair into tuple for easier splitting
        let reg_pair_tup = self.regs.split_regs(reg_pair);

        self.regs.B = reg_pair_tup.0;
        self.regs.B = reg_pair_tup.1;
    } else {
        self.sp += 1
    }
    }

    // Increment register
    fn inr(&mut self, reg: &str) {
        let addr = self.return_byte_at_location("HL");
        match reg {
            "B" => {self.regs.B += 1},
            "C" => {self.regs.C += 1},
            "D" => {self.regs.D += 1},
            "E" => {self.regs.E += 1},
            "H" => {self.regs.H += 1},
            "L" => {self.regs.L += 1},
            "M" => {self.ram[addr as usize] += 1}
            _ => {}
        };
    }

    // Decrement register
    fn dcr(&mut self, reg: &str) {
        let addr = self.return_byte_at_location("HL");
        match reg {
            "B" => {self.regs.B -= 1},
            "C" => {self.regs.C -= 1},
            "D" => {self.regs.D -= 1},
            "E" => {self.regs.E -= 1},
            "H" => {self.regs.H -= 1},
            "L" => {self.regs.L -= 1},
            "M" => {self.ram[addr as usize] += 1},
            _ => {}
        };
    }

    // Reg B = next byte fetched
    fn mvi(&mut self, reg: &str) {
        let byte = self.fetch();
        let addr = self.return_byte_at_location("HL");

        match reg {
            "B" => {self.regs.B = byte},
            "C" => {self.regs.C = byte},
            "D" => {self.regs.D = byte},
            "E" => {self.regs.E = byte},
            "H" => {self.regs.H = byte},
            "L" => {self.regs.L = byte},
            "M" => {self.ram[addr as usize] = byte},
            _ => {}
        }
    }

    // Rotate A left
    fn rlc(&mut self) {
        self.regs.A = self.regs.A.rotate_left(7);
    }

    // Register pair is added to HL
    fn dad(&mut self, reg: &str) {
        let mut hl = self.regs.return_joined_regs("HL");
        let reg_to_add_to_hl = 
        match reg {
            "B" => {
                self.regs.return_joined_regs("BC")
                }
            "D" => {
                self.regs.return_joined_regs("DE")
            }, 
            "H" => {
                self.regs.return_joined_regs("HL")
            },
            _ => 0
    };
        match reg {
            "B" => {
                hl += reg_to_add_to_hl;
                self.regs.H = self.regs.split_regs(hl).0;
                self.regs.L = self.regs.split_regs(hl).0;
                }
            "D" => {
                hl += reg_to_add_to_hl;
                self.regs.H = self.regs.split_regs(hl).0;
                self.regs.L = self.regs.split_regs(hl).0; }, 
            "H" => {
                hl += reg_to_add_to_hl;
                self.regs.H = self.regs.split_regs(hl).0;
                self.regs.L = self.regs.split_regs(hl).0;},
            "SP" => {
                self.sp += reg_to_add_to_hl;
            }
            _ => ()
        }
    }

    // Reg A = location indexed by reg pair
    fn ldax(&mut self, reg: &str) {
        let mem_index = 
        match reg {
            "B" => {
                self.regs.return_joined_regs("BC")
                }
            "D" => {
                self.regs.return_joined_regs("DE")
            }, 
            "H" => {
                self.regs.return_joined_regs("HL")
            },
            _ => 0
    };

        self.regs.A = self.ram[mem_index as usize];

    }

    // Decrement reg pair
    fn dcx(&mut self, reg: &str) {
        if reg != "SP" {
        let mut reg_pair = if reg == "B" {
            self.regs.return_joined_regs("BC")
        } else if reg == "D" {
            self.regs.return_joined_regs("DE")
        }  else if reg == "H" {
            self.regs.return_joined_regs("HL")
        } else {
            0
        };

        reg_pair -= 1;

        // Convert register pair into tuple for easier splitting
        let reg_pair_tup = self.regs.split_regs(reg_pair);

        self.regs.B = reg_pair_tup.0;
        self.regs.B = reg_pair_tup.1;
    } else {
        self.sp -= 1;
    }
    }


    // Rotate reg A right
    fn rrc(&mut self) {
        self.regs.A = self.regs.A.rotate_right(7);
    }

    // Rotate reg A left with carry
    fn ral(&mut self) {

    }

    // Rotate reg A right with carry
    fn rar(&mut self) {

    }

    // Store HL at immediate address
    // ADDR = L
    // ADDR + 1 = H
    fn shld(&mut self) {
        let addr = self.fetch();
        let addr1 = self.fetch();

        self.ram[addr as usize] = self.regs.L;
        self.ram[addr1 as usize] = self.regs.H;

    }

    // Reg A is converted to 2 nibbles in BCD
    fn daa(&mut self) {
        let hi_nibble = self.regs.A >> 4;
        let lo_nibble = self.regs.A & 0xF;

        // Convert both nibbles to BCD. Join, then set register to the new number.

        // I think
    }

    // Load reg pair HL from imm address
    // Reg L = addr
    // Reg H = addr + 1
    fn lhld(&mut self) {
        let addr = self.fetch();
        let addr1 = self.fetch();

        self.regs.L = self.ram[addr as usize];
        self.regs.H = self.ram[addr1 as usize];
    }

    // Complement reg A
    fn cma(&mut self) {
        self.regs.A = !self.regs.A;
    }

    // Store reg A in addr
    fn sta(&mut self) {
        let addr = self.return_addr();

        self.ram[addr as usize] = self.regs.A;
    }

    // Set carry flag
    fn stc(&mut self) {

        // Flags not yet implemented
    }

    // Load reg A to byte at addr
    fn lda(&mut self) {
        let addr = self.return_addr();

        self.regs.A = self.ram[addr as usize];
    }

    // Complement carry flag
    fn cmc(&mut self) {

        // Flags not yet implemented
    }

    // Move register value to other register
    fn mov(&mut self, reg_to: &str, reg_from: &str) {
        let addr = self.regs.return_joined_regs("HL");

        match reg_to {
            "B" => {
                match reg_from {
                    "B" => {self.regs.B = self.regs.B},
                    "C" => {self.regs.B = self.regs.C},
                    "D" => {self.regs.B = self.regs.D},
                    "E" => {self.regs.B = self.regs.E},
                    "H" => {self.regs.B = self.regs.H},
                    "L" => {self.regs.B = self.regs.L},
                    "M" => {
                        self.regs.B = self.return_byte_at_location("HL")
                    },
                    "A" => {self.regs.B = self.regs.A},
                    _ => {}
                }
            },
            "C" => {
                match reg_from {
                    "B" => {self.regs.C = self.regs.B},
                    "C" => {self.regs.C = self.regs.C},
                    "D" => {self.regs.C = self.regs.D},
                    "E" => {self.regs.C = self.regs.E},
                    "H" => {self.regs.C = self.regs.H},
                    "L" => {self.regs.C = self.regs.L},
                    "M" => {
                        self.regs.C = self.return_byte_at_location("HL")
                    },
                    "A" => {self.regs.C = self.regs.A},
                    _ => {}
                }
            },
            "D" => {
                match reg_from {
                    "B" => {self.regs.D = self.regs.B},
                    "C" => {self.regs.D = self.regs.C},
                    "D" => {self.regs.D = self.regs.D},
                    "E" => {self.regs.D = self.regs.E},
                    "H" => {self.regs.D = self.regs.H},
                    "L" => {self.regs.D = self.regs.L},
                    "M" => {
                        self.regs.D = self.return_byte_at_location("HL")
                    },
                    "A" => {self.regs.D = self.regs.A},
                    _ => {}
                }
            },
            "E" => {
                match reg_from {
                    "B" => {self.regs.E = self.regs.B},
                    "C" => {self.regs.E = self.regs.C},
                    "D" => {self.regs.E = self.regs.D},
                    "E" => {self.regs.E = self.regs.E},
                    "H" => {self.regs.E = self.regs.H},
                    "L" => {self.regs.E = self.regs.L},
                    "M" => {
                        self.regs.E = self.return_byte_at_location("HL")
                    },
                    "A" => {self.regs.E = self.regs.A},
                    _ => {}
                }
            },
            "H" => {
                match reg_from {
                    "B" => {self.regs.H = self.regs.B},
                    "C" => {self.regs.H = self.regs.C},
                    "D" => {self.regs.H = self.regs.D},
                    "E" => {self.regs.H = self.regs.E},
                    "H" => {self.regs.H = self.regs.H},
                    "L" => {self.regs.H = self.regs.L},
                    "M" => {
                        self.regs.H = self.return_byte_at_location("HL")
                    },
                    "A" => {self.regs.H = self.regs.A},
                    _ => {}
                }
            },
            "L" => {
                match reg_from {
                    "B" => {self.regs.L = self.regs.B},
                    "C" => {self.regs.L = self.regs.C},
                    "D" => {self.regs.L = self.regs.D},
                    "E" => {self.regs.L = self.regs.E},
                    "H" => {self.regs.L = self.regs.H},
                    "L" => {self.regs.L = self.regs.L},
                    "M" => {
                        self.regs.L = self.return_byte_at_location("HL")
                    },
                    "A" => {self.regs.L = self.regs.A},
                    _ => {}
                }
            },
            "M" => {match reg_from {                
                "B" => {self.ram[addr as usize] = self.regs.B},
                "C" => {self.ram[addr as usize] = self.regs.C},
                "D" => {self.ram[addr as usize] = self.regs.D},
                "E" => {self.ram[addr as usize] = self.regs.E},
                "H" => {self.ram[addr as usize] = self.regs.H},
                "L" => {self.ram[addr as usize] = self.regs.L},
                "A" => {self.ram[addr as usize] = self.regs.A},
                _ => {}
            }
        },
            "A" => {
                match reg_from {
                    "B" => {self.regs.A = self.regs.B},
                    "C" => {self.regs.A = self.regs.C},
                    "D" => {self.regs.A = self.regs.D},
                    "E" => {self.regs.A = self.regs.E},
                    "H" => {self.regs.A = self.regs.H},
                    "L" => {self.regs.A = self.regs.L},
                    "M" => {
                        self.regs.A = 
                        self.return_byte_at_location("HL")
                    },
                    "A" => {self.regs.A = self.regs.A},
                    _ => {}
                }
            },
            _ => {}
        }
    }

    // Halt CPU
    fn hlt(&mut self) {
        // Clock cycles not yet implemented
    }


    fn addc(&mut self) {

    }

    fn sub(&mut self) {

    }

    fn sbb(&mut self) {

    }

    fn ana(&mut self) {

    }

    fn xra(&mut self) {

    }

    fn ora(&mut self) {

    }

    fn cmp(&mut self) {

    }

    fn rnz(&mut self) {

    }

    fn pop(&mut self) {

    }

    fn jnz(&mut self) {

    }

    fn jmp(&mut self) {

    }

    fn cnz(&mut self) {

    }

    fn adi(&mut self) {

    }

    fn rz(&mut self) {

    }

    fn ret(&mut self) {

    }


    fn jz(&mut self) {

    }

    fn cz(&mut self) {

    }

    fn call(&mut self) {

    }

    fn aci(&mut self) {

    }

    fn rnc(&mut self) {

    }

    fn jnc(&mut self) {

    }

    fn out(&mut self) {

    }

    fn cnc(&mut self) {

    }

    fn sui(&mut self) {

    }

    fn rc(&mut self) {

    }

    fn jc(&mut self) {

    }

    fn ind(&mut self) {

    }

    fn cc(&mut self) {

    }

    fn sbi(&mut self) {

    }

    fn rpo(&mut self) {

    }

    fn jpo(&mut self) {

    }

    fn xthl(&mut self) {

    }

    fn cpo(&mut self) {

    }

    fn ani(&mut self) {

    }

    fn rpe(&mut self) {

    }

    fn pchl(&mut self) {

    }

    fn jpe(&mut self) {

    }

    fn xchg(&mut self) {

    }

    fn cpe(&mut self) {

    }

    fn xri(&mut self) {

    }

    fn rp(&mut self) {

    }

    fn jp(&mut self) {

    }

    fn di(&mut self) {

    }

    fn cp(&mut self) {

    }

    


    fn rst(&mut self) {

    }

    fn cpi(&mut self) {

    }

    fn cm(&mut self) {

    }

    fn ei(&mut self) {

    }

    fn jm(&mut self) {

    }

    fn sphl(&mut self) {

    }

    fn rm(&mut self) {

    }

    fn ori(&mut self) {

    }

    fn push(&mut self) {

    }

    fn return_addr(&mut self) -> u16 {
        let lo = self.fetch();
        let hi = self.fetch();

        let addr = (hi as u16) << 8 | (lo as u16);

        return addr
    }

    fn return_byte_at_location(&mut self, reg_pair: &str) -> u8 {
        let addr = self.regs.return_joined_regs(reg_pair);

        return self.ram[addr as usize]
    }
}