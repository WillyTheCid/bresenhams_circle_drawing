use crate::utils::config;

pub struct Circle {
    radius: usize,
    pos: Vec<usize>
}

impl Circle {
    pub fn new(radius: usize, pos: Vec<usize>) -> Circle {
        Circle {
            radius,
            pos
        }
    }

    pub fn draw(&self, buffer: &mut Vec<u32>) {
        let xc: isize = self.pos[0] as isize;
        let yc: isize = self.pos[1] as isize;

        let window_width: isize = config::WINDOW_WIDTH as isize;

        let mut x: isize = 0;
        let mut y: isize = self.radius as isize;
        let mut d: isize = 3 - (2 * self.radius as isize);

        while x <= y {
            buffer[((xc + x) + (window_width * (yc + y))) as usize] = config::CIRCLE_COLOR;
            buffer[((xc - x) + (window_width * (yc + y))) as usize] = config::CIRCLE_COLOR;
            buffer[((xc + x) + (window_width * (yc - y))) as usize] = config::CIRCLE_COLOR;
            buffer[((xc - x) + (window_width * (yc - y))) as usize] = config::CIRCLE_COLOR;

            buffer[((xc + y) + (window_width * (yc + x))) as usize] = config::CIRCLE_COLOR;
            buffer[((xc - y) + (window_width * (yc + x))) as usize] = config::CIRCLE_COLOR;
            buffer[((xc + y) + (window_width * (yc - x))) as usize] = config::CIRCLE_COLOR;
            buffer[((xc - y) + (window_width * (yc - x))) as usize] = config::CIRCLE_COLOR;
            
            if d < 4 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }

            x += 1;
        }
    }
}