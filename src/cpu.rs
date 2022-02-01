use crate::registers::Regs;

// This allows us to print to terminal.
#[derive(Debug)]
pub struct CPU {
    ram: [u8; 65536],
    pc: u16,
    sp: u16,
    regs: Regs,
}

impl CPU {
    pub fn init_cpu() -> Self {
        CPU {
            ram: [0; 65536],
            pc: 0,
            sp: 0,
            regs: Regs::init_regs(),
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
        let addr = self.return_addr();
        match inst {
            0x00 => self.nop(),
            0x01 => self.lxi("B"),
            0x02 => self.stax("B"),
            0x03 => self.inx("B"),
            0x04 => self.inr("B"),
            0x05 => self.dcr("B"),
            0x06 => self.mvi("B"),
            0x07 => self.rlc(),
            0x09 => self.dad("B"),
            0x0A => self.ldax("B"),
            0x0B => self.dcx("B"),
            0x0C => self.inr("C"),
            0x0D => self.dcr("C"),
            0x0E => self.mvi("C"),
            0x0F => self.rrc(),

            0x11 => self.lxi("D"),
            0x12 => self.stax("D"),
            0x13 => self.inx("D"),
            0x14 => self.inr("D"),
            0x15 => self.dcr("D"),
            0x16 => self.mvi("D"),
            0x17 => self.ral(),
            0x19 => self.dad("D"),
            0x1A => self.ldax("D"),
            0x1B => self.dcx("D"),
            0x1C => self.inr("E"),
            0x1D => self.dcr("E"),
            0x1E => self.mvi("E"),
            0x1F => self.rar(),

            0x21 => self.lxi("H"),
            0x22 => self.shld(),
            0x23 => self.inx("H"),
            0x24 => self.inr("H"),
            0x25 => self.dcr("H"),
            0x26 => self.mvi("H"),
            0x27 => self.daa(),
            0x29 => self.dad("H"),
            0x2A => self.lhld(),
            0x2B => self.dcx("H"),
            0x2C => self.inr("L"),
            0x2D => self.dcr("L"),
            0x2E => self.mvi("L"),
            0x2F => self.cma(),

            0x31 => self.lxi("SP"),
            0x32 => self.sta(),
            0x33 => self.inx("SP"),
            0x34 => self.inr("M"),
            0x35 => self.dcr("M"),
            0x36 => self.mvi("M"),
            0x37 => self.stc(),
            0x39 => self.dad("SP"),
            0x3A => self.lda(),
            0x3B => self.dcx("SP"),
            0x3C => self.inr("A"),
            0x3D => self.dcr("A"),
            0x3E => self.mvi("A"),
            0x3F => self.cmc(),

            0x40 => self.mov("B", "B"),
            0x41 => self.mov("B", "C"),
            0x42 => self.mov("B", "D"),
            0x43 => self.mov("B", "E"),
            0x44 => self.mov("B", "H"),
            0x45 => self.mov("B", "L"),
            0x46 => self.mov("B", "M"),
            0x47 => self.mov("B", "A"),
            0x48 => self.mov("C", "B"),
            0x49 => self.mov("C", "C"),
            0x4A => self.mov("C", "D"),
            0x4B => self.mov("C", "E"),
            0x4C => self.mov("C", "H"),
            0x4D => self.mov("C", "L"),
            0x4E => self.mov("C", "M"),
            0x4F => self.mov("C", "A"),

            0x50 => self.mov("D", "B"),
            0x51 => self.mov("D", "C"),
            0x52 => self.mov("D", "D"),
            0x53 => self.mov("D", "E"),
            0x54 => self.mov("D", "H"),
            0x55 => self.mov("D", "L"),
            0x56 => self.mov("D", "M"),
            0x57 => self.mov("D", "A"),
            0x58 => self.mov("E", "B"),
            0x59 => self.mov("E", "C"),
            0x5A => self.mov("E", "D"),
            0x5B => self.mov("E", "E"),
            0x5C => self.mov("E", "H"),
            0x5D => self.mov("E", "L"),
            0x5E => self.mov("E", "M"),
            0x5F => self.mov("E", "A"),

            0x60 => self.mov("H", "B"),
            0x61 => self.mov("H", "C"),
            0x62 => self.mov("H", "D"),
            0x63 => self.mov("H", "E"),
            0x64 => self.mov("H", "H"),
            0x65 => self.mov("H", "L"),
            0x66 => self.mov("H", "M"),
            0x67 => self.mov("H", "A"),
            0x68 => self.mov("L", "B"),
            0x69 => self.mov("L", "C"),
            0x6A => self.mov("L", "D"),
            0x6B => self.mov("L", "E"),
            0x6C => self.mov("L", "H"),
            0x6D => self.mov("L", "L"),
            0x6E => self.mov("L", "M"),
            0x6F => self.mov("L", "A"),

            0x70 => self.mov("M", "B"),
            0x71 => self.mov("M", "C"),
            0x72 => self.mov("M", "D"),
            0x73 => self.mov("M", "E"),
            0x74 => self.mov("M", "H"),
            0x75 => self.mov("M", "L"),
            0x76 => self.hlt(),
            0x77 => self.mov("M", "A"),
            0x78 => self.mov("A", "B"),
            0x79 => self.mov("A", "C"),
            0x7A => self.mov("A", "D"),
            0x7B => self.mov("A", "E"),
            0x7C => self.mov("A", "H"),
            0x7D => self.mov("A", "L"),
            0x7E => self.mov("A", "M"),
            0x7F => self.mov("A", "A"),

            0x80 => self.add("B"),
            0x81 => self.add("C"),
            0x82 => self.add("D"),
            0x83 => self.add("E"),
            0x84 => self.add("H"),
            0x85 => self.add("L"),
            0x86 => self.add("M"),
            0x87 => self.add("A"),
            0x88 => self.adc("B"),
            0x89 => self.adc("C"),
            0x8A => self.adc("D"),
            0x8B => self.adc("E"),
            0x8C => self.adc("H"),
            0x8D => self.adc("L"),
            0x8E => self.adc("M"),
            0x8F => self.adc("A"),

            0x90 => self.sub("B"),
            0x91 => self.sub("C"),
            0x92 => self.sub("D"),
            0x93 => self.sub("E"),
            0x94 => self.sub("H"),
            0x95 => self.sub("L"),
            0x96 => self.sub("M"),
            0x97 => self.sub("A"),
            0x98 => self.sbb("B"),
            0x99 => self.sbb("C"),
            0x9A => self.sbb("D"),
            0x9B => self.sbb("E"),
            0x9C => self.sbb("H"),
            0x9D => self.sbb("L"),
            0x9E => self.sbb("M"),
            0x9F => self.sbb("A"),

            0xA0 => self.ana("B"),
            0xA1 => self.ana("C"),
            0xA2 => self.ana("D"),
            0xA3 => self.ana("E"),
            0xA4 => self.ana("H"),
            0xA5 => self.ana("L"),
            0xA6 => self.ana("M"),
            0xA7 => self.ana("A"),
            0xA8 => self.xra("B"),
            0xA9 => self.xra("C"),
            0xAA => self.xra("D"),
            0xAB => self.xra("E"),
            0xAC => self.xra("H"),
            0xAD => self.xra("L"),
            0xAE => self.xra("M"),
            0xAF => self.xra("A"),

            0xB0 => self.ora("B"),
            0xB1 => self.ora("C"),
            0xB2 => self.ora("D"),
            0xB3 => self.ora("E"),
            0xB4 => self.ora("H"),
            0xB5 => self.ora("L"),
            0xB6 => self.ora("M"),
            0xB7 => self.ora("A"),
            0xB8 => self.cmp("B"),
            0xB9 => self.cmp("C"),
            0xBA => self.cmp("D"),
            0xBB => self.cmp("E"),
            0xBC => self.cmp("H"),
            0xBD => self.cmp("L"),
            0xBE => self.cmp("M"),
            0xBF => self.cmp("A"),

            0xC0 => self.rnz(),
            0xC1 => self.pop("BC"),
            0xC2 => self.jnz(),
            0xC3 => self.jmp(),
            0xC4 => self.cnz(),
            0xC5 => self.push("BC"),
            0xC6 => self.adi(),
            0xC7 => self.rst(0x0),
            0xC8 => self.rz(),
            0xC9 => self.ret(),
            0xCA => self.jz(),
            0xCC => self.cz(),
            0xCD => self.call(addr),
            0xCE => self.aci(),
            0xCF => self.rst(0x8),

            0xD0 => self.rnc(),
            0xD1 => self.pop("DE"),
            0xD2 => self.jnc(),
            0xD3 => self.out(),
            0xD4 => self.cnc(),
            0xD5 => self.push("DE"),
            0xD6 => self.sui(),
            0xD7 => self.rst(0x10),
            0xD8 => self.rc(),
            0xDA => self.jc(),
            0xDB => self.ind(),
            0xDC => self.cc(),
            0xDE => self.sbi(),
            0xDF => self.rst(0x18),

            0xE0 => self.rpo(),
            0xE1 => self.pop("HL"),
            0xE2 => self.jpo(),
            0xE3 => self.xthl(),
            0xE4 => self.cpo(),
            0xE5 => self.push("HL"),
            0xE6 => self.ani(),
            0xE7 => self.rst(0x20),
            0xE8 => self.rpe(),
            0xE9 => self.pchl(),
            0xEA => self.jpe(),
            0xEB => self.xchg(),
            0xEC => self.cpe(),
            0xEE => self.xri(),
            0xEF => self.rst(0x28),

            0xF0 => self.rp(),
            0xF1 => self.pop("PSW"),
            0xF2 => self.jp(),
            0xF3 => self.di(),
            0xF4 => self.cp(),
            0xF5 => self.push("PSW"),
            0xF6 => self.ori(),
            0xF7 => self.rst(0x30),
            0xF8 => self.rm(),
            0xF9 => self.sphl(),
            0xFA => self.jm(),
            0xFB => self.ei(),
            0xFC => self.cm(),
            0xFE => self.cpi(),
            0xFF => self.rst(0x38),

            _ => {}
        };
    }

    fn nop(&mut self) {}

    // Load the immediate 16 bit value to reg pair
    // Reg C == byte 2
    // Reg B == Byte 3
    fn lxi(&mut self, reg: &str) {
        match reg {
            "B" => {
                self.regs.C = self.fetch();
                self.regs.B = self.fetch();
            }
            "D" => {
                self.regs.D = self.fetch();
                self.regs.E = self.fetch();
            }
            "H" => {
                self.regs.H = self.fetch();
                self.regs.L = self.fetch();
            }
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
        } else if reg == "H" {
            self.regs.return_joined_regs("HL")
        } else {
            0
        };

        self.ram[location as usize] = self.regs.A;
    }

    // Increment reg pair by 1
    fn inx(&mut self, reg: &str) {
        if reg != "SP" {
            let reg_pair = if reg == "B" {
                self.regs.return_joined_regs("BC")
            } else if reg == "D" {
                self.regs.return_joined_regs("DE")
            } else if reg == "H" {
                self.regs.return_joined_regs("HL")
            } else {
                0
            };

            // Convert register pair into tuple for easier splitting
            let reg_pair_tup = self.regs.split_regs(reg_pair.overflowing_add(1).0);

            self.regs.B = reg_pair_tup.0;
            self.regs.B = reg_pair_tup.1;
        } else {
            self.sp = self.sp.overflowing_add(1).0
        }
    }

    // Increment register
    fn inr(&mut self, reg: &str) {
        let addr = self.return_byte_at_location("HL");
        match reg {
            "B" => self.regs.B = self.regs.B.overflowing_add(1).0,
            "C" => self.regs.C = self.regs.C.overflowing_add(1).0,
            "D" => self.regs.D = self.regs.D.overflowing_add(1).0,
            "E" => self.regs.E = self.regs.E.overflowing_add(1).0,
            "H" => self.regs.H = self.regs.H.overflowing_add(1).0,
            "L" => self.regs.L = self.regs.L.overflowing_add(1).0,
            "M" => self.ram[addr as usize] += 1,
            _ => {}
        }
    }

    // Decrement register
    fn dcr(&mut self, reg: &str) {
        let addr = self.return_byte_at_location("HL");
        match reg {
            "B" => self.regs.B = self.regs.B.overflowing_sub(1).0,
            "C" => self.regs.C = self.regs.C.overflowing_sub(1).0,
            "D" => self.regs.D = self.regs.D.overflowing_sub(1).0,
            "E" => self.regs.E = self.regs.E.overflowing_sub(1).0,
            "H" => self.regs.H = self.regs.H.overflowing_sub(1).0,
            "L" => self.regs.L = self.regs.L.overflowing_sub(1).0,
            "M" => self.ram[addr as usize] += 1,
            _ => {}
        };
    }

    // Reg B = next byte fetched
    fn mvi(&mut self, reg: &str) {
        let byte = self.fetch();
        let addr = self.return_byte_at_location("HL");

        match reg {
            "B" => self.regs.B = byte,
            "C" => self.regs.C = byte,
            "D" => self.regs.D = byte,
            "E" => self.regs.E = byte,
            "H" => self.regs.H = byte,
            "L" => self.regs.L = byte,
            "M" => self.ram[addr as usize] = byte,
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
        let reg_to_add_to_hl = match reg {
            "B" => self.regs.return_joined_regs("BC"),
            "D" => self.regs.return_joined_regs("DE"),
            "H" => self.regs.return_joined_regs("HL"),
            _ => 0,
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
                self.regs.L = self.regs.split_regs(hl).0;
            }
            "H" => {
                hl += reg_to_add_to_hl;
                self.regs.H = self.regs.split_regs(hl).0;
                self.regs.L = self.regs.split_regs(hl).0;
            }
            "SP" => {
                self.sp += reg_to_add_to_hl;
            }
            _ => (),
        }
    }

    // Reg A = location indexed by reg pair
    fn ldax(&mut self, reg: &str) {
        let mem_index = match reg {
            "B" => self.regs.return_joined_regs("BC"),
            "D" => self.regs.return_joined_regs("DE"),
            "H" => self.regs.return_joined_regs("HL"),
            _ => 0,
        };

        self.regs.A = self.ram[mem_index as usize];
    }

    // Decrement reg pair
    fn dcx(&mut self, reg: &str) {
        if reg != "SP" {
            let reg_pair = if reg == "B" {
                self.regs.return_joined_regs("BC")
            } else if reg == "D" {
                self.regs.return_joined_regs("DE")
            } else if reg == "H" {
                self.regs.return_joined_regs("HL")
            } else {
                0
            };

            let result = reg_pair.overflowing_sub(1);

            // Convert register pair into tuple for easier splitting
            let reg_pair_tup = self.regs.split_regs(result.0);

            self.regs.B = reg_pair_tup.0;
            self.regs.B = reg_pair_tup.1;
        } else {
            self.sp = self.sp.overflowing_sub(1).0;
        }
    }

    // Rotate reg A right
    fn rrc(&mut self) {
        self.regs.A = self.regs.A.rotate_right(7);
    }

    // Rotate reg A left with carry
    fn ral(&mut self) {}

    // Rotate reg A right with carry
    fn rar(&mut self) {}

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
        self.regs.set_flags("C");
    }

