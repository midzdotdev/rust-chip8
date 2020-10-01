use crate::ram::Ram;
use crate::chip8::PROGRAM_START;

// CPU has 16 8-bit registers, V0 to VF
// Also 16-bit register called I
// VF is a flag used by some instructions and shouldn't be modified directly

pub struct Cpu {
  vx: [u8; 16],
  i: u16,
  pc: u16,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      vx: [0; 16],
      i: 0,
      pc: PROGRAM_START,
    }
  }

  pub fn run_instruction(&mut self, ram: &mut Ram) {

  } 
}