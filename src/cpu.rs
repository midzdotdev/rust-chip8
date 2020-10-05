use crate::chip8::PROGRAM_START;
use crate::bus::Bus;

// CPU has 16 8-bit registers, V0 to VF
// Also 16-bit register called I
// VF is a flag used by some instructions and shouldn't be modified directly

pub struct Cpu {
  v: [u8; 16],
  i: u16,
  pc: u16,
  ret_stack: Vec<u16>,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      v: [0; 16],
      i: 0,
      pc: PROGRAM_START,
      ret_stack: Vec::<u16>::new(),
    }
  }

  pub fn run_instruction(&mut self, bus: &mut Bus) {
    let address = self.pc;
    
    let hi = bus.ram_read_byte(address) as u16;
    let lo = bus.ram_read_byte(address+1) as u16;

    let opcode: u16 = (hi << 8) | lo;

    println!("Address: {:#X}, Opcode: {:#X}", address, opcode);

    let nnn = opcode & 0x0FFF;
    let kk = (opcode & 0x00FF) as u8;
    let n = (opcode & 0x000F) as u8;
    let x = ((opcode & 0x0F00) >> 8) as u8;
    let y = ((opcode & 0x00F0) >> 4) as u8;

    println!("nnn: {:#X}, kk: {:?}, n: {:?}, x: {}, y: {}", nnn, kk, n, x, y);

    self.debug_state();

    let mut next_pc = self.pc + 2;

    let opcode_panic = || panic!("Unknown opcode {:#X}", opcode);

    match (opcode & 0xF000) >> 12 {
      
      0x0 => {
        match opcode {
          // 00E0: Clear display
          0x00E0 => bus.display_clear(),

          // 00EE: Return from subroutine
          0x00EE => next_pc = self.ret_stack.pop().unwrap(),

          // 0nnn: Jump to routine at nnn
          _ => panic!("Instruction 0nnn should not be used.")
        }
      },

      // 1nnn: Goto nnn
      0x1 => next_pc = nnn,

      // 2nnn: Call subroutine at nnn
      0x2 => {
        self.ret_stack.push(self.pc);
        next_pc = nnn;
      },

      // 3xkk: Skip next instruction if Vx != kk
      0x3 => {
        if self.read_reg(x) != kk {
          next_pc = self.pc + 4;
        }
      },

      // 5xy0: Skip next instruction if Vx == Vy
      0x5 => {
        let vx = self.read_reg(x);
        let vy = self.read_reg(y);

        if vx == vy {
          next_pc = self.pc + 4;
        }
      },

      // 6xkk: Vx = kk
      0x6 => self.write_reg(x, kk),

      // 7xkk: Vx = Vx + kk
      0x7 => {
        let vx = self.read_reg(x);
        let result = vx.wrapping_add(kk);
        self.write_reg(x, result);
      },

      0x8 => {
        let vx = self.read_reg(x);
        let vy = self.read_reg(y);

        match n {
          // 8xy0: Vx = Vy
          0 => self.write_reg(x, vy),

          // 8xy1: Vx = Vx OR Vy
          1 => self.write_reg(x, vx | vy),

          // 8xy2: Vx = Vx AND Vy
          2 => self.write_reg(x, vx & vy),

          // 8xy3: Vx = Vx XOR Vy
          3 => self.write_reg(x, vx ^ vy),

          // 8xy4: Vx = Vx + Vy, VF = carry
          4 => {
            let result: u16 = vx as u16 + vy as u16;
            self.write_reg(x, result as u8);

            if result > 0xFF {
              self.write_reg(0xF, 1);
            }
          },

          _ => panic!(),
        }
      },

      // Annn: I = nnn
      0xA => self.i = nnn,

      // Dxyn: Display sprite n-byte sprite at address I at (Vx, Vy), set VF = collision
      0xD => self.debug_draw_sprite(bus, x, y, n),

      0xE => {
        let key_code = self.read_reg(x);
        let is_key_pressed = bus.key_pressed(key_code);

        match kk {
          // Ex9E: Skip next instruction if key Vx is pressed
          0x9E => {
            if is_key_pressed {
              next_pc = self.pc + 4;
            }
          },

          // ExA1: Skip next instruction if key with value Vx is not pressed
          0xA1 => {
            if !is_key_pressed {
              next_pc = self.pc + 4;
            }
          },

          _ => opcode_panic(),
        }
      },

      0xF => {
        let vx = self.read_reg(x);
        
        match kk {
          // Fx07: Vx = delay timer
          0x07 => self.write_reg(x, bus.get_delay_timer()),

          // Fx15: delay timer = Vx
          0x15 => bus.set_delay_timer(vx),

          // Fx1E: I = I + Vx
          0x1E => self.i = self.i.wrapping_add(vx as u16),

          // Fx65: Fill V0..Vx with values from memory starting at I
          0x65 => {
            for index in 0..x+1 {
              let value = bus.ram_read_byte(self.i + index as u16);
              self.write_reg(index, value);
            }
          }

          _ => opcode_panic(),
        }
      },

      _ => opcode_panic(),
    }

    if self.pc == next_pc {
      panic!("Infinite loop: PC has not changed");
    }

    self.pc = next_pc;
  }

  fn debug_state(&mut self) {
    for i in 0..16 {
      print!("V{:X?}: {:X?} ", i, self.read_reg(i));
    }

    print!("\n");
    print!("I: {} ", self.i);
    print!("PC: {}", self.pc);
    println!("\n");
  }

  fn debug_draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8) {
    let mut vf_value = 0;

    for line in 0..height {
      let address = self.i + (line as u16);
      let byte = bus.ram_read_byte(address);
      let pos_y = y + line;
      
      let flipped = bus.display_draw_byte(byte, x, pos_y);

      if flipped {
        vf_value = 1;
      }
    }

    self.write_reg(0xF, vf_value);
  }

  fn write_reg(&mut self, index: u8, value: u8) {
    self.v[index as usize] = value;
  }

  fn read_reg(&mut self, index: u8) -> u8 {
    return self.v[index as usize]
  }
}