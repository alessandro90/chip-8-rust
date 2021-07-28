mod chip;
mod keypad;
mod memory;
mod random;
mod registers;
mod stack;
mod video;

extern crate sdl2;

use std::{thread, time::Duration};

use chip::Chip;
use sdl2::{
    pixels::PixelFormatEnum,
    render::{Canvas, Texture},
    video::Window,
    Sdl,
};

use crate::keypad::{Key, Keypad};
use sdl2::{event::Event, keyboard::Keycode, EventPump};

use video::{PITCH, VIDEO_HEIGHT, VIDEO_WIDTH};

fn make_canvas(context: &Sdl) -> Canvas<Window> {
    let width = (10 * VIDEO_WIDTH) as u32;
    let height = (10 * VIDEO_HEIGHT) as u32;

    let window = context
        .video()
        .unwrap()
        .window("Chip-8 Emulator", width, height)
        .position_centered()
        .build()
        .unwrap();
    window.into_canvas().build().unwrap()
}

fn update_texture<'a>(
    texture: &mut Texture<'a>,
    buffer: &[u32],
    pitch: usize,
    canvas: &mut Canvas<Window>,
) -> Result<(), String> {
    unsafe {
        let (_, bytes, _) = buffer.align_to::<u8>();
        texture
            .update(None, bytes, pitch)
            .map_err(|_| String::from("Texture Update error"))?;
    }
    canvas.clear();
    canvas.copy(texture, None, None)?;
    canvas.present();
    Ok(())
}

fn process_input(keypad: &mut Keypad, event_pump: &mut EventPump) -> bool {
    let mut quit = false;
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                quit = true;
            }
            Event::KeyDown {
                keycode: Some(k), ..
            } => match k {
                Keycode::Escape => {
                    quit = true;
                }
                Keycode::X => keypad.set_pressed(Key::KeyX),
                Keycode::Num1 => keypad.set_pressed(Key::Key1),
                Keycode::Num2 => keypad.set_pressed(Key::Key2),
                Keycode::Num3 => keypad.set_pressed(Key::Key3),
                Keycode::Q => keypad.set_pressed(Key::KeyQ),
                Keycode::W => keypad.set_pressed(Key::KeyW),
                Keycode::E => keypad.set_pressed(Key::KeyE),
                Keycode::A => keypad.set_pressed(Key::KeyA),
                Keycode::S => keypad.set_pressed(Key::KeyS),
                Keycode::D => keypad.set_pressed(Key::KeyD),
                Keycode::Z => keypad.set_pressed(Key::KeyZ),
                Keycode::C => keypad.set_pressed(Key::KeyC),
                Keycode::Num4 => keypad.set_pressed(Key::Key4),
                Keycode::R => keypad.set_pressed(Key::KeyR),
                Keycode::F => keypad.set_pressed(Key::KeyF),
                Keycode::V => keypad.set_pressed(Key::KeyV),
                _ => (),
            },

            Event::KeyUp {
                keycode: Some(k), ..
            } => match k {
                Keycode::X => keypad.set_unpressed(Key::KeyX),
                Keycode::Num1 => keypad.set_unpressed(Key::Key1),
                Keycode::Num2 => keypad.set_unpressed(Key::Key2),
                Keycode::Num3 => keypad.set_unpressed(Key::Key3),
                Keycode::Q => keypad.set_unpressed(Key::KeyQ),
                Keycode::W => keypad.set_unpressed(Key::KeyW),
                Keycode::E => keypad.set_unpressed(Key::KeyE),
                Keycode::A => keypad.set_unpressed(Key::KeyA),
                Keycode::S => keypad.set_unpressed(Key::KeyS),
                Keycode::D => keypad.set_unpressed(Key::KeyD),
                Keycode::Z => keypad.set_unpressed(Key::KeyZ),
                Keycode::C => keypad.set_unpressed(Key::KeyC),
                Keycode::Num4 => keypad.set_unpressed(Key::Key4),
                Keycode::R => keypad.set_unpressed(Key::KeyR),
                Keycode::F => keypad.set_unpressed(Key::KeyF),
                Keycode::V => keypad.set_unpressed(Key::KeyV),
                _ => (),
            },
            _ => (),
        }
    }
    quit
}

pub fn run(args: Vec<String>) {
    let mut chip8 = Chip::new();
    chip8.load_rom(&args[1]);
    let context = sdl2::init().unwrap();
    let mut canvas = make_canvas(&context);
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGBA8888,
            VIDEO_WIDTH as u32,
            VIDEO_HEIGHT as u32,
        )
        .unwrap();

    let mut event_pump = context.event_pump().unwrap();

    loop {
        if process_input(chip8.get_keypad(), &mut event_pump) {
            break;
        }
        chip8.cycle();
        update_texture(&mut texture, chip8.get_video().buffer(), PITCH, &mut canvas).unwrap();
        thread::sleep(Duration::from_millis(10u64));
    }
}
