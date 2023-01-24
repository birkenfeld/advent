use advtools::input;
use advtools::prelude::HashSet;
use advtools::grid::{Dir, Pos, Grid};

const WALL: u8 = 255;
const LEFT: u8 = 0;
const RIGHT: u8 = 1;
const UP: u8 = 2;
const DOWN: u8 = 3;

fn step(grid: &Grid<u8>) -> Grid<u8> {
    let xm = grid.width() as i32 - 1;
    let ym = grid.height() as i32 - 1;
    let mut new_grid = Grid::fill(0, grid.width(), grid.height());
    for pos in grid.positions::<i32>() {
        let cur = grid[pos];
        if cur == WALL {
            new_grid[pos] = WALL;
            continue;
        }
        if cur & (1 << LEFT) != 0 {
            if pos.x > 1 {
                new_grid[pos.left()] |= 1 << LEFT;
            } else {
                new_grid[Pos(xm - 1, pos.y)] |= 1 << LEFT;
            }
        }
        if cur & (1 << RIGHT) != 0 {
            if pos.x < xm - 1 {
                new_grid[pos.right()] |= 1 << RIGHT;
            } else {
                new_grid[Pos(1, pos.y)] |= 1 << RIGHT;
            }
        }
        if cur & (1 << UP) != 0 {
            if pos.y > 1 {
                new_grid[pos.up()] |= 1 << UP;
            } else {
                new_grid[Pos(pos.x, ym - 1)] |= 1 << UP;
            }
        }
        if cur & (1 << DOWN) != 0 {
            if pos.y < ym - 1 {
                new_grid[pos.down()] |= 1 << DOWN;
            } else {
                new_grid[Pos(pos.x, 1)] |= 1 << DOWN;
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
            for new_pos in Dir::iter().filter_map(|d| pos.maybe_step(d, w, h)).chain(Some(pos)) {
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
                '<' => 1 << LEFT,
                '>' => 1 << RIGHT,
                '^' => 1 << UP,
                'v' => 1 << DOWN,
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
