use advtools::input;
use advtools::grid::{Grid, Pos};
use advtools::prelude::{HashMap, Itertools};

/// BFS search for a given condition, stepping up or down from the starting point.
fn run(grid: &Grid<u8>, start: Pos, down: bool, cond: impl Fn(Pos) -> bool) -> usize {
    let mut visited = HashMap::with_capacity(10000);
    let mut queue = HashMap::new();
    queue.insert(start, 1);

    loop {
        for (pos, n) in std::mem::take(&mut queue) {
            for nbpos in grid.neighbors(pos) {
                if (!down && grid[nbpos] <= grid[pos] + 1 ||
                    down  && grid[pos] <= grid[nbpos] + 1)
                    && visited.get(&nbpos).map_or(true, |&m| m > n)
                {
                    if cond(nbpos) {
                        return n;
                    }
                    visited.insert(nbpos, n + 1);
                    queue.insert(nbpos, n + 1);
                }
            }
        }
    }
}

fn main() {
    let mut start = Pos(0, 0);
    let mut end = Pos(0, 0);
    let grid = Grid::new(input::lines().enumerate().map(|(y, line)| {
        line.as_bytes().iter().enumerate().map(|(x, mut pos)| {
            if pos == &b'S' {
                start = Pos(x as _, y as _);
                pos = &b'a';
            } else if pos == &b'E' {
                end = Pos(x as _, y as _);
                pos = &b'z';
            }
            pos - b'a'
        }).collect_vec()
    }));

    let part1 = run(&grid, start, false, |p| p == end);
    advtools::verify("Minimum steps from start", part1, 380);

    let part2 = run(&grid, end, true, |p| grid[p] == 0);
    advtools::verify("Trail length", part2, 375);
}
