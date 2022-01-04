use advtools::input;
use advtools::grid::{Grid, Pos};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    None,
    Down,
    Right,
}

fn main() {
    let mut grid = Grid::new(input::lines().map(|l| l.chars().map(|ch| match ch {
        '.' => Tile::None,
        'v' => Tile::Down,
        '>' => Tile::Right,
        _ => unreachable!()
    }).collect()));
    let (w, h) = (grid.width(), grid.height());

    for i in 1.. {
        let mut new_grid = grid.clone();

        for pos in grid.positions::<usize>() {
            if let Tile::Right = grid[pos] {
                let dpos = Pos((pos.x + 1) % w, pos.y);
                if grid[dpos] == Tile::None {
                    new_grid[pos] = Tile::None;
                    new_grid[dpos] = Tile::Right;
                }
            }
        }

        let right_moved = grid != new_grid;

        grid = new_grid;
        let mut new_grid = grid.clone();

        for pos in grid.positions::<usize>() {
            if let Tile::Down = grid[pos] {
                let dpos = Pos(pos.x, (pos.y + 1) % h);
                if grid[dpos] == Tile::None {
                    new_grid[pos] = Tile::None;
                    new_grid[dpos] = Tile::Down;
                }
            }
        }

        if grid == new_grid && !right_moved {
            advtools::verify("Last step", i, 0);
            return;
        }

        grid = new_grid;
    }
}
