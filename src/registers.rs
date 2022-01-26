#[derive(Debug)]
pub struct Regs {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    H: u8,
    L: u8,
    F: u8,
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
            F: 0,
        }
    }

    // Registers BC, DE and HL are sometimes read together so we must join them and return the value.
    // The values in the registers don't change, we just do some bitwise ops to return the correct value.
    pub fn return_joined_regs(&mut self, reg_pair: &str) -> u16 {
        match reg_pair {
            "BC" => return self.join_regs(self.B, self.C),
            "DE" => return self.join_regs(self.D, self.E),
            "HL" => return self.join_regs(self.H, self.L),
            _ => 0,
        }
    }

    fn join_regs(&mut self, a: u8, b: u8) -> u16 {
        let mut val: u16 = 0;

        let hi = (a as u16) << 8;

        val = hi | (b as u16);

        return val;
    }
}
