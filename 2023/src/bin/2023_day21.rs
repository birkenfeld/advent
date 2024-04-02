use advtools::input;
use advtools::grid::{Grid, Pos};
use advtools::prelude::{Itertools, HashSet};

fn parity(pos: Pos<usize>) -> bool {
    (pos.x + pos.y) % 2 == 1
}

fn visit(maze: &Grid<bool>, start: Pos<usize>, steps: usize) -> usize {
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut queue = vec![start];
    for _ in 1..steps+1 {
        for pos in std::mem::take(&mut queue) {
            for nbpos in pos.neighbors() {
                if !maze[nbpos] {
                    continue;
                }
                if !seen.insert(nbpos) {
                    continue;
                }
                queue.push(nbpos);
            }
        }
    }
    println!("{}", seen.len());
    seen.into_iter().filter(|&pos| parity(pos) == parity(start)).count()
}

fn main() {
//     input::set("\
// ...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........
// ");
    let mut start = Pos(0, 0);
    let grid = Grid::new(input::lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, ch)| match ch {
            '.' => true,
            '#' => false,
            'S' => {
                start = Pos(x, y);
                true
            }
            _ => unreachable!()
        }).collect_vec()
    }));

    advtools::verify("Reachable positions", visit(&grid, start, 64), 3762);
}
