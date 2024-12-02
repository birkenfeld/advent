use advtools::input;
use advtools::prelude::Itertools;
use advtools::grid::{Grid, Dir, Pos};

enum Move {
    Go(i32),
    Turn(bool),
}

#[derive(Clone, Copy)]
enum Transform {
    InvertX,
    InvertY,
    SwapXY,
}

/// Specifies where we land and how to adjust coordinates
/// when going outside of one side.
#[derive(Clone, Copy)]
struct Connection {
    index: usize,  // index of next side in list
    new_dir: Dir,  // new direction to face (in terms of flat map)
    transform: Transform,  // how to transform X and Y
}

/// Represents a single side of the cube / square in the flat map.
#[derive(Clone)]
struct Side {
    grid: Grid<bool>,  // true = accessible
    flat_pos: (i32, i32),  // original position in input
    connect: [Option<Connection>; 4],  // indexed by Dir as usize
}

impl Side {
    fn connect(&mut self, dir: Dir, index: usize, new_dir: Dir, transform: Transform) {
        self.connect[dir as usize].get_or_insert(Connection { index, new_dir, transform });
    }

    fn jump(&self, mut pos: Pos, dir: Dir, size: i32) -> (usize, Dir, Pos) {
        let connect = self.connect[dir as usize].unwrap();
        // Process the coordinate transformation for the side jump.
        match connect.transform {
            Transform::InvertX => pos.x = size - 1 - pos.x,
            Transform::InvertY => pos.y = size - 1 - pos.y,
            Transform::SwapXY  => (pos.x, pos.y) = (pos.y, pos.x),
        }
        (connect.index, connect.new_dir, pos)
    }
}

fn run(size: i32, moves: &[Move], sides: &[Side]) -> i32 {
    // Starting location: on the first side, find the first non-wall.
    let mut side = 0;
    let mut pos = sides[side].grid.positions().find(|&p| sides[side].grid[p]).unwrap();
    let mut dir = Dir::R;

    for mov in moves {
        match *mov {
            Move::Turn(true) => dir = dir.left(),
            Move::Turn(false) => dir = dir.right(),
            Move::Go(n) => for _ in 0..n {
                // Find the next location, first try to step within the current side.
                let step = pos.maybe_to(dir, size, size);
                let (new_side, new_dir, new_pos) = if let Some(step_pos) = step {
                    // Can stay on this side.
                    (side, dir, step_pos)
                } else {
                    // Need to jump to the adjacent side.
                    sides[side].jump(pos, dir, size)
                };
                // If it is not a wall, go there.
                if sides[new_side].grid[new_pos] {
                    (side, pos, dir) = (new_side, new_pos, new_dir);
                } else {
                    // Else stop moving.
                    break;
                }
            }
        }
    }

    let dir_part = match dir {
        Dir::R => 0,
        Dir::D => 1,
        Dir::L => 2,
        Dir::U => 3,
    };

    let offsets = sides[side].flat_pos;
    dir_part + (offsets.0*size + pos.x + 1) * 4 + (offsets.1*size + pos.y + 1) * 1000
}

fn main() {
    let (cells, instr) = input::string().split("\n\n").collect_tuple().unwrap();

    // Parse the moves.
    let moves = instr.chars().chunk_by(|ch| ch.is_numeric()).into_iter()
        .map(|(num, mut ch)| if num {
            Move::Go(ch.fold(0, |n, ch| n*10 + ch as i32 - b'0' as i32))
        } else {
            Move::Turn(ch.next().unwrap() == 'L')
        }).collect_vec();

    // Calculate the size of each cube side.
    let total = cells.chars().filter(|&c| c == '.' || c == '#').count();
    let size = (total as f64 / 6.).sqrt() as usize;

    // Parse all the cube sides in the input.
    let mut sides = vec![];
    for (y, chunk) in cells.lines().chunks(size).into_iter().enumerate() {
        // We're iterating by chunks of lines that represent one side in height.
        let chunk = chunk.map(|l| l.as_bytes()).collect_vec();
        // Go through each potential side in this chunk.
        let n_sides = chunk[0].len() / size;
        for x in 0..n_sides {
            // Check if there is a data here at all.
            if chunk[0][x*size] == b' ' {
                continue;
            }
            // We have a side! Parse its wall/open cells.
            let grid = Grid::new(
                chunk.iter().map(|line| {
                    // Take only the relevant part out of each line.
                    line.iter().map(|&b| b == b'.').skip(x*size).take(size)
                })
            );
            sides.push(Side {
                grid,
                flat_pos: (x as i32, y as i32),
                connect: [None; 4],
            });
        }
    }

    // Helper to connect to a second side whose flat_pos matches the given target.
    let flat_pos = sides.iter().map(|side| side.flat_pos).collect_vec();
    let maybe_connect = |side: &mut Side, target, dir, transform| {
        for (other, &pos) in flat_pos.iter().enumerate() {
            if pos == target {
                side.connect(dir, other, dir, transform);
                break;
            }
        }
    };

    // Assign initial connections.
    for side in &mut sides {
        let (x, y) = side.flat_pos;
        maybe_connect(side, (x-1, y), Dir::L, Transform::InvertX);
        maybe_connect(side, (x+1, y), Dir::R, Transform::InvertX);
        maybe_connect(side, (x, y-1), Dir::U, Transform::InvertY);
        maybe_connect(side, (x, y+1), Dir::D, Transform::InvertY);
    }

    // Part 1: add connections based on location in the flattened map.
    let mut sides_part1 = sides.clone();

    for side in &mut sides_part1 {
        let (x, y) = side.flat_pos;
        // Go through all edges of the side and find the closest matching side.
        for offset in 2..6 {
            maybe_connect(side, ((x-offset).rem_euclid(5), y), Dir::L, Transform::InvertX);
            maybe_connect(side, ((x+offset).rem_euclid(5), y), Dir::R, Transform::InvertX);
            maybe_connect(side, (x, (y-offset).rem_euclid(5)), Dir::U, Transform::InvertY);
            maybe_connect(side, (x, (y+offset).rem_euclid(5)), Dir::D, Transform::InvertY);
        }
    }

    advtools::verify("Flat password", run(size as i32, &moves, &sides_part1), 97356);

    // Part 2: add connections corresponding to the folded cube.
    // Hardcoded for now...

    sides[0].connect(Dir::U, 5, Dir::R, Transform::SwapXY);
    sides[5].connect(Dir::L, 0, Dir::D, Transform::SwapXY);

    sides[0].connect(Dir::L, 3, Dir::R, Transform::InvertY);
    sides[3].connect(Dir::L, 0, Dir::R, Transform::InvertY);

    sides[1].connect(Dir::U, 5, Dir::U, Transform::InvertY);
    sides[5].connect(Dir::D, 1, Dir::D, Transform::InvertY);

    sides[1].connect(Dir::R, 4, Dir::L, Transform::InvertY);
    sides[4].connect(Dir::R, 1, Dir::L, Transform::InvertY);

    sides[1].connect(Dir::D, 2, Dir::L, Transform::SwapXY);
    sides[2].connect(Dir::R, 1, Dir::U, Transform::SwapXY);

    sides[2].connect(Dir::L, 3, Dir::D, Transform::SwapXY);
    sides[3].connect(Dir::U, 2, Dir::R, Transform::SwapXY);

    sides[4].connect(Dir::D, 5, Dir::L, Transform::SwapXY);
    sides[5].connect(Dir::R, 4, Dir::U, Transform::SwapXY);

    advtools::verify("Folded password", run(size as i32, &moves, &sides), 120175);
}
