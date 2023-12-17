use std::cmp::Reverse;
use std::collections::BinaryHeap;
use advtools::prelude::HashMap;
use advtools::input;
use advtools::grid::{Grid, Pos, Dir};

// Dijkstra search through the grid for the minimum-cost way to reach the exit
// (bottom right) position.
fn visit(grid: &Grid<u8>, min_count: u8, max_count: u8) -> u16 {
    let goal = Pos(grid.width() as u8 - 1, grid.height() as u8 - 1);
    // All the positions we have discovered, mapped to the minimal cost of
    // reaching them.
    let mut visited = HashMap::with_capacity(100000);
    // All the current positions we need to check, ordered by the heat loss so
    // far to reach them.
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0u16), Pos(0u8, 0u8), Dir::R, 0u8));

    loop {
        let (Reverse(prev_loss), prev_pos, prev_dir, prev_dir_count) = queue.pop().unwrap();
        if prev_pos == goal && prev_dir_count >= min_count {
            return prev_loss;
        }
        // Check each neighbor for new positions, or positions with a
        // lower-heat loss path to them.
        for (dir, pos) in Dir::all().map(|d| (d, grid.neighbor(prev_pos, d))) {
            if let Some(pos) = pos {
                // Don't flip direction.
                if prev_dir == dir.flip() && prev_dir_count > 0 {
                    continue;
                }
                // Check the minimum/maximum straight line conditions.
                if prev_dir != dir && prev_dir_count < min_count {
                    continue;
                }
                if prev_dir == dir && prev_dir_count >= max_count {
                    continue;
                }
                // Good to go, check if we found a new cheaper way to arrive at pos,
                // with the given direction and number of straight line segments.
                let loss = prev_loss + grid[pos] as u16;
                let dir_count = if dir == prev_dir { prev_dir_count + 1 } else { 1 };
                if visited.get(&(pos, dir, dir_count)).map_or(true, |&old_loss| old_loss > loss) {
                    visited.insert((pos, dir, dir_count), loss);
                    queue.push((Reverse(loss), pos, dir, dir_count));
                }
            }
        }
    }
}

fn main() {
    // Parse the grid.
    let grid = Grid::new(input::lines().map(|line| {
        line.chars().map(|ch| ch as u8 - b'0')
    }));

    advtools::verify("Minimum heat loss", visit(&grid, 0, 3), 791);
    advtools::verify("Minimum heat loss with ultra", visit(&grid, 4, 10), 900);
}
