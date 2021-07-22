pub const VIDEO_WIDTH: usize = 32;
pub const VIDEO_HEIGHT: usize = 64;
const VIDEO_SIZE: usize = VIDEO_HEIGHT * VIDEO_WIDTH;

pub struct Video {
    buf: [u32; VIDEO_SIZE],
}

impl Video {
    pub fn new() -> Video {
        Video {
            buf: [0u32; VIDEO_SIZE],
        }
    }

    pub fn clear(&mut self) {
        self.buf.fill(0);
    }

    pub fn pixel(&mut self, x: usize, y: usize) -> &mut u32 {
        &mut self.buf[(y % VIDEO_HEIGHT) * VIDEO_WIDTH + x % VIDEO_WIDTH]
    }
}
