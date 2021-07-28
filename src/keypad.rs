const KEYS_NUM: usize = 16;

pub struct Keypad {
    keys: [bool; KEYS_NUM],
}

pub enum Key {
    KeyX = 0,
    Key1 = 1,
    Key2 = 2,
    Key3 = 3,
    KeyQ = 4,
    KeyW = 5,
    KeyE = 6,
    KeyA = 7,
    KeyS = 8,
    KeyD = 9,
    KeyZ = 10,
    KeyC = 11,
    Key4 = 12,
    KeyR = 13,
    KeyF = 14,
    KeyV = 15,
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

    pub fn set_pressed(&mut self, k: Key) {
        self.keys[k as usize] = true;
    }

    pub fn set_unpressed(&mut self, k: Key) {
        self.keys[k as usize] = false;
    }
}
