pub const VIDEO_WIDTH: usize = 64;
pub const VIDEO_HEIGHT: usize = 32;
pub const PITCH: usize = 4 * VIDEO_WIDTH;
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

    pub fn buffer(&self) -> &[u32] {
        &self.buf[..]
    }

    pub fn pixel(&mut self, y: usize, x: usize) -> &mut u32 {
        &mut self.buf[y * VIDEO_WIDTH + x]
    }
}
