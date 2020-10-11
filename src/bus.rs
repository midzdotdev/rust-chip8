use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::ram::Ram;

pub struct Bus {
  display: Display,
  keyboard: Keyboard,
  ram: Ram,
  delay_timer: u8,
}

impl Bus {
  pub fn new() -> Bus {
    Bus {
      display: Display::new(),
      keyboard: Keyboard::new(),
      ram: Ram::new(),
      delay_timer: 0,
    }
  }

  pub fn display_clear(&mut self) {
    self.display.clear();
  }

  pub fn display_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
    return self.display.draw_byte(byte, x, y);
  }

  pub fn ram_read_byte (&self, address: u16) -> u8 {
    return self.ram.read_byte(address);
  }

  pub fn ram_write_byte(&mut self, address: u16, value: u8) {
    self.ram.write_byte(address, value);
  }

  pub fn key_pressed(&self, key_code: u8) -> bool {
    return self.keyboard.key_pressed(key_code);
  }

  pub fn set_delay_timer(&mut self, value: u8) {
    self.delay_timer = value;
  }

  pub fn tick(&mut self) {
    if self.delay_timer > 0 {
      self.delay_timer -= 1;
    }
  }

  pub fn get_delay_timer(&self) -> u8 {
    return self.delay_timer;
  }

  pub fn get_display_buffer(&self) -> &[u8] {
    return self.display.get_display_buffer();
  }
}