    // Load reg A to byte at addr
    fn lda(&mut self) {
        let addr = self.return_addr();

        self.regs.A = self.ram[addr as usize];
    }

    // Complement carry flag
    fn cmc(&mut self) {
        self.regs.PSW.C = !self.regs.PSW.C;
    }

    // Move register value to other register
    fn mov(&mut self, reg_to: &str, reg_from: &str) {
        let addr = self.regs.return_joined_regs("HL");

        match reg_to {
            "B" => match reg_from {
                "B" => self.regs.B = self.regs.B,
                "C" => self.regs.B = self.regs.C,
                "D" => self.regs.B = self.regs.D,
                "E" => self.regs.B = self.regs.E,
                "H" => self.regs.B = self.regs.H,
                "L" => self.regs.B = self.regs.L,
                "M" => self.regs.B = self.return_byte_at_location("HL"),
                "A" => self.regs.B = self.regs.A,
                _ => {}
            },
            "C" => match reg_from {
                "B" => self.regs.C = self.regs.B,
                "C" => self.regs.C = self.regs.C,
                "D" => self.regs.C = self.regs.D,
                "E" => self.regs.C = self.regs.E,
                "H" => self.regs.C = self.regs.H,
                "L" => self.regs.C = self.regs.L,
                "M" => self.regs.C = self.return_byte_at_location("HL"),
                "A" => self.regs.C = self.regs.A,
                _ => {}
            },
            "D" => match reg_from {
                "B" => self.regs.D = self.regs.B,
                "C" => self.regs.D = self.regs.C,
                "D" => self.regs.D = self.regs.D,
                "E" => self.regs.D = self.regs.E,
                "H" => self.regs.D = self.regs.H,
                "L" => self.regs.D = self.regs.L,
                "M" => self.regs.D = self.return_byte_at_location("HL"),
                "A" => self.regs.D = self.regs.A,
                _ => {}
            },
            "E" => match reg_from {
                "B" => self.regs.E = self.regs.B,
                "C" => self.regs.E = self.regs.C,
                "D" => self.regs.E = self.regs.D,
                "E" => self.regs.E = self.regs.E,
                "H" => self.regs.E = self.regs.H,
                "L" => self.regs.E = self.regs.L,
                "M" => self.regs.E = self.return_byte_at_location("HL"),
                "A" => self.regs.E = self.regs.A,
                _ => {}
            },
            "H" => match reg_from {
                "B" => self.regs.H = self.regs.B,
                "C" => self.regs.H = self.regs.C,
                "D" => self.regs.H = self.regs.D,
                "E" => self.regs.H = self.regs.E,
                "H" => self.regs.H = self.regs.H,
                "L" => self.regs.H = self.regs.L,
                "M" => self.regs.H = self.return_byte_at_location("HL"),
                "A" => self.regs.H = self.regs.A,
                _ => {}
            },
            "L" => match reg_from {
                "B" => self.regs.L = self.regs.B,
                "C" => self.regs.L = self.regs.C,
                "D" => self.regs.L = self.regs.D,
                "E" => self.regs.L = self.regs.E,
                "H" => self.regs.L = self.regs.H,
                "L" => self.regs.L = self.regs.L,
                "M" => self.regs.L = self.return_byte_at_location("HL"),
                "A" => self.regs.L = self.regs.A,
                _ => {}
            },
            "M" => match reg_from {
                "B" => self.ram[addr as usize] = self.regs.B,
                "C" => self.ram[addr as usize] = self.regs.C,
                "D" => self.ram[addr as usize] = self.regs.D,
                "E" => self.ram[addr as usize] = self.regs.E,
                "H" => self.ram[addr as usize] = self.regs.H,
                "L" => self.ram[addr as usize] = self.regs.L,
                "A" => self.ram[addr as usize] = self.regs.A,
                _ => {}
            },
            "A" => match reg_from {
                "B" => self.regs.A = self.regs.B,
                "C" => self.regs.A = self.regs.C,
                "D" => self.regs.A = self.regs.D,
                "E" => self.regs.A = self.regs.E,
                "H" => self.regs.A = self.regs.H,
                "L" => self.regs.A = self.regs.L,
                "M" => self.regs.A = self.return_byte_at_location("HL"),
                "A" => self.regs.A = self.regs.A,
                _ => {}
            },
            _ => {}
        }
    }

