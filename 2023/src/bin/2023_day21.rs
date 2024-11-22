use advtools::input;
use advtools::grid::{Grid, Pos};
use advtools::prelude::{Itertools, HashSet};

fn parity(pos: Pos<i32>) -> bool {
    (pos.x + pos.y).rem_euclid(2) == 0
}

fn visit(maze: &Grid<bool>, start: Pos<i32>, steps: i64) -> i64 {
    let (w, h) = (maze.width() as i32, maze.height() as i32);
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut queue = vec![start];
    for _ in 1..steps+1 {
        for pos in std::mem::take(&mut queue) {
            for nbpos in pos.neighbors() {
                // Taking the modulo here allows the infinite maze of part 2.
                if !maze[Pos(nbpos.x.rem_euclid(w), nbpos.y.rem_euclid(h))] {
                    continue;
                }
                if !seen.insert(nbpos) {
                    continue;
                }
                queue.push(nbpos);
            }
        }
    }
    seen.into_iter()
        .filter(|&pos| parity(pos) == parity(start) ^ (steps % 2 != 0))
        .count() as _
}

fn main() {
    let mut start = Pos(0, 0);
    let grid = Grid::new(input::lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, ch)| match ch {
            '.' => true,
            '#' => false,
            'S' => {
                start = Pos(x as i32, y as i32);
                true
            }
            _ => unreachable!()
        }).collect_vec()
    }));

    // Part 1: just visit once.
    const STEPS_1: i64 = 64;
    advtools::verify("Reachable positions", visit(&grid, start, STEPS_1), 3762);

    // Part 2: assume the number of reachable positions grows regularly in
    // quadratic fashion, so we can sample at three points.
    const STEPS_2: i64 = 26501365;
    let w = grid.width() as i64;
    let x = (STEPS_2 - w/2) / w;

    let mut y = Vec::new();
    for i in 0..3 {
        let steps = w/2 + i*w;
        y.push(visit(&grid, start, steps));
    }

    // Calculate the coefficients of the quadratic function.
    let a = (y[2] - 2*y[1] + y[0]) / 2;
    let b = y[1] - y[0] - a;
    let c = y[0];

    let result = a*x*x + b*x + c;

    advtools::verify("Reachable for many steps", result, 621944727930768_usize);
}
