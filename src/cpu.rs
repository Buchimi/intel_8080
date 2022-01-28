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
            0x02 => {self.stax()},
            0x03 => {self.inx()},
            0x04 => {self.inr()},
            0x05 => {self.dcr()},
            0x06 => {self.mvi()},
            0x07 => {self.rlc()},
            0x09 => {self.dad()},
            0x0A => {self.ldax()},
            0x0B => {self.dcx()},
            0x0C => {self.inr()},
            0x0D => {self.dcr()},
            0x0E => {self.mvi()},
            0x0F => {self.rrc()},
            
            0x11 => {self.lxi()},
            0x12 => {self.stax()},
            0x13 => {self.inx()},
            0x14 => {self.inr()},
            0x15 => {self.dcr()},
            0x16 => {self.mvi()},
            0x17 => {self.ral()},
            0x19 => {self.dad()},
            0x1A => {self.ldax()},
            0x1B => {self.dcx()},
            0x1C => {self.inr()},
            0x1D => {self.dcr()},
            0x1E => {self.mvi()},
            0x1F => {self.rar()},

            0x21 => {self.lxi()},
            0x22 => {self.shld()},
            0x23 => {self.inx()},
            0x24 => {self.inr()},
            0x25 => {self.dcr()},
            0x26 => {self.mvi()},
            0x27 => {self.daa()},
            0x29 => {self.dad()},
            0x2A => {self.lhld()},
            0x2B => {self.dcx()},
            0x2C => {self.inr()},
            0x2D => {self.dcr()},
            0x2E => {self.mvi()},
            0x2F => {self.cma()},

            0x31 => {self.lxi()},
            0x32 => {self.sta()},
            0x33 => {self.inx()},
            0x34 => {self.inr()},
            0x35 => {self.dcr()},
            0x36 => {self.mvi()},
            0x37 => {self.stc()},
            0x39 => {self.dad()},
            0x3A => {self.lda()},
            0x3B => {self.dcx()},
            0x3C => {self.inr()},
            0x3D => {self.dcr()},
            0x3E => {self.mvi()},
            0x3F => {self.cmc()},

            0x40 => {self.mov()},
            0x41 => {self.mov()},
            0x42 => {self.mov()},
            0x43 => {self.mov()},
            0x44 => {self.mov()},
            0x45 => {self.mov()},
            0x46 => {self.mov()},
            0x47 => {self.mov()},
            0x48 => {self.mov()},
            0x49 => {self.mov()},
            0x4A => {self.mov()},
            0x4B => {self.mov()},
            0x4C => {self.mov()},
            0x4D => {self.mov()},
            0x4E => {self.mov()},
            0x4F => {self.mov()},
           
            0x50 => {self.mov()},
            0x51 => {self.mov()},
            0x52 => {self.mov()},
            0x53 => {self.mov()},
            0x54 => {self.mov()},
            0x55 => {self.mov()},
            0x56 => {self.mov()},
            0x57 => {self.mov()},
            0x58 => {self.mov()},
            0x59 => {self.mov()},
            0x5A => {self.mov()},
            0x5B => {self.mov()},
            0x5C => {self.mov()},
            0x5D => {self.mov()},
            0x5E => {self.mov()},
            0x5F => {self.mov()},

            0x60 => {self.mov()},
            0x61 => {self.mov()},
            0x62 => {self.mov()},
            0x63 => {self.mov()},
            0x64 => {self.mov()},
            0x65 => {self.mov()},
            0x66 => {self.mov()},
            0x67 => {self.mov()},
            0x68 => {self.mov()},
            0x69 => {self.mov()},
            0x6A => {self.mov()},
            0x6B => {self.mov()},
            0x6C => {self.mov()},
            0x6D => {self.mov()},
            0x6E => {self.mov()},
            0x6F => {self.mov()},

            0x70 => {self.mov()},
            0x71 => {self.mov()},
            0x72 => {self.mov()},
            0x73 => {self.mov()},
            0x74 => {self.mov()},
            0x75 => {self.mov()},
            0x76 => {self.hlt()},
            0x77 => {self.mov()},
            0x78 => {self.mov()},
            0x79 => {self.mov()},
            0x7A => {self.mov()},
            0x7B => {self.mov()},
            0x7C => {self.mov()},
            0x7D => {self.mov()},
            0x7E => {self.mov()},
            0x7F => {self.mov()},

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
    }

    // Increment register
    fn inr(&mut self, reg: &str) {
        match reg {
            "B" => {self.regs.B += 1},
            "C" => {self.regs.C += 1},
            "D" => {self.regs.D += 1},
            "E" => {self.regs.E += 1},
            "H" => {self.regs.H += 1},
            "L" => {self.regs.L += 1},
            _ => {}
        };
    }

    // Decrement register
    fn dcr(&mut self, reg: &str) {
        match reg {
            "B" => {self.regs.B -= 1},
            "C" => {self.regs.C -= 1},
            "D" => {self.regs.D -= 1},
            "E" => {self.regs.E -= 1},
            "H" => {self.regs.H -= 1},
            "L" => {self.regs.L -= 1},
            _ => {}
        };
    }

    fn mvi(&mut self) {
        
    }

    fn rlc(&mut self) {

    }

    fn dad(&mut self) {

    }

    fn ldax(&mut self) {

    }

    fn dcx(&mut self) {

    }

    fn rrc(&mut self) {

    }

    fn ral(&mut self) {

    }


    fn rar(&mut self) {

    }

    fn shld(&mut self) {

    }

    fn daa(&mut self) {

    }

    fn lhld(&mut self) {

    }

    fn cma(&mut self) {

    }

    fn sta(&mut self) {

    }

    fn stc(&mut self) {

    }

    fn lda(&mut self) {
        
    }

    fn cmc(&mut self) {

    }

    fn mov(&mut self) {

    }

    fn hlt(&mut self) {

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



}
