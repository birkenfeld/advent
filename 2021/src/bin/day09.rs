use advtools::input::iter_lines;
use advtools::grid::{Grid, Pos};

// Flood-fill and mark all reachable neighbors in a basin, and return the number
// of squares in the basin.
fn get_size(grid: &mut Grid<(u32, bool)>, pos: Pos<u32>) -> u32 {
    grid[pos].1 = true;
    grid.neighbors(pos).map(|nbpos| {
        let (height, marked) = grid[nbpos];
        if height != 9 && !marked {
            get_size(grid, nbpos)
        } else {
            0
        }
    }).sum::<u32>() + 1
}

fn main() {
    // Parse the initial grid.
    let mut grid = Grid::new(iter_lines().map(|line| {
        line.chars().map(|ch| ((ch as u8 - b'0') as u32, false)).collect()
    }));

    let mut risk_level = 0;
    let mut basin_sizes = vec![];

    // Go through all squares, find minima and from each minimum start marking
    // the basin's squares, recording the basin sizes.
    for pos in grid.positions::<u32>() {
        if grid.neighbors(pos).all(|nbpos| grid[nbpos].0 > grid[pos].0) {
            risk_level += 1 + grid[pos].0;
            basin_sizes.push(get_size(&mut grid, pos));
        }
    }

    // Determine the biggest basins.
    basin_sizes.sort();
    let biggest_basins: u32 = basin_sizes.iter().rev().take(3).product();

    advtools::verify("Risk level", risk_level, 580);
    advtools::verify("Biggest basins", biggest_basins, 856716);
}