    // Halt CPU
    fn hlt(&mut self) {
        // Clock cycles not yet implemented
    }

    // Add register or memory to reg A
    fn add(&mut self, reg: &str) {
        match reg {
            "B" => self.regs.A = self.regs.A.overflowing_add(self.regs.B).0,
            "C" => self.regs.A = self.regs.A.overflowing_add(self.regs.C).0,
            "D" => self.regs.A = self.regs.A.overflowing_add(self.regs.D).0,
            "E" => self.regs.A = self.regs.A.overflowing_add(self.regs.E).0,
            "H" => self.regs.A = self.regs.A.overflowing_add(self.regs.H).0,
            "L" => self.regs.A = self.regs.A.overflowing_add(self.regs.L).0,
            "M" => {
                self.regs.A = self
                    .regs
                    .A
                    .overflowing_add(self.return_byte_at_location("HL"))
                    .0
            }
            "A" => self.regs.A = self.regs.A.overflowing_add(self.regs.A).0,
            _ => {}
        }
    }

    // Add register or memory to reg a with carry
    // Add carry bit to reg
    fn adc(&mut self, reg: &str) {
        match reg {
            "B" => {
                self.regs.A += self.regs.B;
                if self.regs.PSW.C {
                    self.regs.A += 1;
                }
            }
            "C" => {
                self.regs.A += self.regs.C;
                if self.regs.PSW.C {
                    self.regs.A += 1;
                }
            }
            "D" => {
                self.regs.A += self.regs.D;
                if self.regs.PSW.C {
                    self.regs.A += 1;
                }
            }
            "E" => {
                self.regs.A += self.regs.D;
                if self.regs.PSW.C {
                    self.regs.A += 1;
                }
            }
            "H" => {
                self.regs.A += self.regs.H;
                if self.regs.PSW.C {
                    self.regs.A += 1;
                }
            }
            "L" => {
                self.regs.A += self.regs.L;
                if self.regs.PSW.C {
                    self.regs.A += 1;
                }
            }
            "M" => {
                self.regs.A += self.ram[self.regs.return_joined_regs("HL") as usize];
                if self.regs.PSW.C {
                    self.regs.A += 1;
                }
            }
            "A" => {
                self.regs.A += self.regs.A;
                if self.regs.PSW.C {
                    self.regs.A += 1;
                }
            }
            _ => {}
        }
    }

