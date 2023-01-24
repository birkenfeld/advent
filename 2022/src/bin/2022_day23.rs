use advtools::input;
use advtools::prelude::HashMap;
use advtools::grid::{Dir, Grid};

fn main() {
    let mut grid = Grid::new(input::lines().map(|line| line.chars().map(|c| c == '#')));
    grid.enlarge(60, false);

    let mut dirs = [Dir::U, Dir::D, Dir::L, Dir::R];

    for round in 1.. {
        let mut targets = HashMap::new();

        // Step 1: select new targets.
        for pos in grid.positions::<usize>().filter(|&p| grid[p]) {
            // Not moving if no neighbors at all.
            if grid.neighbors_diag(pos).all(|p| !grid[p]) { continue; }
            // Try to find a free direction.
            for &main in &dirs {
                let pos1 = pos.to(main);
                if !(grid[pos1] || grid[pos1.to(main.left())] || grid[pos1.to(main.right())]) {
                    targets.entry(pos1).or_insert(vec![]).push(pos);
                    break;
                }
            }
        }

        let mut moved = false;

        // Step 2: execute moves.
        for (target, elves) in targets {
            if let [pos] = elves[..] {
                grid[pos] = false;
                grid[target] = true;
                moved = true;
            }
        }

        // Part 1: count the empty places in the elves' bounding rectangle.
        if round == 10 {
            let mut max_x = 0; let mut max_y = 0;
            let mut min_x = grid.width(); let mut min_y = grid.height();
            for pos in grid.positions::<usize>().filter(|&p| grid[p]) {
                max_x = max_x.max(pos.x);
                max_y = max_y.max(pos.y);
                min_x = min_x.min(pos.x);
                min_y = min_y.min(pos.y);
            }
            let free = grid.positions::<usize>()
                .filter(|&p| !grid[p] && p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y)
                .count();
            advtools::verify("Free spaces after 10", free, 4172);
        }

        // Part 2: no moves, we're done.
        if !moved {
            advtools::verify("No elf moves in round", round, 942);
            return;
        }

        dirs.rotate_left(1);
    }
}
