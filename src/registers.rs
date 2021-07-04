const REGISTERS_NUM: usize = 16;

pub struct Registers {
    regs: [u8; REGISTERS_NUM],
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            regs: [0u8; REGISTERS_NUM],
        }
    }

    pub fn read(&self, n: u8) -> u8 {
        self.regs[n as usize]
    }

    pub fn set(&mut self, i: u8, n: u8) {
        self.regs[i as usize] = n;
    }

    pub fn add_inplace(&mut self, lhs: u8, rhs: u8) {
        let sum = lhs as u16 + rhs as u16;
        if sum > 255u16 {
            self.vx_set(1);
        } else {
            self.vx_set(0);
        }
        self.regs[lhs as usize] = (sum & 0xFF) as u8;
    }

    pub fn sub_inplace(&mut self, lhs: u8, rhs: u8) {
        if lhs > rhs {
            self.vx_set(1);
        } else {
            self.vx_set(0);
        }
        self.regs[lhs as usize] = u8::wrapping_sub(lhs, rhs);
    }

    pub fn or(&self, reg_1: u8, reg_2: u8) -> u8 {
        self.regs[reg_1 as usize] | self.regs[reg_2 as usize]
    }

    pub fn and(&self, reg_1: u8, reg_2: u8) -> u8 {
        self.regs[reg_1 as usize] & self.regs[reg_2 as usize]
    }

    pub fn xor(&self, reg_1: u8, reg_2: u8) -> u8 {
        self.regs[reg_1 as usize] ^ self.regs[reg_2 as usize]
    }

    fn vx_set(&mut self, val: u8) {
        self.regs[0xF] = val;
    }
}