    // Subtract register or memory from reg A
    fn sub(&mut self, reg: &str) {
        match reg {
            "B" => self.regs.A = self.regs.A.overflowing_sub(self.regs.B).0,
            "C" => self.regs.A = self.regs.A.overflowing_sub(self.regs.C).0,
            "D" => self.regs.A = self.regs.A.overflowing_sub(self.regs.D).0,
            "E" => self.regs.A = self.regs.A.overflowing_sub(self.regs.E).0,
            "H" => self.regs.A = self.regs.A.overflowing_sub(self.regs.H).0,
            "L" => self.regs.A = self.regs.A.overflowing_sub(self.regs.L).0,
            "M" => {
                self.regs.A = self
                    .regs
                    .A
                    .overflowing_sub(self.return_byte_at_location("HL"))
                    .0
            }
            "A" => self.regs.A = self.regs.A.overflowing_sub(self.regs.A).0,
            _ => {}
        }
    }

    // Subtract from A with borrow
    // Subtract carry bit
    fn sbb(&mut self, reg: &str) {
        match reg {
            "B" => {
                self.regs.A -= self.regs.B;
                if self.regs.PSW.C {
                    self.regs.A -= 1;
                }
            }
            "C" => {
                self.regs.A -= self.regs.C;
                if self.regs.PSW.C {
                    self.regs.A -= 1;
                }
            }
            "D" => {
                self.regs.A -= self.regs.D;
                if self.regs.PSW.C {
                    self.regs.A -= 1;
                }
            }
            "E" => {
                self.regs.A -= self.regs.D;
                if self.regs.PSW.C {
                    self.regs.A -= 1;
                }
            }
            "H" => {
                self.regs.A -= self.regs.H;
                if self.regs.PSW.C {
                    self.regs.A -= 1;
                }
            }
            "L" => {
                self.regs.A -= self.regs.L;
                if self.regs.PSW.C {
                    self.regs.A -= 1;
                }
            }
            "M" => {
                self.regs.A -= self.ram[self.regs.return_joined_regs("HL") as usize];
                if self.regs.PSW.C {
                    self.regs.A -= 1;
                }
            }
            "A" => {
                self.regs.A -= self.regs.A;
                if self.regs.PSW.C {
                    self.regs.A -= 1;
                }
            }
            _ => {}
        }
    }

