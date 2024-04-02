use advtools::input;
use advtools::prelude::{Itertools, HashMap, HashSet};

#[derive(Clone)]
enum Brick {
    X(i32, i32, i32, i32),
    Y(i32, i32, i32, i32),
    Z(i32, i32, i32, i32),
}

fn range_overlap(a: i32, b: i32, c: i32, d: i32) -> bool {
    // Check if the ranges [a, a+b] and [c, c+d] overlap.
    a <= c+d && c <= a+b
}

impl Brick {
    fn overlaps_xy(&self, other: &Self) -> bool {
        match (self, other) {
            (&Brick::X(x1, y1, _, xe1), &Brick::X(x2, y2, _, xe2)) => {
                range_overlap(x1, xe1, x2, xe2) && y1 == y2
            }
            (&Brick::X(x1, y1, _, xe1), &Brick::Y(x2, y2, _, ye2)) => {
                (x1..=x1 + xe1).contains(&x2) && (y2..=y2 + ye2).contains(&y1)
            }
            (&Brick::X(x1, y1, _, xe1), &Brick::Z(x2, y2, _, _)) => {
                (x1..=x1 + xe1).contains(&x2) && y1 == y2
            }
            (&Brick::Y(x1, y1, _, ye1), &Brick::Y(x2, y2, _, ye2)) => {
                x1 == x2 && range_overlap(y1, ye1, y2, ye2)
            }
            (&Brick::Y(x1, y1, _, ye1), &Brick::Z(x2, y2, _, _)) => {
                (y1..=y1 + ye1).contains(&y2) && x1 == x2
            }
            (&Brick::Z(x1, y1, _, _), &Brick::Z(x2, y2, _, _)) => {
                x1 == x2 && y1 == y2
            }
            _ => other.overlaps_xy(self),
        }
    }

    fn bottom_z(&self) -> i32 {
        match *self {
            Brick::X(_, _, z, _) => z,
            Brick::Y(_, _, z, _) => z,
            Brick::Z(_, _, z, _) => z,
        }
    }

    fn top_z(&self) -> i32 {
        match *self {
            Brick::X(_, _, z, _) => z,
            Brick::Y(_, _, z, _) => z,
            Brick::Z(_, _, z, ze) => z + ze,
        }
    }

    fn set_z(&mut self, new_z: i32) {
        match self {
            Brick::X(_, _, z, _) => *z = new_z,
            Brick::Y(_, _, z, _) => *z = new_z,
            Brick::Z(_, _, z, _) => *z = new_z,
        }
    }
}

fn main() {
    let mut bricks = Vec::new();
    let mut supports = HashMap::<_, Vec<_>>::new();
    for line in input::rx_lines(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)") {
        let (x1, y1, z1, x2, y2, z2): (i32, i32, i32, i32, i32, i32) = line;
        let brick = if x1 != x2 {
            Brick::X(x1, y1, z1, x2 - x1)
        } else if y1 != y2 {
            Brick::Y(x1, y1, z1, y2 - y1)
        } else {
            Brick::Z(x1, y1, z1, z2 - z1)
        };
        let brick_id = bricks.len();
        bricks.push(brick);
        supports.entry(bricks[brick_id].top_z()).or_default().push(brick_id);
    }

    loop {
        let mut moved = false;

        'bricks:
        for brick_id in 0..bricks.len() {
            // Check for any brick supporting this one.
            let support_z = bricks[brick_id].bottom_z() - 1;

            if support_z == 0 {
                continue;
            }

            if let Some(others) = supports.get(&support_z) {
                for &other_id in others {
                    if bricks[brick_id].overlaps_xy(&bricks[other_id]) {
                        continue 'bricks;
                    }
                }
            }
            // No support found, move brick down.
            moved = true;
            supports.get_mut(&bricks[brick_id].top_z()).unwrap().retain(|b| *b != brick_id);
            bricks[brick_id].set_z(support_z);
            supports.entry(bricks[brick_id].top_z()).or_default().push(brick_id);
        }

        if !moved {
            break;
        }
    }

    // Find supporting bricks.
    let mut support_map = HashMap::<usize, Vec<usize>>::new();
    for brick_id in 0..bricks.len() {
        // Check for any brick supporting this one.
        let support_z = bricks[brick_id].bottom_z() - 1;

        // It's on the ground?
        if support_z == 0 {
            continue;
        }

        // support_map: brick_id -> those that support it.
        if let Some(others) = supports.get(&support_z) {
            for &other_id in others {
                if bricks[brick_id].overlaps_xy(&bricks[other_id]) {
                    support_map.entry(brick_id).or_default().push(other_id);
                }
            }
        }
    }

    let mut candidates = (0..bricks.len()).collect_vec();
    for supported in support_map.values() {
        // If a brick is supported by only one, that one can't be a candidate.
        if supported.len() == 1 {
            candidates.retain(|&c| c != supported[0]);
        }
    }
    advtools::verify("Bricks safe to disintegrate", candidates.len(), 515);

    let mut total = 0;
    for brick_id in 0..bricks.len() {
        // For each brick, find all bricks that will fall if it is disintegrated.
        let mut moving = HashSet::new();
        // This one isn't technically moving, but it needs to be in the set
        // to get things started.
        moving.insert(brick_id);
        loop {
            let mut changed = false;

            for (brick, sup_by) in support_map.iter() {
                if sup_by.iter().all(|s| moving.contains(s)) {
                    changed |= moving.insert(*brick);
                }
            }

            if !changed {
                total += moving.len() - 1;  // don't count the disintegrated one
                break;
            }
        }
    }
    advtools::verify("Sum of falling bricks", total, 101541);
}
