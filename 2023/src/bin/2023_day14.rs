use advtools::input;
use advtools::grid::{Grid, Pos};
use advtools::prelude::{HashMap, Itertools};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Stone,
    Cube,
}

type Spans = Vec<(usize, usize)>;

// Find all spans of empty or round-stone tiles in a row or column.
fn find_spans(input: impl Iterator<Item=Tile>) -> Spans {
    input.enumerate().group_by(|&(_, t)| t != Tile::Cube)
                     .into_iter()
                     .filter(|&(ok, _)| ok)
                     .map(|(_, mut group)| (group.next().unwrap().0, group.count() + 1))
                     .collect()
}

// Roll all stones in a certain way, using the spans to determine which stones
// are rolled.
fn roll(grid: &mut Grid<Tile>, all_spans: &[Spans], pos: impl Fn(usize, usize) -> Pos<usize>, fwd: bool) {
    for (i, spans) in all_spans.iter().enumerate() {
        for &(start, count) in spans {
            let stones = (0..count).filter(|j| grid[pos(i, start+j)] == Tile::Stone).count();
            for j in 0..count {
                grid[pos(i, start+j)] = match fwd {
                    true if j >= count - stones => Tile::Stone,
                    false if j < stones => Tile::Stone,
                    _ => Tile::Empty,
                };
            }
        }
    }
}

// Calculate the load as given.
fn load(grid: &Grid<Tile>) -> usize {
    grid.positions::<usize>().flat_map(
        |pos| (grid[pos] == Tile::Stone).then_some(grid.height() - pos.y)
    ).sum()
}

fn main() {
    // Parse the grid.
    let mut grid = Grid::new(input::lines().map(|l| {
        l.chars().map(|c| match c {
            '.' => Tile::Empty,
            'O' => Tile::Stone,
            '#' => Tile::Cube,
            _ => unreachable!(),
        })
    }));

    // Find all spans of empty or round-stone tiles in each row and column.
    let by_row = (0..grid.height()).map(|row| {
        find_spans((0..grid.width()).map(|col| grid[Pos(col, row)]))
    }).collect_vec();
    let by_col = (0..grid.width()).map(|col| {
        find_spans((0..grid.height()).map(|row| grid[Pos(col, row)]))
    }).collect_vec();

    // Part 1: roll once and determine the load.
    roll(&mut grid, &by_col, |i, j| Pos(i, j), false);
    advtools::verify("Load on beams", load(&grid), 108826);

    // Part 2: roll until we find a cycle.
    let mut seen = HashMap::new();
    for iteration in 0.. {
        if iteration != 0 {  // The initial "northward" roll is already done.
            roll(&mut grid, &by_col, |i, j| Pos(i, j), false);
        }
        roll(&mut grid, &by_row, |i, j| Pos(j, i), false);
        roll(&mut grid, &by_col, |i, j| Pos(i, j), true);
        roll(&mut grid, &by_row, |i, j| Pos(j, i), true);

        if let Some(start) = seen.insert(grid.clone(), iteration) {
            // Found a cycle: calculate the remaining iterations, and retrieve
            // the resulting grid from the memorizing map.
            let cycle = iteration - start;
            let remaining = (1_000_000_000 - iteration) % cycle;
            grid = seen.into_iter().find(|&(_, v)| v + 1 == start + remaining).unwrap().0;
            break;
        }
    }
    advtools::verify("Load on beams after billion cycles", load(&grid), 99291);
}
