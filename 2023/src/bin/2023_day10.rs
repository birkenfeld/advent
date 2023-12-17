use advtools::input;
use advtools::grid::{Grid, Dir, Dir::*, Pos};
use advtools::prelude::Itertools;

fn main() {
    // Parse the input.
    let mut grid = Grid::new(input::lines().map(|line| line.chars().map(|ch| match ch {
        '|' => [U, D],
        '-' => [L, R],
        'J' => [U, L],
        'L' => [U, R],
        '7' => [D, L],
        'F' => [D, R],
        'S' => [D, D], // later
        _   => [U, U], // dummy
    })));
    // Keep a new grid for the second part.
    let mut pipe = Grid::fill(None, grid.width(), grid.height());

    // Find start position and reconstruct its pipe.
    let start = grid.find_pos(|&tile| tile == [D, D]).unwrap();
    let goes = |dir: Dir| grid[start + dir].contains(&dir.flip());
    let dirs = Dir::all().tuple_combinations().find(|&(a, b)| goes(a) && goes(b)).unwrap();
    grid[start] = [dirs.0, dirs.1];

    // Part 1: Walk through the pipe (and mark it in grid2).
    let mut steps = 0;
    let mut pos = start;
    let mut cur_dir = grid[start][0];
    'outer: loop {
        for dir in grid[pos] {
            if dir != cur_dir.flip() {
                // Found the continuation.
                cur_dir = dir;
                pos = pos.to(dir);
                steps += 1;

                // Mark pieces of our pipe loop.
                pipe[pos] = Some(grid[pos]);

                if pos == start {
                    break 'outer;
                }
                break;
            }
        }
    }
    advtools::verify("Steps to furthest point", steps / 2, 7102);

    // Part 2: Raycast through grid2 for enclosed tiles.
    let mut enclosed = 0;
    for y in 0..grid.height() {
        let mut inside = false;
        for x in 0..grid.width() {
            match pipe[Pos(x, y)] {
                // Need to ignore tiles that don't change the "in/out" state.
                Some([L, R] | [D, R|L]) => (),
                Some(_) => inside = !inside,
                _ => enclosed += inside as usize
            }
        }
    }
    advtools::verify("Enclosed tiles", enclosed, 363);
}
