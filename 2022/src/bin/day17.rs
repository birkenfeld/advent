use advtools::input;
use advtools::prelude::{HashSet, Itertools};

const SHAPES: [(&[(i32, i32)], i32); 5] = [
    (&[(0, 0), (1, 0), (2, 0), (3, 0)], 1),
    (&[(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)], 3),
    (&[(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)], 3),
    (&[(0, 3), (0, 2), (0, 1), (0, 0)], 4),
    (&[(0, 1), (1, 1), (0, 0), (1, 0)], 2),
];

fn main() {
    let jet_pattern = input::chars().map(|ch| 2*(ch == '>') as i32 - 1).collect_vec();
    let mut jets = jet_pattern.into_iter().cycle();

    let mut height = 0;
    let mut in_place = HashSet::new();
    let mut hdiffs = vec![];

    for ((frags, rock_height), nrocks) in SHAPES.iter().cycle().zip(1..) {
        // New rock, starts at the given coordinates.
        let mut rx = 2;
        let mut ry = height + 3;
        loop {
            // Apply jet movement.
            let jet = jets.next().unwrap();
            if !frags.iter().map(|(dx, dy)| (rx + jet + dx, ry + dy))
                            .any(|p| in_place.contains(&p) || p.0 < 0 || p.0 > 6) {
                rx += jet;
            }
            // Fall downward.
            if frags.iter().map(|(dx, dy)| (rx + dx, ry - 1 + dy))
                           .any(|p| in_place.contains(&p) || p.1 < 0) {
                break;
            }
            ry -= 1;
        }
        // Fix the current rock in place.
        frags.iter().for_each(|(dx, dy)| { in_place.insert((dx + rx, dy + ry)); });

        // Record the height difference.
        let new_height = height.max(ry + rock_height);
        hdiffs.push(new_height - height);
        height = new_height;

        // Part 1: get the height for 2022 rocks.
        if nrocks == 2022 {
            advtools::verify("Height after 2022", height, 3111);
        }

        // Part 2: find a cycle in the height differences and calculate the final
        // height for 1 billion.
        for cycle_len in 20..hdiffs.len()/2 {
            let (first, second) = hdiffs.rchunks(cycle_len).next_tuple().unwrap();
            if first == second {
                // Found a cycle, calculate the adjustments to get the final size.
                let ncycles = (1_000_000_000_000 - nrocks) / cycle_len;
                let nfinal = (1_000_000_000_000 - nrocks) % cycle_len;

                let percycle = first.iter().sum::<i32>() as usize;
                let additional = first[..nfinal].iter().sum::<i32>() as usize;

                let final_height = height as usize + percycle * ncycles + additional;
                advtools::verify("Height after 1 billion", final_height, 1526744186042_usize);
                return;
            }
        }
    }
}
