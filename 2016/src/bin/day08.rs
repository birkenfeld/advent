use advtools::prelude::Itertools;
use advtools::input::{iter_input, to_usize};

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

struct Screen {
    pixels: [[bool; WIDTH]; HEIGHT]
}

impl Screen {
    fn new() -> Self {
        Screen { pixels: [[false; WIDTH]; HEIGHT] }
    }
    fn light_rect(&mut self, (nx, ny): (usize, usize)) {
        for y in 0..ny {
            for x in 0..nx {
                self.pixels[y][x] = true;
            }
        }
    }
    fn rotate_row(&mut self, (y, by): (usize, usize)) {
        self.pixels[y][..WIDTH-by].reverse();
        self.pixels[y][WIDTH-by..].reverse();
        self.pixels[y].reverse();
    }
    fn rotate_col(&mut self, (x, by): (usize, usize)) {
        let initial = (0..HEIGHT).map(|i| self.pixels[i][x]).collect_vec();
        for i in 0..HEIGHT {
            self.pixels[i][x] = initial[(i+HEIGHT-by) % HEIGHT];
        }
    }
    fn lit(&self) -> usize {
        self.pixels.iter().map(|col| col.iter().filter(|&&x| x).count()).sum()
    }
    fn print(&self) -> String {
        let mut result = String::new();
        for row in &self.pixels {
            result.push('\n');
            for pixel in &row[..] {
                result.push(if *pixel { '#' } else { ' ' });
            }
        }
        result
    }
}

fn main() {
    let mut screen = Screen::new();
    for line in iter_input::<String>() {
        if line.starts_with("rect") {
            let xy = line[5..].split('x').map(to_usize).collect_tuple().unwrap();
            screen.light_rect(xy);
        } else if line.starts_with("rotate row") {
            let yby = line[13..].split(" by ").map(to_usize).collect_tuple().unwrap();
            screen.rotate_row(yby);
        } else if line.starts_with("rotate column") {
            let xby = line[16..].split(" by ").map(to_usize).collect_tuple().unwrap();
            screen.rotate_col(xby);
        }
    }
    advtools::verify("Lit pixels", screen.lit(), 123);
    advtools::print("Code:", screen.print());
}
