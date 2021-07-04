const VIDEO_SIZE: usize = 64 * 32;

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
}
