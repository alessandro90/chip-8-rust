const STACK_SIZE: usize = 16;

pub struct Stack {
    buf: [u16; STACK_SIZE],
    sp: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            buf: [0u16; STACK_SIZE],
            sp: 0usize,
        }
    }

    pub fn top(&self) -> u16 {
        self.buf[self.sp]
    }

    pub fn push(&mut self, address: u16) {
        self.buf[self.sp] = address;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.top()
    }
}
