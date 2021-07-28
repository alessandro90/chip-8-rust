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

    fn top(&self) -> u16 {
        self.buf[self.sp - 1]
    }

    pub fn push(&mut self, address: u16) {
        self.buf[self.sp] = address;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        let v = self.top();
        self.sp -= 1;
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn empty_test_top() {
        let stack = Stack::new();
        stack.top();
    }

    #[test]
    fn empty_push() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.top(), 1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
    }

    #[test]
    fn stack_pop() {
        let mut stack = Stack::new();
        for i in 0..STACK_SIZE {
            stack.push(i as u16);
        }

        for i in 0..STACK_SIZE {
            let v = stack.pop();
            assert_eq!(v, (STACK_SIZE - i - 1) as u16);
        }
    }

    #[test]
    #[should_panic]
    fn stack_empty_pop() {
        let mut stack = Stack::new();
        stack.pop();
    }

    #[test]
    #[should_panic]
    fn stack_full_push() {
        let mut stack = Stack::new();
        for i in 0..STACK_SIZE {
            stack.push(i as u16);
        }
        stack.push(100);
    }
}
