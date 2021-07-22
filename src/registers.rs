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
        let sum = self.regs[lhs as usize] as u16 + self.regs[rhs as usize] as u16;
        if sum > 255u16 {
            self.vx_set(1);
        } else {
            self.vx_set(0);
        }
        self.regs[lhs as usize] = (sum & 0xFF) as u8;
    }

    pub fn sub_inplace(&mut self, lhs: u8, rhs: u8) {
        let vx = self.regs[lhs as usize];
        let vy = self.regs[rhs as usize];
        if vx > vy {
            self.vx_set(1);
        } else {
            self.vx_set(0);
        }
        self.regs[lhs as usize] = u8::wrapping_sub(vx, vy);
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

    pub fn shitf_right_inplace(&mut self, reg: u8) {
        self.vx_set(self.regs[reg as usize] & 0x1);
        self.regs[reg as usize] >>= 1;
    }

    pub fn sub_n(&mut self, reg_1: u8, reg_2: u8) {
        if self.regs[reg_2 as usize] > self.regs[reg_1 as usize] {
            self.vx_set(1);
        } else {
            self.vx_set(0);
        }
        self.regs[reg_1 as usize] =
            u8::wrapping_sub(self.regs[reg_2 as usize], self.regs[reg_1 as usize]);
    }

    pub fn shift_left_inplace(&mut self, reg: u8) {
        self.vx_set(self.regs[reg as usize] & 0x80);
        self.regs[reg as usize] <<= 1;
    }

    pub fn vx_set(&mut self, val: u8) {
        self.regs[0xF] = val;
    }

    pub fn slice(&self, from: usize, to: usize) -> &[u8] {
        &self.regs[from..to]
    }

    pub fn copy_from(&mut self, src: &[u8]) {
        self.regs[0..src.len()].copy_from_slice(src);
    }
}
