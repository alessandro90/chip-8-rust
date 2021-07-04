mod chip;
mod memory;
mod random;
mod registers;
mod stack;
mod video;

use chip::Chip;

pub fn run() {
    let chip8 = Chip::new();
}
// struct Registers {}
// struct Stack {}
// struct DelayTimer {}
// struct SoundTimer {}
// struct Keypad {}
// struct Video {}
