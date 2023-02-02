use advtools::input;
use advtools::prelude::HashSet;
use advtools::grid::{Dir, Pos, Grid};

const WALL: u8 = 255;
const LEFT: u8 = 1;
const RIGHT: u8 = 2;
const UP: u8 = 4;
const DOWN: u8 = 8;

/// Advance the grid by one step.
fn step(grid: &Grid<u8>) -> Grid<u8> {
    let xmax = grid.width() - 2;
    let ymax = grid.height() - 2;
    let mut new_grid = Grid::fill(0, grid.width(), grid.height());
    for pos in grid.positions() {
        let cur = grid[pos];
        // Walls stay walls.
        if cur == WALL {
            new_grid[pos] = WALL;
            continue;
        }
        // Advance blizzards.
        for (bit, dir) in [(LEFT, Dir::L), (RIGHT, Dir::R), (UP, Dir::U), (DOWN, Dir::D)] {
            if cur & bit != 0 {
                let Pos { mut x, mut y } = pos + dir;
                if x == 0 { x = xmax; }
                else if x == xmax + 1 { x = 1; }
                if y == 0 { y = ymax; }
                else if y == ymax + 1 { y = 1; }
                new_grid[Pos(x, y)] |= bit;
            }
        }
    }
    new_grid
}

fn bfs(grids: &[Grid<u8>], start_step: usize, start_pos: Pos, target_pos: Pos) -> usize {
    let ngrids = grids.len();
    let w = grids[0].width() as i32;
    let h = grids[0].height() as i32;

    let mut queue = HashSet::new();
    // Add initial state.
    queue.insert(start_pos);

    // BFS for target position, starting at step 2.
    for step in start_step.. {
        for pos in std::mem::take(&mut queue) {
            if pos == target_pos {
                return step;
            }
            for new_pos in Dir::all().filter_map(|d| pos.maybe_to(d, w, h)).chain(Some(pos)) {
                if grids[(step + 1) % ngrids][new_pos] == 0 {
                    queue.insert(new_pos);
                }
            }
        }
    }
    unreachable!()
}

fn main() {
    let grid = Grid::new(
        input::lines().map(|line| {
            line.chars().map(|ch| match ch {
                '#' => WALL,
                '<' => LEFT,
                '>' => RIGHT,
                '^' => UP,
                'v' => DOWN,
                _ => 0
            })
        })
    );
    let w = grid.width() as i32;
    let h = grid.height() as i32;
    let entrance = Pos(1, 0);
    let exit = Pos(w - 2, h - 1);

    let mut grids = vec![grid];
    loop {
        grids.push(step(grids.last().unwrap()));
        if grids.first() == grids.last() {
            break;
        }
    }

    let first_there = bfs(&grids, 0, entrance, exit);
    advtools::verify("First to exit", first_there - 1, 373);

    let first_back = bfs(&grids, first_there, exit, entrance);
    let second_there = bfs(&grids, first_back, entrance, exit);
    advtools::verify("Second to exit", second_there - 3, 997);
}
