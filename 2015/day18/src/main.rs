extern crate advtools;

use std::mem;
use advtools::IterExt;

const N: usize = 100;
const STEPS: usize = 100;

fn step(grid: &mut [[bool; N+2]; N+2], stuck: bool) {
    let ref mut outgrid = [[false; N+2]; N+2];
    if stuck {
        for &(i, j) in &[(1, 1), (1, N), (N, 1), (N, N)] {
            grid[i][j] = true;
        }
    }
    for _ in 0..STEPS {
        for i in 1..N+1 {
            for j in 1..N+1 {
                let neighb =
                    grid[i-1][j-1] as u8 +
                    grid[ i ][j-1] as u8 +
                    grid[i+1][j-1] as u8 +
                    grid[i-1][ j ] as u8 +
                    grid[i+1][ j ] as u8 +
                    grid[i-1][j+1] as u8 +
                    grid[ i ][j+1] as u8 +
                    grid[i+1][j+1] as u8;
                if grid[i][j] {
                    outgrid[i][j] = neighb == 2 || neighb == 3;
                } else {
                    outgrid[i][j] = neighb == 3;
                }
            }
        }
        if stuck {
            for &(i, j) in &[(1, 1), (1, N), (N, 1), (N, N)] {
                outgrid[i][j] = true;
            }
        }
        mem::swap(grid, outgrid);
    }
}

fn main() {
    for &stuck in &[false, true] {
        let mut grid = [[false; N+2]; N+2];
        let input_lines = advtools::iter_input::<String>();
        for (line, input) in grid.iter_mut().skip(1).zip(input_lines) {
            for (loc, ch) in line.iter_mut().skip(1).zip(input.chars()) {
                *loc = ch == '#';
            }
        }
        step(&mut grid, stuck);
        let number_on: usize = grid.iter().map(
            |row| row.iter().filter(|&&lamp| lamp).count()).sum_from(0);
        println!("On after 100 steps{}: {}",
                 if stuck { " with stuck corners" } else { "" }, number_on);
    }
}