    // Bitwise & reg A with reg
    fn ana(&mut self, reg: &str) {
        match reg {
            "B" => self.regs.A &= self.regs.B,
            "C" => self.regs.A &= self.regs.C,
            "D" => self.regs.A &= self.regs.D,
            "E" => self.regs.A &= self.regs.E,
            "H" => self.regs.A &= self.regs.H,
            "L" => self.regs.A &= self.regs.L,
            "M" => self.regs.A &= self.return_byte_at_location("HL"),
            "A" => self.regs.A &= self.regs.A,
            _ => {}
        }
    }

    // XOR with reg
    fn xra(&mut self, reg: &str) {
        match reg {
            "B" => self.regs.A ^= self.regs.B,
            "C" => self.regs.A ^= self.regs.C,
            "D" => self.regs.A ^= self.regs.D,
            "E" => self.regs.A ^= self.regs.E,
            "H" => self.regs.A ^= self.regs.H,
            "L" => self.regs.A ^= self.regs.L,
            "M" => self.regs.A ^= self.return_byte_at_location("HL"),
            "A" => self.regs.A ^= self.regs.A,
            _ => {}
        }
    }

    // OR with reg
    fn ora(&mut self, reg: &str) {
        match reg {
            "B" => self.regs.A |= self.regs.B,
            "C" => self.regs.A |= self.regs.C,
            "D" => self.regs.A |= self.regs.D,
            "E" => self.regs.A |= self.regs.E,
            "H" => self.regs.A |= self.regs.H,
            "L" => self.regs.A |= self.regs.L,
            "M" => self.regs.A |= self.return_byte_at_location("HL"),
            "A" => self.regs.A |= self.regs.A,
            _ => {}
        }
    }

