const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
  screen: [[bool; WIDTH]; HEIGHT],
}

impl Display {
  pub fn new() -> Display {
    Display {
      screen: [[false; WIDTH]; HEIGHT],
    }
  }

  pub fn clear(&mut self) {
    for pixel_y in 0..HEIGHT {
      for pixel_x in 0..WIDTH {
        self.screen[pixel_y][pixel_x] = false;
      }
    }
  }

  pub fn paint(&mut self) {
    for y in 0..HEIGHT {
      for x in 0..WIDTH {
        if self.screen[y][x] {
          print!(" ");
        } else {
          print!("#");
        }
      }
      print!("\n");
    }

    println!("\n")
  }

  pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
    let mut flipped = false;
    let mut b = byte;

    for i in 0..8 {
      let pixel_x = (x + i) as usize;
      let pixel_y = y as usize;

      match (b & 0b1000_0000) >> 7 {
        0 => {
          if self.screen[pixel_y][pixel_x] == true {
            flipped = true;
          }

          self.screen[pixel_y][pixel_x] = false;
          print!(" ");
        },
        1 => {
          self.screen[pixel_y][pixel_x] = true;
          print!("#");
        },
        _ => unreachable!(),
      }

      b = b << 1;
    }

    print!("\n");

    return flipped
  }
}