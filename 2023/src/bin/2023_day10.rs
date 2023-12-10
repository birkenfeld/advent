use advtools::input;
use advtools::grid::{Grid, Dir, Dir::*, Pos};
use advtools::prelude::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {  // for part 2
    Pipe,
    Unmarked,
    Outside
}

fn flood_fill(grid: &mut Grid<Tile>, pos: Pos<u32>) {
    grid[pos] = Tile::Outside;
    grid.neighbors(pos).for_each(
        |nbpos| if grid[nbpos] == Tile::Unmarked {
            flood_fill(grid, nbpos)
        }
    )
}

fn main() {
    // Parse the input.
    let mut grid = Grid::new(input::lines().map(|line| line.chars().map(|ch| match ch {
        '|' => [U, D],
        '-' => [L, R],
        'J' => [L, U],
        'L' => [R, U],
        '7' => [L, D],
        'F' => [R, D],
        'S' => [D, D], // later
        _   => [U, U], // dummy
    })));
    // Keep a new grid for the second part, with double the size, so that
    // we can easily flood-fill "squeezing" between the pipes.
    let mut grid2 = Grid::fill(Tile::Unmarked, grid.width()*2 + 1, grid.height()*2 + 1);

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

                // Mark the pipe in grid2.
                let grid2pos = Pos(pos.x*2 + 1, pos.y*2 + 1);
                grid2[grid2pos] = Tile::Pipe;
                grid2[grid2pos + grid[pos][0]] = Tile::Pipe;
                grid2[grid2pos + grid[pos][1]] = Tile::Pipe;

                if pos == start {
                    break 'outer;
                }
                break;
            }
        }
    }
    advtools::verify("Steps to furthest point", steps / 2, 7102);

    // Part 2: Find enclosed tiles.
    flood_fill(&mut grid2, Pos(0, 0));
    let enclosed = grid2.positions().filter(|pos: &Pos<u32>| {
        grid2[*pos] == Tile::Unmarked && pos.x % 2 == 1 && pos.y % 2 == 1
    }).count();
    advtools::verify("Enclosed tiles", enclosed, 363);
}