    // Compare registers. Subtract from eachother
    fn cmp(&mut self, reg: &str) {
        match reg {
            "B" => {
                if self.regs.A - self.regs.B == 0 {
                    self.regs.set_flags("P");
                }
            }
            "C" => {
                if self.regs.A - self.regs.C == 0 {
                    self.regs.set_flags("P");
                }
            }
            "D" => {
                if self.regs.A - self.regs.D == 0 {
                    self.regs.set_flags("P");
                }
            }
            "E" => {
                if self.regs.A - self.regs.E == 0 {
                    self.regs.set_flags("P");
                }
            }
            "H" => {
                if self.regs.A - self.regs.H == 0 {
                    self.regs.set_flags("P");
                }
            }
            "L" => {
                if self.regs.A - self.regs.L == 0 {
                    self.regs.set_flags("P");
                }
            }
            "M" => {
                if self.regs.A - self.return_byte_at_location("HL") == 0 {
                    self.regs.set_flags("P");
                }
            }
            "A" => {
                if self.regs.A - self.regs.A == 0 {
                    self.regs.set_flags("P");
                }
            }
            _ => {}
        }
    }

    // Return if zero is 0
    fn rnz(&mut self) {
        if self.regs.PSW.Z {
            self.ret();
        } else {
        }
    }

