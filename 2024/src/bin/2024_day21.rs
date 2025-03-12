use advtools::input;
use advtools::prelude::Itertools;
use advtools::grid::{Grid, Dir, Pos};

use Dir::*;

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

// 0-9, then A
const NUMERIC: [(usize, usize); 11] = [
    (3, 1), (2, 0), (2, 1), (2, 2),
    (1, 0), (1, 1), (1, 2), (0, 0),
    (0, 1), (0, 2), (3, 2),
];

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

// Dir as usize: U, D, L, R, A
const DIRECT: [(usize, usize); 5] = [
    (0, 1), (1, 1), (1, 0), (1, 2), (0, 2),
];

fn find_path(from: (usize, usize), to: (usize, usize), cur: &mut Vec<Vec<Dir>>, poison: (usize, usize)) {
    let (fx, fy) = from;
    let (tx, ty) = to;
    if fx < tx {
        for way in cur {
            way.push(R);
            
        }
    }
    
}

fn main() {
    input::set("029A
980A
179A
456A
379A
");

    // find all possible paths between buttons on the numeric keypad
    let mut paths_numeric = Vec::new();
    for (i, &(sy, sx)) in NUMERIC.iter().enumerate() {
        for (j, &(dy, dx)) in NUMERIC.iter().enumerate() {
            let (mut cx, mut cy) = (sx, sy);
            
        }
    }
}
