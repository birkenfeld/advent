use advtools::prelude::Itertools;
use advtools::input;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;
const FORMAT: &str = r"(rect|rotate row|rotate column)\D*(\d+)(?:x| by )(\d+)";

struct Screen {
    pixels: [[bool; WIDTH]; HEIGHT]
}

impl Screen {
    fn new() -> Self {
        Screen { pixels: [[false; WIDTH]; HEIGHT] }
    }

    fn light_rect(&mut self, nx: usize, ny: usize) {
        for y in 0..ny {
            for x in 0..nx {
                self.pixels[y][x] = true;
            }
        }
    }

    fn rotate_row(&mut self, y: usize, by: usize) {
        self.pixels[y][..WIDTH-by].reverse();
        self.pixels[y][WIDTH-by..].reverse();
        self.pixels[y].reverse();
    }

    fn rotate_col(&mut self, x: usize, by: usize) {
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
                result.push(if *pixel { '█' } else { ' ' });
            }
        }
        result
    }
}

fn main() {
    let mut screen = Screen::new();
    for (what, a, b) in input::rx_lines(FORMAT) {
        match what {
            "rect" => screen.light_rect(a, b),
            "rotate row" => screen.rotate_row(a, b),
            "rotate column" => screen.rotate_col(a, b),
            _ => unreachable!()
        }
    }
    advtools::verify("Lit pixels", screen.lit(), 123);
    advtools::print("Code:", screen.print());
}
