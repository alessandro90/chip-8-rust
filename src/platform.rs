extern crate sdl2;
use crate::video::{VIDEO_HEIGHT, VIDEO_WIDTH};
use sdl2::{
    pixels::PixelFormatEnum,
    render::{Canvas, Texture, TextureCreator},
    video::Window,
    video::WindowContext,
};

pub struct Platform {
    // context: Sdl,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
}

impl Platform {
    pub fn new() -> Platform {
        let width = (10 * VIDEO_WIDTH) as u32;
        let height = (10 * VIDEO_HEIGHT) as u32;
        // Looks like this is the rendering object
        // use .copy and .present should suffice
        // canvas.copy(&texture, None, None);
        // texture.update(None, /* video.buf */, pitch)

        let context = sdl2::init().unwrap();
        let window = context
            .video()
            .unwrap()
            .window("Chip-8 Emulator", width, height)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Platform {
            // context,
            canvas,
            texture_creator,
        }
    }

    pub fn get_texture<'b>(&'b mut self) -> Texture<'b> {
        self.texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGBA8888,
                VIDEO_WIDTH as u32,
                VIDEO_HEIGHT as u32,
            )
            .unwrap()
    }

    pub fn update_texture<'a>(
        &mut self,
        texture: &mut Texture<'a>,
        buffer: &[u8],
        pitch: usize,
    ) -> Result<(), String> {
        texture
            .update(None, buffer, pitch)
            .map_err(|_| String::from("Texture Update error"))?;
        self.canvas.clear();
        self.canvas.copy(texture, None, None)?;
        self.canvas.present();
        Ok(())
    }
}
