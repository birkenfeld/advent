use advtools::input::iter_lines;
use advtools::grid::{Grid, Pos};

fn mark(grid: &mut Grid<(u8, usize)>, pos: Pos<u32>, basin: usize) -> usize {
    grid.neighbors(pos).map(|nbpos| {
        let (nheight, nbasin) = grid[nbpos];
        if nheight != 9 && nbasin == 0 {
            grid[nbpos].1 = basin;
            1 + mark(grid, nbpos, basin)
        } else {
            0
        }
    }).sum()
}

fn main() {
    let mut grid = Grid::new(iter_lines().map(|line| {
        line.chars().map(|ch| (ch as u8 - b'0', 0)).collect()
    }));

    let mut risk_level = 0;
    let mut basins = vec![0];

    for pos in grid.positions::<u32>() {
        let (height, basin) = grid[pos];

        if grid.neighbors(pos).all(|nbpos| grid[nbpos].0 > height) {
            risk_level += 1 + height as u32;
        }

        if height != 9 && basin == 0 {
            let basin = basins.len();
            grid[pos].1 = basin;
            basins.push(1 + mark(&mut grid, pos, basin));
        }
    }

    basins.sort();
    let biggest_basins = basins.iter().rev().take(3).product::<usize>();

    advtools::verify("Risk level", risk_level, 580);
    advtools::verify("Biggest basins", biggest_basins, 856716);
}
