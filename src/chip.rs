use crate::memory::Memory;
use crate::random::Generator;
use crate::video::Video;

type InstructionExecutor = fn(&mut Chip, u16);

pub struct Chip {
    memory: Memory,
    rand_gen: Generator<u8>,
    video: Video,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            memory: Memory::new(),
            rand_gen: Generator::new(),
            video: Video::new(),
        }
    }

    pub fn execute_instruction(&mut self, opcode: u16) {
        match opcode {
            0x00E0 => self.op_00E0(),
            0x00EE => self.op_00EE(),
            opcode if opcode & 0xF000 == 0x1000 => self.op_1nnn(decode_all(opcode)),
            opcode if opcode & 0xF000 == 0x2000 => self.op_2nnn(decode_all(opcode)),
            opcode if opcode & 0xF000 == 0x3000 => {
                self.op_3xkk(decode_fst(opcode), decode_bytes(opcode))
            }
            opcode if opcode & 0xF000 == 0x4000 => {
                self.op_4xkk(decode_fst(opcode), decode_bytes(opcode))
            }
            opcode if opcode & 0xF00F == 0x5000 => {
                self.op_5xy0(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF000 == 0x6000 => {
                self.op_6xkk(decode_fst(opcode), decode_bytes(opcode))
            }
            opcode if opcode & 0xF000 == 0x7000 => {
                self.op_7xkk(decode_fst(opcode), decode_bytes(opcode))
            }
            opcode if opcode & 0xF00F == 0x8000 => {
                self.op_8xy0(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF00F == 0x8001 => {
                self.op_8xy1(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF00F == 0x8002 => {
                self.op_8xy2(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF00F == 0x8003 => {
                self.op_8xy3(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF00F == 0x8004 => {
                self.op_8xy4(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF00F == 0x8005 => {
                self.op_8xy5(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF00F == 0x8006 => {
                self.op_8xy6(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF00F == 0x8007 => {
                self.op_8xy7(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF00F == 0x800E => {
                self.op_8xyE(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF00F == 0x9000 => {
                self.op_9xy0(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF000 == 0xA000 => self.op_Annn(decode_all(opcode)),
            opcode if opcode & 0xF000 == 0xB000 => self.op_Bnnn(decode_all(opcode)),
            opcode if opcode & 0xF000 == 0xC000 => {
                self.op_Cxkk(decode_fst(opcode), decode_bytes(opcode))
            }
            opcode if opcode & 0xF000 == 0xD000 => {
                self.op_Dxyn(decode_fst(opcode), decode_snd(opcode), decode_thrd(opcode))
            }
            opcode if opcode & 0xF0FF == 0xE09E => self.op_Ex9E(decode_fst(opcode)),
            opcode if opcode & 0xF0FF == 0xE0A1 => self.op_ExA1(decode_fst(opcode)),
            opcode if opcode & 0xF0FF == 0xF007 => self.op_Fx07(decode_fst(opcode)),
            opcode if opcode & 0xF0FF == 0xF00A => self.op_Fx0A(decode_fst(opcode)),
            opcode if opcode & 0xF0FF == 0xF015 => self.op_Fx15(decode_fst(opcode)),
            opcode if opcode & 0xF0FF == 0xF018 => self.op_Fx18(decode_fst(opcode)),
            opcode if opcode & 0xF0FF == 0xF01E => self.op_Fx1E(decode_fst(opcode)),
            opcode if opcode & 0xF0FF == 0xF029 => self.op_Fx29(decode_fst(opcode)),
            opcode if opcode & 0xF0FF == 0xF033 => self.op_Fx33(decode_fst(opcode)),
            opcode if opcode & 0xF0FF == 0xF055 => self.op_Fx55(decode_fst(opcode)),
            opcode if opcode & 0xF0FF == 0xF065 => self.op_Fx65(decode_fst(opcode)),
            _ => (),
        }
    }

    #[allow(non_snake_case)]
    fn op_00E0(&mut self) {
        self.video.clear();
    }

    #[allow(non_snake_case)]
    fn op_00EE(&mut self) {}

    #[allow(non_snake_case)]
    fn op_1nnn(&mut self, instruction: u16) {}

    #[allow(non_snake_case)]
    fn op_2nnn(&mut self, instruction: u16) {}

    #[allow(non_snake_case)]
    fn op_3xkk(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_4xkk(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_5xy0(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_6xkk(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_7xkk(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_8xy0(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_8xy1(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_8xy2(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_8xy3(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_8xy4(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_8xy5(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_8xy6(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_8xy7(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_8xyE(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_9xy0(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_Annn(&mut self, instruction: u16) {}

    #[allow(non_snake_case)]
    fn op_Bnnn(&mut self, instruction: u16) {}

    #[allow(non_snake_case)]
    fn op_Cxkk(&mut self, fst: u8, snd: u8) {}

    #[allow(non_snake_case)]
    fn op_Dxyn(&mut self, fst: u8, snd: u8, thrd: u8) {}

    #[allow(non_snake_case)]
    fn op_Ex9E(&mut self, instruction: u8) {}

    #[allow(non_snake_case)]
    fn op_ExA1(&mut self, instruction: u8) {}

    #[allow(non_snake_case)]
    fn op_Fx07(&mut self, instruction: u8) {}

    #[allow(non_snake_case)]
    fn op_Fx0A(&mut self, instruction: u8) {}

    #[allow(non_snake_case)]
    fn op_Fx15(&mut self, instruction: u8) {}

    #[allow(non_snake_case)]
    fn op_Fx18(&mut self, instruction: u8) {}

    #[allow(non_snake_case)]
    fn op_Fx1E(&mut self, instruction: u8) {}

    #[allow(non_snake_case)]
    fn op_Fx29(&mut self, instruction: u8) {}

    #[allow(non_snake_case)]
    fn op_Fx33(&mut self, instruction: u8) {}

    #[allow(non_snake_case)]
    fn op_Fx55(&mut self, instruction: u8) {}

    #[allow(non_snake_case)]
    fn op_Fx65(&mut self, instruction: u8) {}
}

fn decode_fst(n: u16) -> u8 {
    ((n & 0x0F00) >> 8u16) as u8
}

fn decode_snd(n: u16) -> u8 {
    ((n & 0x00F0) >> 4u16) as u8
}

fn decode_thrd(n: u16) -> u8 {
    (n & 0x000F) as u8
}

fn decode_bytes(n: u16) -> u8 {
    (n & 0x00FF) as u8
}

fn decode_all(n: u16) -> u16 {
    n & 0x0FFF
}