    // Pop from stack to register pair
    // C = sp
    // B = sp + 1
    // sp = sp + 2
    fn pop(&mut self, reg: &str) {
        match reg {
            "BC" => {
                self.regs.B = self.sp as u8;
                self.regs.C = (self.sp + 1) as u8;
                self.sp = self.sp + 2;
            }
            "DE" => {
                self.regs.B = self.sp as u8;
                self.regs.C = (self.sp + 1) as u8;
                self.sp = self.sp + 2;
            }
            "HL" => {
                self.regs.B = self.sp as u8;
                self.regs.C = (self.sp + 1) as u8;
                self.sp = self.sp + 2;
            }
            "PSW" => {}
            _ => {}
        }
    }

    // Jump if zero bit is set
    fn jnz(&mut self) {
        if self.regs.PSW.Z {
            self.pc = self.return_addr();
        } else {
        }
    }

    // Jump to addr
    fn jmp(&mut self) {
        self.pc = self.return_addr();
    }

    // Call if zero not set
    fn cnz(&mut self) {
        if !self.regs.PSW.Z {
            let addr = self.return_addr();
            self.call(addr);
        } else {
        }
    }

    // Add immediate to reg A
    fn adi(&mut self) {
        self.regs.A = self.regs.A.overflowing_add(self.fetch()).0;
    }

    // Return if zero bit is set
    fn rz(&mut self) {
        if self.regs.PSW.Z {
            self.ret();
        } else {
        }
    }

    // Return from subroutine
    // PC.lo = sp
    // PC.hi = sp + 1
    // SP = sp + 2
    fn ret(&mut self) {
        let lo = (self.sp & 0xFF) as u8;
        self.sp += 1;
        let hi = (self.sp & 0xFF) as u8;
        self.sp += 2;

        self.pc = (hi as u16) << 8 | lo as u16;
    }

    // Jump if zero is set
    fn jz(&mut self) {
        if self.regs.PSW.Z {
            self.pc = self.return_addr();
        } else {
        }
    }

    // Call if zero is set
    fn cz(&mut self) {
        if self.regs.PSW.Z {
            let addr = self.return_addr();
            self.call(addr);
        } else {
        }
    }

    // Call subroutine
    // SP - 1 = PC.hi
    // SP - 2 = PC.lo
    // SP = sp - 2
    // PC = addr
    fn call(&mut self, addr: u16) {
        self.sp -= 1;
        self.sp = self.pc & 0xFF00;
        self.sp -= 2;
        self.sp = self.pc & 0x00FF;
        self.sp -= 2;

        self.pc = addr;
    }

    // Add immediate to reg A with carry
    fn aci(&mut self) {
        self.regs.A = self.fetch();
    }

    // Return is carry is 0
    fn rnc(&mut self) {
        if !self.regs.PSW.C {
            self.ret();
        } else {
        }
    }

    // Jump is carry not set
    fn jnc(&mut self) {
        if !self.regs.PSW.C {
            self.pc = self.return_addr();
        }
    }

    // Output A to port
    fn out(&mut self) {}

    // Call if carry is not set
    fn cnc(&mut self) {
        if !self.regs.PSW.C {
            let addr = self.return_addr();
            self.call(addr);
        } else {
        }
    }

    // Subtract immediate from A
    fn sui(&mut self) {
        self.regs.A = self.regs.A.overflowing_sub(self.fetch()).0;
    }

    // Return if carry is set
    fn rc(&mut self) {
        if self.regs.PSW.C {
            self.ret();
        } else {
        }
    }

    // Jump if carry is set
    fn jc(&mut self) {
        if self.regs.PSW.C {
            self.pc = self.return_addr();
        } else {
        }
    }

    // Input port to A
    fn ind(&mut self) {}

    // Call if carry is set
    fn cc(&mut self) {
        if self.regs.PSW.C {
            let addr = self.return_addr();
            self.call(addr);
        } else {
        }
    }

    // Subtract immediate from reg A with borrow
    fn sbi(&mut self) {
        self.regs.A = self.regs.A.overflowing_sub(self.fetch()).0;
        if self.regs.PSW.C {
            self.regs.A -= 1
        }
    }

