use advtools::prelude::iproduct;
use advtools::input;

enum Todo { On, Off, Toggle }

const FORMAT: &str = r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)";

fn main() {
    let mut bool_grid = [[false; 1000]; 1000];
    let mut dim_grid = [[0u16; 1000]; 1000];

    for (verb, (x1, y1), (x2, y2)) in input::rx_lines(FORMAT) {
        let todo = match verb {
            "turn on" => Todo::On,
            "turn off" => Todo::Off,
            _ => Todo::Toggle
        };
        for (ix, iy) in iproduct!(x1..=x2, y1..=y2) {
            let (ix, iy): (usize, usize) = (ix, iy);
            match todo {
                Todo::On => {
                    bool_grid[iy][ix] = true;
                    dim_grid[iy][ix] += 1;
                }
                Todo::Off => {
                    bool_grid[iy][ix] = false;
                    dim_grid[iy][ix] = dim_grid[iy][ix].saturating_sub(1);
                }
                Todo::Toggle => {
                    bool_grid[iy][ix] ^= true;
                    dim_grid[iy][ix] += 2;
                }
            }
        }
    }

    let number_on: usize = bool_grid.iter().map(
        |row| row.iter().filter(|&&lamp| lamp).count()).sum();
    advtools::verify("Lights on", number_on, 543903);

    let total_brightness: usize = dim_grid.iter().map(
        |row| row.iter().map(|&v| v as usize).sum::<usize>()).sum();
    advtools::verify("Total brightness", total_brightness, 14687245);
}
