#![allow(non_snake_case)]

use std::array::IntoIter;

use crate::keypad::Keypad;
use crate::memory::Memory;
use crate::random::Generator;
use crate::registers::Registers;
use crate::stack::Stack;
use crate::video::{Video, VIDEO_HEIGHT, VIDEO_WIDTH};

pub struct Chip {
    memory: Memory,
    rand_gen: Generator<u8>,
    video: Video,
    stack: Stack,
    registers: Registers,
    keypad: Keypad,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            memory: Memory::new(),
            rand_gen: Generator::new(),
            video: Video::new(),
            stack: Stack::new(),
            registers: Registers::new(),
            keypad: Keypad::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn get_keypad(&mut self) -> &mut Keypad {
        &mut self.keypad
    }

    pub fn get_video(&mut self) -> &mut Video {
        &mut self.video
    }

    pub fn load_rom(&mut self, filename: &str) {
        self.memory.load_rom(filename)
    }

    pub fn cycle(&mut self) {
        let opcode = self.memory.fetch();

        self.execute_instruction(opcode);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn execute_instruction(&mut self, opcode: u16) {
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
            opcode if opcode & 0xF00F == 0x8006 => self.op_8xy6(decode_fst(opcode)),
            opcode if opcode & 0xF00F == 0x8007 => {
                self.op_8xy7(decode_fst(opcode), decode_snd(opcode))
            }
            opcode if opcode & 0xF00F == 0x800E => self.op_8xyE(decode_fst(opcode)),
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
            opcode => eprintln!("Unknown instruction (skipping): {:#06x}", opcode),
        }
    }

    fn op_00E0(&mut self) {
        self.video.clear();
    }

    fn op_00EE(&mut self) {
        self.memory.set_address(self.stack.pop());
    }

    fn op_1nnn(&mut self, addr: u16) {
        self.memory.set_address(addr)
    }

    fn op_2nnn(&mut self, addr: u16) {
        self.stack.push(self.memory.get_pointer());
        self.memory.set_address(addr);
    }

    fn op_3xkk(&mut self, reg: u8, byte: u8) {
        if self.registers.read(reg) == byte {
            self.memory.advance(2);
        }
    }

    fn op_4xkk(&mut self, reg: u8, byte: u8) {
        if self.registers.read(reg) != byte {
            self.memory.advance(2);
        }
    }

    fn op_5xy0(&mut self, reg_1: u8, reg_2: u8) {
        if self.registers.read(reg_1) == self.registers.read(reg_2) {
            self.memory.advance(2);
        }
    }

    fn op_6xkk(&mut self, reg: u8, val: u8) {
        self.registers.set(reg, val);
    }

    fn op_7xkk(&mut self, reg: u8, val: u8) {
        let value = self.registers.read(reg);
        self.registers.set(reg, value.wrapping_add(val));
    }

    fn op_8xy0(&mut self, reg_1: u8, reg_2: u8) {
        self.registers.set(reg_1, self.registers.read(reg_2));
    }

    fn op_8xy1(&mut self, reg_1: u8, reg_2: u8) {
        self.registers.set(reg_1, self.registers.or(reg_1, reg_2));
    }

    fn op_8xy2(&mut self, reg_1: u8, reg_2: u8) {
        self.registers.set(reg_1, self.registers.and(reg_1, reg_2));
    }

    fn op_8xy3(&mut self, reg_1: u8, reg_2: u8) {
        self.registers.set(reg_1, self.registers.xor(reg_1, reg_2));
    }

    fn op_8xy4(&mut self, reg_1: u8, reg_2: u8) {
        self.registers.add_inplace(reg_1, reg_2);
    }

    fn op_8xy5(&mut self, lhs: u8, rhs: u8) {
        self.registers.sub_inplace(lhs, rhs);
    }

    fn op_8xy6(&mut self, reg: u8) {
        self.registers.shitf_right_inplace(reg);
    }

    fn op_8xy7(&mut self, reg_1: u8, reg_2: u8) {
        self.registers.sub_n(reg_1, reg_2);
    }

    fn op_8xyE(&mut self, reg: u8) {
        self.registers.shift_left_inplace(reg);
    }

    fn op_9xy0(&mut self, fst: u8, snd: u8) {
        if self.registers.read(fst) != self.registers.read(snd) {
            self.memory.advance(2);
        }
    }

    fn op_Annn(&mut self, address: u16) {
        self.memory.index_register = address;
    }

    fn op_Bnnn(&mut self, address: u16) {
        self.memory.advance(self.registers.read(0) as u16 + address);
    }

    fn op_Cxkk(&mut self, fst: u8, snd: u8) {
        self.registers.set(fst, self.rand_gen.get_random() & snd);
    }

    fn op_Dxyn(&mut self, vx: u8, vy: u8, height: u8) {
        let from = self.memory.index_register as usize;
        let sprite = self.memory.slice(from, from + height as usize);
        let x_pos = self.registers.read(vx) as usize;
        let y_pos = self.registers.read(vy) as usize;

        self.registers.vx_set(0);

        for (row, byte) in sprite.iter().enumerate() {
            for (col, sprite_pixel) in byte_to_enumeration(*byte) {
                let screen_pixel = self
                    .video
                    .pixel((y_pos + row) % VIDEO_HEIGHT, (x_pos + col) % VIDEO_WIDTH);
                if sprite_pixel != 0 {
                    if *screen_pixel == 0xFFFFFFFF {
                        self.registers.vx_set(1);
                    }
                    *screen_pixel ^= 0xFFFFFFFF;
                }
            }
        }
    }

    fn op_Ex9E(&mut self, key: u8) {
        if self.keypad.is_pressed(key) {
            self.memory.advance(2);
        }
    }

    fn op_ExA1(&mut self, key: u8) {
        if !self.keypad.is_pressed(key) {
            self.memory.advance(2);
        }
    }

    fn op_Fx07(&mut self, reg: u8) {
        self.registers.set(reg, self.delay_timer);
    }

    fn op_Fx0A(&mut self, reg: u8) {
        if let Some(key) = self.keypad.get_pressed() {
            self.registers.set(reg, key as u8);
        } else {
            self.memory.go_back(2);
        }
    }

    fn op_Fx15(&mut self, instruction: u8) {
        self.delay_timer = instruction;
    }

    fn op_Fx18(&mut self, reg: u8) {
        self.sound_timer = self.registers.read(reg)
    }

    fn op_Fx1E(&mut self, reg: u8) {
        self.memory.index_register += self.registers.read(reg) as u16;
    }

    fn op_Fx29(&mut self, font_no: u8) {
        self.memory.set_index_register_to_font_no(font_no);
    }

    fn op_Fx33(&mut self, n: u8) {
        self.memory.store_bcd_repr(self.registers.read(n));
    }

    fn op_Fx55(&mut self, val: u8) {
        self.memory
            .copy_from(self.registers.slice(0, (val + 1) as usize));
    }

    fn op_Fx65(&mut self, val: u8) {
        self.registers.copy_from(self.memory.slice(
            self.memory.index_register as usize,
            self.memory.index_register as usize + (val + 1) as usize,
        ));
    }
}

fn byte_to_enumeration(b: u8) -> impl Iterator<Item = (usize, u8)> {
    IntoIter::new([
        b & 0b10000000,
        b & 0b01000000,
        b & 0b00100000,
        b & 0b00010000,
        b & 0b00001000,
        b & 0b00000100,
        b & 0b00000010,
        b & 0b00000001,
    ])
    .enumerate()
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
