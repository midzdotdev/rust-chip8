use crate::bus::Bus;
use crate::cpu::Cpu;

pub const PROGRAM_START: u16 = 0x200;

pub struct Chip8 {
  bus: Bus,
  cpu: Cpu,
}

impl Chip8 {
  pub fn new() -> Chip8 {
    Chip8 {
      bus: Bus::new(),
      cpu: Cpu::new(),
    }
  }

  pub fn load_rom(&mut self, data: &Vec<u8>) {
    for i in 0..data.len() {
      let address = PROGRAM_START + i as u16;
      self.bus.ram_write_byte(address, data[i]);
    }
  }

  pub fn run_instruction(&mut self) {
    self.bus.tick();
    self.cpu.run_instruction(&mut self.bus);
    self.bus.display_present();
  }
}
