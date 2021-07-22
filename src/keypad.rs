const KEYS_NUM: usize = 16;

pub struct Keypad {
    keys: [bool; KEYS_NUM],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [false; KEYS_NUM],
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn get_pressed(&self) -> Option<usize> {
        self.keys
            .iter()
            .enumerate()
            .find(|(_, k)| **k)
            .map(|(i, _)| i)
    }
}