    // Return if parity not set
    fn rpo(&mut self) {
        if !self.regs.PSW.P {
            self.ret();
        } else {
        }
    }

    // Jump is parity not set
    fn jpo(&mut self) {
        if !self.regs.PSW.P {
            self.pc = self.return_addr();
        } else {
        }
    }

    // Exchange stack
    // Reg L is swapped with memory at sp
    // Reg h is swapped with memory at sp + 1
    fn xthl(&mut self) {
        let byte_one = self.ram[self.sp as usize];
        let byte_two = self.ram[(self.sp + 1) as usize];
        let l = self.regs.L;
        let h = self.regs.H;

        self.ram[self.sp as usize] = self.regs.L;
        self.regs.L = byte_one;

        self.ram[(self.sp + 1) as usize] = self.regs.H;
        self.regs.H = byte_two;
    }

    // Call if parity not set
    fn cpo(&mut self) {
        if !self.regs.PSW.P {
            let addr = self.return_addr();
            self.call(addr);
        } else {
        }
    }

    // AND reg A with immediate
    fn ani(&mut self) {
        self.regs.A &= self.fetch();
    }

    // Return if parity is set
    fn rpe(&mut self) {
        if self.regs.PSW.P {
            self.ret();
        } else {
        }
    }

    // Load pc from HL
    fn pchl(&mut self) {
        self.pc = self.regs.return_joined_regs("HL");
    }

    // Jump if parity is set
    fn jpe(&mut self) {
        if self.regs.PSW.P {
            self.pc = self.return_addr();
        } else {
        }
    }

    // Exchange DE and HL
    fn xchg(&mut self) {
        let d = self.regs.D;
        let h = self.regs.H;

        self.regs.D = self.regs.E;
        self.regs.E = d;

        self.regs.H = self.regs.L;
        self.regs.L = h;
    }

    // Call if parity is set
    fn cpe(&mut self) {
        if self.regs.PSW.P {
            let addr = self.return_addr();
            self.call(addr)
        } else {
        }
    }

    // XOR A with immediate
    fn xri(&mut self) {
        self.regs.A ^= self.fetch();
    }

    // Return is S not set
    fn rp(&mut self) {
        if !self.regs.PSW.S {
            self.ret();
        } else {
        }
    }

    // Jump if S is not set
    fn jp(&mut self) {
        if !self.regs.PSW.S {
            self.pc = self.return_addr();
        } else {
        }
    }

    // Disable interrupts
    fn di(&mut self) {}

    // Call if S not set
    fn cp(&mut self) {
        if !self.regs.PSW.S {
            let addr = self.return_addr();
            self.call(addr);
        }
    }

    // Call restart subroutine
    fn rst(&mut self, addr: u16) {
        self.call(addr)
    }

    // Compare reg A with immediate
    fn cpi(&mut self) {
        if self.regs.A - self.fetch() == 0 {
            // parity flag
        } else {
            // parity flag
        }
    }

    // Call if S is set
    fn cm(&mut self) {
        if self.regs.PSW.S {
            let addr = self.return_addr();
            self.call(addr);
        }
    }

    // Enable interrupts
    fn ei(&mut self) {}

    // Jump if S is set
    fn jm(&mut self) {
        if self.regs.PSW.S {
        self.pc = self.return_addr();
        } else {

        }
    }

    // Load SP from HL
    // SP = HL
    fn sphl(&mut self) {
        self.sp = self.regs.return_joined_regs("HL");
    }

    // Return is s is set
    fn rm(&mut self) {
        if self.regs.PSW.S {
            self.ret();
        } else {
        }
    }

    // OR A with immediate
    fn ori(&mut self) {
        self.regs.A |= self.fetch();
    }

    // Push register pair to stack
    fn push(&mut self, reg: &str) {
        match reg {
            "BC" => self.sp = self.regs.return_joined_regs("BC"),
            "DE" => self.sp = self.regs.return_joined_regs("DE"),
            "HL" => self.sp = self.regs.return_joined_regs("HL"),
            "PSW" => {}
            _ => {}
        }
    }

    fn return_addr(&mut self) -> u16 {
        let lo = self.fetch();
        let hi = self.fetch();

        let addr = (hi as u16) << 8 | (lo as u16);

        return addr;
    }

    fn return_byte_at_location(&mut self, reg_pair: &str) -> u8 {
        let addr = self.regs.return_joined_regs(reg_pair);

        return self.ram[addr as usize];
    }
}
