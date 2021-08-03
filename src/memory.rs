use std::{fs, usize};
const MEMORY_SIZE: usize = 4096;
const START_ADDRESS: usize = 0x200;
const FONTSET_START_ADDRESS: usize = 0x50;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Memory {
    buf: [u8; MEMORY_SIZE],
    pc: u16,
    pub index_register: u16,
}

impl Memory {
    pub fn new() -> Memory {
        let pc = START_ADDRESS as u16; // First instruction to be executed
        let mut buf: [u8; MEMORY_SIZE] = [0u8; MEMORY_SIZE];
        let font_span = FONTSET_START_ADDRESS..FONTSET_START_ADDRESS + FONTSET.len();
        buf[font_span].copy_from_slice(&FONTSET);
        Memory {
            buf,
            pc,
            index_register: 0,
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let opcode = ((self.buf[self.pc as usize] as u16) << 8u16)
            | (self.buf[(self.pc + 1) as usize] as u16);
        self.advance(2);
        opcode
    }

    // TODO: move out of module this function
    pub fn load_rom(&mut self, filename: &str) {
        let rom_data = fs::read(filename).expect("Error loading ROM file");
        self.load_instructions(&rom_data);
    }

    pub fn load_instructions(&mut self, data: &[u8]) {
        if data.len() > self.buf.len() - START_ADDRESS {
            panic!("ROM file exeeds max allowed size")
        }

        self.buf[START_ADDRESS..START_ADDRESS + data.len()].copy_from_slice(&data);
    }

    pub fn set_address(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub fn get_pointer(&self) -> u16 {
        self.pc
    }

    pub fn advance(&mut self, n: u16) {
        self.pc += n;
    }

    pub fn go_back(&mut self, n: u16) {
        self.pc -= n;
    }

    pub fn slice(&self, from: usize, to: usize) -> &[u8] {
        &self.buf[from..to]
    }

    pub fn set_index_register_to_font_no(&mut self, font_no: u8) {
        self.index_register = (FONTSET_START_ADDRESS + (5 * (font_no as usize))) as u16;
    }

    pub fn store_bcd_repr(&mut self, n: u8) {
        self.buf[self.index_register as usize + 2] = n % 10;
        let n = n / 10;
        self.buf[self.index_register as usize + 1] = n % 10;
        let n = n / 10;
        self.buf[self.index_register as usize] = n % 10;
    }

    pub fn copy_from(&mut self, src: &[u8]) {
        self.buf[self.index_register as usize..self.index_register as usize + src.len()]
            .copy_from_slice(src);
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn memory_build() {
//         let memory = Memory::new();
//         assert_eq!(memory.pc as usize, START_ADDRESS);
//         assert_eq!(
//             memory.buf[..FONTSET_START_ADDRESS],
//             [0; FONTSET_START_ADDRESS]
//         );
//         assert_eq!(memory.fonts(), FONTSET);
//         assert_eq!(
//             memory.buf[FONTSET_START_ADDRESS + FONTSET.len()..],
//             [0; MEMORY_SIZE - (FONTSET_START_ADDRESS + FONTSET.len())]
//         );
//     }

//     #[test]
//     fn load_instructions_test() {
//         let mut memory = Memory::new();
//         let instructions = vec![1u8, 10, 6, 7, 99, 3, 4];
//         memory.load_instructions(&instructions);
//         assert_eq!(memory.instructions()[..instructions.len()], instructions);
//     }

//     #[test]
//     #[should_panic]
//     fn load_too_much_instructions() {
//         let mut memory = Memory::new();
//         let instructions = [0u8; MEMORY_SIZE - START_ADDRESS + 1];
//         memory.load_instructions(&instructions);
//     }
// }
