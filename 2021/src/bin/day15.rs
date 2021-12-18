use advtools::prelude::{Itertools, HashMap};
use advtools::input::iter_lines;
use advtools::grid::{Grid, Pos};

// BFS search through the grid for the minimum-cost way to reach the exit
// (bottom right) position.
fn visit(grid: &Grid<u8>) -> u32 {
    // All the positions we have discovered, mapped to the minimal cost of
    // reaching them.
    let mut visited = HashMap::with_capacity(10000);
    // All the current positions we need to check, and the risk so far
    // to reach them.
    let mut queue = HashMap::new();
    queue.insert(Pos(0u16, 0u16), 0);

    loop {
        for (pos, risk) in std::mem::replace(&mut queue, HashMap::with_capacity(10000)) {
            // Check each neighbor for new positions, or positions with a
            // lower-risk path to them.
            for nbpos in grid.neighbors(pos) {
                let new_risk = risk + grid[nbpos] as u32;
                if visited.get(&nbpos).map_or(true, |&old_risk| old_risk > new_risk) {
                    visited.insert(nbpos, new_risk);
                    queue.insert(nbpos, new_risk);
                }
            }
        }
        if queue.is_empty() {
            // Nothing new to check -> we must have found the best way.
            let goal = Pos(grid.width() as u16 - 1, grid.height() as u16 - 1);
            return visited[&goal];
        }
    }
}

fn main() {
    // Parse the initial grid.
    let grid = Grid::new(iter_lines().map(|line| {
        line.chars().map(|ch| ch as u8 - b'0').collect()
    }));

    advtools::verify("Small grid", visit(&grid), 790);

    // Multiply the grid for part two.
    let (w, h) = (grid.width(), grid.height());
    let big_grid = Grid::from_iter(5*w, (0..5*w).cartesian_product(0..5*h).map(|(x, y)| {
        let new = grid[Pos(x % w, y % h)] + (x / w) as u8 + (y / h) as u8;
        (new - 1) % 9 + 1
    }));

    advtools::verify("Big grid", visit(&big_grid), 2998);
}
