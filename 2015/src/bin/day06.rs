extern crate advtools;
use advtools::input::iter_input_regex;

enum Todo { On, Off, Toggle }

const FORMAT: &str = r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)";
type Pos = (usize, usize);

fn main() {
    let mut bool_grid = [[false; 1000]; 1000];
    let mut dim_grid = [[0u16; 1000]; 1000];
    for (verb, from, to) in iter_input_regex::<(String, Pos, Pos)>(FORMAT) {
        let todo = match &*verb {
            "turn on" => Todo::On,
            "turn off" => Todo::Off,
            _ => Todo::Toggle
        };
        for ix in from.0..=to.0 {
            for iy in from.1..=to.1 {
                match todo {
                    Todo::On => {
                        bool_grid[ix][iy] = true;
                        dim_grid[ix][iy] += 1;
                    }
                    Todo::Off => {
                        bool_grid[ix][iy] = false;
                        dim_grid[ix][iy] = dim_grid[ix][iy].saturating_sub(1);
                    }
                    Todo::Toggle => {
                        bool_grid[ix][iy] = !bool_grid[ix][iy];
                        dim_grid[ix][iy] += 2;
                    }
                }
            }
        }
    }
    let number_on: usize = bool_grid.iter().map(
        |row| row.iter().filter(|&&lamp| lamp).count()).sum();
    println!("Lights on: {}", number_on);
    let total_brightness: usize = dim_grid.iter().map(
        |row| row.iter().map(|&v| v as usize).sum::<usize>()).sum();
    println!("Total brightness: {}", total_brightness);
}
