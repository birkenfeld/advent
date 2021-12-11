use advtools::input::iter_lines;
use advtools::grid::{Grid, Pos};

// Recursively set the flash indicator.
fn maybe_flash(grid: &mut Grid<(u8, bool)>, pos: Pos<u32>) {
    let (level, flashed) = &mut grid[pos];
    if *level > 9 && !*flashed {
        *flashed = true;
        for npos in grid.neighbors_diag(pos) {
            grid[npos].0 += 1;
            maybe_flash(grid, npos);
        }
    }
}

fn main() {
    // Parse the initial grid.
    let mut grid = Grid::new(iter_lines().map(|line| {
        line.chars().map(|ch| ((ch as u8 - b'0') as u8, false)).collect()
    }));

    let mut total_flashes = 0;

    for step in 1.. {
        // First round, update energy level and maybe flash.
        for pos in grid.positions::<u32>() {
            grid[pos].0 += 1;
            maybe_flash(&mut grid, pos);
        }

        // Reset flash indicators and count the total flashes.
        let mut step_flashes = 0;
        for pos in grid.positions::<u32>() {
            if grid[pos].1 {
                step_flashes += 1;
                grid[pos] = (0, false);
            }
        }
        total_flashes += step_flashes;

        // Keep track of part 1.
        if step == 100 {
            advtools::verify("Total flashes", total_flashes, 1757);
        }

        // Check for synchronization.
        if step_flashes == grid.len() {
            advtools::verify("Synchronized step", step, 422);
            break;
        }
    }
}
