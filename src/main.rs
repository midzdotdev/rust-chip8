extern crate minifb;

use minifb::{Key, WindowOptions, Window, KeyRepeat, Scale, ScaleMode};
use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use display::{Display, WIDTH, HEIGHT};
use std::time::{Duration, Instant};

mod chip8;
mod bus;
mod cpu;
mod display;
mod keyboard;
mod ram;

fn main() {
  let mut file = File::open("roms/MISSILE").unwrap();
  let mut data = Vec::<u8>::new();
  file.read_to_end(&mut data);

  let mut chip8 = Chip8::new();
  chip8.load_rom(&data);

  let mut window = Window::new(
    "CHIP-8 Emulator",
    640,
    320,
    WindowOptions {
      borderless: false,
      title: true,
      resize: false,
      scale: Scale::X2,
      scale_mode: ScaleMode::AspectRatioStretch,
      topmost: false,
      transparency: false,
    },
  ).unwrap();

  let mut buffer: Vec<u32> = vec![0; HEIGHT * WIDTH];

  let mut last_key_update_time = Instant::now();
  let mut last_instruction_run_time = Instant::now();
  let mut last_display_time = Instant::now();

  while window.is_open() && !window.is_key_down(Key::Escape) {
    let keys_pressed = window.get_keys_pressed(KeyRepeat::Yes);
    let key = match keys_pressed {
      Some(keys) => if !keys.is_empty() { Some(keys[0]) } else { None },
      None => None,
    };

    let chip8_key = get_chip8_keycode_for(key);
    if chip8_key.is_some() || Instant::now() - last_key_update_time >= Duration::from_millis(200) {
      chip8.set_key_pressed(chip8_key);
      last_key_update_time = Instant::now();
    }

    if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
      chip8.run_instruction();
      last_instruction_run_time = Instant::now();
    }

    if Instant::now() - last_display_time > Duration::from_millis(10) {
      let chip8_buffer = chip8.get_display_buffer();

      for y in 0..HEIGHT {
        for x in 0..WIDTH {
          let index = Display::get_index_from_coords(x, y);
          let pixel = chip8_buffer[index];
          let color_pixel = if pixel == 1 { 0xFFFFFF } else { 0 };

          buffer[y * WIDTH + x] = color_pixel;
        }
      }

      window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
      last_display_time = Instant::now();
    }
  }
}

fn get_chip8_keycode_for(key: Option<Key>) -> Option<u8> {
  match key {
    Some(Key::Key7) => Some(0x1),
    Some(Key::Key8) => Some(0x2),
    Some(Key::Key9) => Some(0x3),
    Some(Key::Key0) => Some(0xC),

    Some(Key::U) => Some(0x4),
    Some(Key::I) => Some(0x5),
    Some(Key::O) => Some(0x6),
    Some(Key::P) => Some(0xD),

    Some(Key::J) => Some(0x7),
    Some(Key::K) => Some(0x8),
    Some(Key::L) => Some(0x9),
    Some(Key::Semicolon) => Some(0xE),

    Some(Key::M) => Some(0xA),
    Some(Key::Comma) => Some(0x0),
    Some(Key::Period) => Some(0xB),
    Some(Key::Slash) => Some(0xF),

    _ => None,
  }
}
