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
    let address = self.pc;

    let hi = ram.read_byte(address) as u16;
    let lo = ram.read_byte(address+1) as u16;

    let instruction: u16 = (hi << 8) | lo;

    println!("Address: {:#X}, Instruction: {:#X}", address, instruction);

    let nnn = instruction & 0x0FFF;
    let kk = (instruction & 0x00FF) as u8;
    let n = (instruction & 0x000F) as u8;
    let x = instruction & 0x0F00 >> 8;
    let y = instruction & 0x00F0 >> 4;

    println!("nnn: {:#X}, kk: {:?}, n: {:?}, x: {}, y: {}", nnn, kk, n, x, y);

    let mut next_pc = self.pc + 2;

    match (instruction & 0xF000) >> 12 {
      0x1 => {
        // 1nnn: goto nnn
        next_pc = nnn;
      },
      0x6 => {
        // 6xkk: vx = kk
        self.write_reg_vx(x, kk);
      },
      0xA => {
        // Annn: I = nnn
        self.i = nnn;
      },

      _ => panic!("Unknown instruction {:#X}", instruction)
    }

    if self.pc == next_pc {
      panic!("Infinite loop: PC has not changed");
    }

    self.pc = next_pc;
  }

  fn write_reg_vx(&mut self, index: u16, value: u8) {
    self.vx[index as usize] = value;
  }

  fn read_reg_vx(self, index: u16) -> u8 {
    return self.vx[index as usize]
  }
}