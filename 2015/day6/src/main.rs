#![feature(iter_arith)]

use std::fs::File;
use std::io::{BufReader, BufRead};

enum Todo { On, Off, Toggle }

fn main() {
    let mut bool_grid = [[false; 1000]; 1000];
    let mut dim_grid = [[0u16; 1000]; 1000];
    for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let parts = line.split(" ").collect::<Vec<_>>();
        let todo = if line.starts_with("turn on") { Todo::On } else
            if line.starts_with("turn off") { Todo::Off } else { Todo::Toggle };
        let coords = if let Todo::Toggle = todo {
            (parts[1], parts[3])
        } else {
            (parts[2], parts[4])
        };
        let from = coords.0.split(",").map(|v| v.parse().unwrap()).collect::<Vec<usize>>();
        let to = coords.1.split(",").map(|v| v.parse().unwrap()).collect::<Vec<usize>>();
        for ix in from[0]..to[0]+1 {
            for iy in from[1]..to[1]+1 {
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
