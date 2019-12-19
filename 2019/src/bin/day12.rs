use advtools::prelude::HashSet;
use advtools::input::iter_input_regex;
use num::Integer;

#[derive(Default)]
struct Direction {
    cycle: u64,
    pos: [i32; 4],
    vel: [i32; 4],
    seen: HashSet<([i32; 4], [i32; 4])>,
}

fn main() {
    // Since the three spatial directions x, y, z are independent, record cycle
    // lengths for each of them separately.  Then, the length of a full cycle is
    // the least common multiple of all three.
    let mut dirs = [Direction::default(), Direction::default(), Direction::default()];

    // Read in positions for each of the four moons.
    for (i, (px, py, pz)) in iter_input_regex(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").enumerate() {
        dirs[0].pos[i] = px; dirs[1].pos[i] = py; dirs[2].pos[i] = pz;
    }

    for step in 0.. {
        for dir in &mut dirs {
            if dir.cycle == 0 {
                // If we saw these coordinates before, a cycle is detected.
                if !dir.seen.insert((dir.pos, dir.vel)) {
                    dir.cycle = step;
                }
            }

            // Go over pairs of moons and assign new velocities.
            for i in 0..dir.pos.len()-1 {
                for j in i+1..dir.pos.len() {
                    if dir.pos[i] < dir.pos[j] {
                        dir.vel[i] += 1;
                        dir.vel[j] -= 1;
                    } else if dir.pos[i] > dir.pos[j] {
                        dir.vel[i] -= 1;
                        dir.vel[j] += 1;
                    }
                }
            }

            // Go over individual moons and assign new positions.
            for (p, &v) in dir.pos.iter_mut().zip(&dir.vel) {
                *p += v;
            }
        }

        // Determine the total energy for part 1.
        if step == 999 {
            let energy: i32 = (0..dirs[0].pos.len()).map(|i| {
                dirs.iter().map(|d| d.pos[i].abs()).sum::<i32>() *
                    dirs.iter().map(|d| d.vel[i].abs()).sum::<i32>()
            }).sum();
            advtools::print("Total energy after 1000 steps", energy);
        }

        // If all three cycles are now known, we are done.
        if dirs.iter().all(|d| d.cycle > 0) {
            let big_cycle = dirs.iter().fold(1, |a, d| a.lcm(&d.cycle));
            advtools::print("Repeating after", big_cycle);
            return;
        }
    }
}
