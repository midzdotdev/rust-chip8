extern crate minifb;

use minifb::{Key, WindowOptions, Window};
use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use display::Display;

mod chip8;
mod bus;
mod cpu;
mod display;
mod keyboard;
mod ram;

fn main() {
  let mut file = File::open("roms/INVADERS").unwrap();
  let mut data = Vec::<u8>::new();
  file.read_to_end(&mut data);

  let mut chip8 = Chip8::new();
  chip8.load_rom(&data);

  let WINDOW_WIDTH = 640;
  let WINDOW_HEIGHT = 320;

  let mut window = Window::new(
    "CHIP-8 Emulator",
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
    WindowOptions::default()
  ).unwrap();

  let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

  while window.is_open() && !window.is_key_down(Key::Escape) {
    chip8.run_instruction();

    let chip8_buffer = chip8.get_display_buffer();

    for y in 0..WINDOW_HEIGHT {
      for x in 0..WINDOW_WIDTH {
        let index = Display::get_index_from_coords(x / 10, y / 10);
        let pixel = chip8_buffer[index];
        let color_pixel = if pixel == 1 { 0xFFFFFFFF } else { 0 };

        buffer[y * WINDOW_WIDTH + x] = color_pixel;
      }
    }

    window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
  }
}
