use advtools::prelude::{HashSet, Regex};
use advtools::input::{iter_input, to_i32};
use advtools::grid::Pos;

fn is_allowed(pos: Pos, blockers: &HashSet<Pos>, size: &Pos) -> bool {
    pos.x >= 0 && pos.x <= size.x && pos.y >= 0 && pos.y <= size.y &&
        !blockers.contains(&pos)
}

fn find_steps(initial: Pos, final_: Pos, blockers: &HashSet<Pos>, size: &Pos) -> usize {
    let mut seen = HashSet::with_capacity(1000);
    let mut positions = vec![initial];
    let mut generation = 0;

    loop {
        generation += 1;
        let mut new_positions = vec![];
        for pos in positions {
            for new_pos in pos.neighbors() {
                if is_allowed(new_pos, blockers, size) && seen.insert(new_pos) {
                    if new_pos == final_ {
                        return generation;
                    }
                    new_positions.push(new_pos);
                }
            }
        }
        positions = new_positions;
    }
}

fn main() {
    let rx = Regex::new(r"node-x(\d+)-y(\d+) +(\d+)T +(\d+)T").unwrap();
    let mut nodes: Vec<(Pos, (i32, i32))> = Vec::new();
    let mut smallest_cap = 1000;
    for line in iter_input::<String>() {
        if let Some(cap) = rx.captures(&line) {
            // (x, y), (size, used)
            nodes.push((
                Pos(to_i32(&cap[1]), to_i32(&cap[2])),
                (to_i32(&cap[3]), to_i32(&cap[4]))
            ));
            smallest_cap = smallest_cap.min((nodes.last().unwrap().1).0);
        }
    }
    let size = nodes[nodes.len() - 1].0;
    let mut blockers = HashSet::new();
    let mut pairs = 0;
    let mut hole_pos = Pos(0, 0);
    for n1 in &nodes {
        // determine if it's a blocker
        if (n1.1).1 > smallest_cap {
            blockers.insert(n1.0);
        }

        // determine initial hole
        if (n1.1).1 == 0 {
            hole_pos = n1.0;
        }

        // determine viable pairs
        for n2 in &nodes {
            if n1.0 == n2.0 || (n1.1).1 == 0 {
                continue;
            }
            if (n1.1).1 <= ((n2.1).0 - (n2.1).1) {
                pairs += 1;
            }
        }
    }
    advtools::verify("Number of viable pairs", pairs, 1043);

    let mut total_steps = 0;
    // move data from (max,0) to (0,0) step by step
    for target_x in (0 .. size.x).rev() {
        let target_pos = Pos(target_x, 0);
        let data_pos = target_pos.right();
        // find shortest way for the hole to move to (tx,0),
        // temporarily blocking the data position (it may not move!)
        blockers.insert(data_pos);
        let steps = find_steps(hole_pos, target_pos, &blockers, &size);
        blockers.take(&data_pos);

        // add one additional step (moving the data into the hole)
        total_steps += steps + 1;
        hole_pos = data_pos;
    }
    advtools::verify("Total # of steps", total_steps, 185);
}
