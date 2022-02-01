#[derive(Debug)]
pub struct Regs {
    pub A: u8,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,
    pub PSW: Flags,
}

#[derive(Debug)]
pub struct Flags {
    pub C: bool,
    pub V: bool,
    pub P: bool,
    pub A: bool,
    pub K: bool,
    pub Z: bool,
    pub S: bool,
}

impl Regs {
    pub fn init_regs() -> Self {
        Regs {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            H: 0,
            L: 0,
            PSW: Flags {
                C: false,
                V: false,
                P: false,
                A: false,
                K: false,
                Z: false,
                S: false,
            },
        }
    }

    // Registers BC, DE and HL are sometimes read together so we must join them and return the value
    // The values in the registers don't change, we just do some bitwise ops to return the correct value
    pub fn return_joined_regs(&mut self, reg_pair: &str) -> u16 {
        match reg_pair {
            "BC" => return self.join_regs(self.B, self.C),
            "DE" => return self.join_regs(self.D, self.E),
            "HL" => return self.join_regs(self.H, self.L),
            _ => 0,
        }
    }

    // Left shift first reg to create a u16
    // Bitwise OR to join the 2 and return
    fn join_regs(&mut self, a: u8, b: u8) -> u16 {
        let mut val: u16 = 0;

        let hi = (a as u16) << 8;

        val = hi | (b as u16);

        return val;
    }

    // Take the u16 value of 2 regs
    // Shift the hi byte and turn it into a u8
    // Bitwise & the lo byte to leave a u8
    // Create a tuple of both values
    pub fn split_regs(&mut self, x: u16) -> (u8, u8) {
        let hi = (x >> 8) as u8;
        let lo = (x & 0x0F) as u8;

        let regs = (hi, lo);

        return regs;
    }

    pub fn set_flags(&mut self, flag: &str) {
        match flag {
            "C" => self.PSW.C = true,
            "V" => self.PSW.V = true,
            "P" => self.PSW.P = true,
            "A" => self.PSW.A = true,
            "K" => self.PSW.K = true,
            "Z" => self.PSW.Z = true,
            "S" => self.PSW.S = true,
            _ => {}
        }
    }
}
