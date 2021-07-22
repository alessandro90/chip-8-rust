mod chip;
mod keypad;
mod memory;
mod platform;
mod random;
mod registers;
mod stack;
mod video;

use std::{thread, time::Duration};

use chip::Chip;
use platform::Platform;

pub fn run() {
    let mut chip8 = Chip::new();
    let mut platform = Platform::new();
    let mut texture = platform.get_texture();

    loop {
        chip8.cycle();
        thread::sleep(Duration::from_millis(100u64));
    }
}
