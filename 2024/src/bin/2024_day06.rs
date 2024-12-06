use advtools::input;
use advtools::grid::{Grid, Pos, Dir};
use advtools::prelude::HashSet;

#[derive(Clone)]
enum Tile {
    Empty,
    Full,
    Visited,
}

fn walk(grid: &Grid<Tile>, mut pos: Pos<usize>) -> Option<usize> {
    let mut tiles = HashSet::new();
    let mut visited = HashSet::new();
    let mut dir = Dir::U;
    loop {
        tiles.insert(pos);
        if !visited.insert((pos, dir)) {
            return None;
        }
        match grid.get(pos + dir) {
            None => return Some(tiles.len()),
            Some(&Tile::Full) => dir = dir.right(),
            _ => pos = pos + dir,
        }
    }
}

fn main() {
    let mut grid = Grid::new(input::lines().map(|line| {
        line.chars().map(|ch| match ch {
            '.' => Tile::Empty,
            '#' => Tile::Full,
            '^' => Tile::Visited,
            _   => panic!("unexpected")
        })
    }));
    let initpos = grid.find_pos(|v| matches!(v, Tile::Visited)).unwrap();
    let visited = walk(&grid, initpos).unwrap();
    advtools::verify("Visited tiles", visited, 4580);

    // TODO: this is brute force and should be changed.
    let mut npos = 0;
    for obspos in grid.positions::<usize>() {
        if matches!(grid[obspos], Tile::Empty) {
            grid[obspos] = Tile::Full;
            if walk(&grid, initpos).is_none() {
                npos += 1;
            }
            grid[obspos] = Tile::Empty;
        }
    }
    advtools::verify("Possibilities for obstruction", npos, 1480);
}
