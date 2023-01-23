use advtools::input;
use advtools::prelude::{HashSet, Itertools};
use advtools::vecs::i32::*;

const SHAPES: [(&[Vec2], i32); 5] = [
    (&[vec2(0, 0), vec2(1, 0), vec2(2, 0), vec2(3, 0)], 1),
    (&[vec2(1, 2), vec2(0, 1), vec2(1, 1), vec2(2, 1), vec2(1, 0)], 3),
    (&[vec2(2, 2), vec2(2, 1), vec2(0, 0), vec2(1, 0), vec2(2, 0)], 3),
    (&[vec2(0, 3), vec2(0, 2), vec2(0, 1), vec2(0, 0)], 4),
    (&[vec2(0, 1), vec2(1, 1), vec2(0, 0), vec2(1, 0)], 2),
];

fn main() {
    let jet_pattern = input::chars().map(
        |ch| vec2(2*(ch == '>') as i32 - 1, 0)
    ).collect_vec();
    let mut jets = jet_pattern.into_iter().cycle();

    let mut height = 0;
    let mut in_place = HashSet::new();
    let mut hdiffs = vec![];

    for ((frags, rock_height), nrocks) in SHAPES.iter().cycle().zip(1..) {
        // New rock, starts at the given coordinates.
        let mut pos = vec2(2, height + 3);
        loop {
            // Apply jet movement.
            let jet = jets.next().unwrap();
            if !frags.iter().map(|d| pos + jet + d)
                            .any(|p| in_place.contains(&p) || p.x < 0 || p.x > 6) {
                pos += jet;
            }
            // Fall downward.
            if frags.iter().map(|d| pos + d - Y2)
                           .any(|p| in_place.contains(&p) || p.y < 0) {
                break;
            }
            pos -= Y2;
        }
        // Fix the current rock in place.
        frags.iter().for_each(|d| { in_place.insert(pos + d); });

        // Record the height difference.
        let new_height = height.max(pos.y + rock_height);
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
