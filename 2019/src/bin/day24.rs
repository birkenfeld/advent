use std::mem::replace;
use advtools::prelude::HashSet;
use advtools::input::input_string;
use bit::BitIndex;

const MINUTES: usize = 200;

fn main() {
    let bugs: u32 = input_string().chars().filter(|&ch| ch == '#' || ch == '.')
        .enumerate()
        .fold(0, |mut bugs, (i, ch)| *bugs.set_bit(i, ch == '#'));

    let mut seen = HashSet::new();
    seen.insert(bugs);

    let mut new_bugs = bugs;
    loop {
        let old_bugs = replace(&mut new_bugs, 0);

        for bit in 0..25 {
            let neighbors = match bit / 5 {
                0 => old_bugs.bit(bit + 5) as u8,
                4 => old_bugs.bit(bit - 5) as u8,
                _ => old_bugs.bit(bit - 5) as u8 + old_bugs.bit(bit + 5) as u8
            } + match bit % 5 {
                0 => old_bugs.bit(bit + 1) as u8,
                4 => old_bugs.bit(bit - 1) as u8,
                _ => old_bugs.bit(bit - 1) as u8 + old_bugs.bit(bit + 1) as u8
            };
            if neighbors == 1 || (!old_bugs.bit(bit) && neighbors == 2) {
                new_bugs.set_bit(bit, true);
            }
        }

        if !seen.insert(new_bugs) {
            advtools::verify("First layout seen twice", new_bugs, 28903899);
            break;
        }
    }

    let mut depths = [0; 2*MINUTES + 3];
    depths[MINUTES + 1] = bugs;

    for _ in 1..=MINUTES {
        let old_depths = replace(&mut depths, [0; 2*MINUTES + 3]);
        for depth in 1..=2*MINUTES + 1 {
            let this = old_depths[depth];
            let down = old_depths[depth - 1];
            let up = old_depths[depth + 1];
            let mut new_bugs = 0;
            for bit in 0..25 {
                let neighbors = match bit / 5 { // top/bottom neighbors
                    0 => this.bit(bit + 5) as u8 + up.bit(7) as u8,
                    4 => this.bit(bit - 5) as u8 + up.bit(17) as u8,
                    _ => if bit == 7 {
                        this.bit(2) as u8 +
                            (0..5).filter(|&i| down.bit(i)).count() as u8
                    } else if bit == 17 {
                        this.bit(22) as u8 +
                            (20..25).filter(|&i| down.bit(i)).count() as u8
                    } else if bit == 12 {
                        0
                    } else {
                        this.bit(bit - 5) as u8 + this.bit(bit + 5) as u8
                    }
                } + match bit % 5 { // left/right neighbors
                    0 => this.bit(bit + 1) as u8 + up.bit(11) as u8,
                    4 => this.bit(bit - 1) as u8 + up.bit(13) as u8,
                    _ => if bit == 11 {
                        this.bit(10) as u8 +
                            (0..25).step_by(5).filter(|&i| down.bit(i)).count() as u8
                    } else if bit == 13 {
                        this.bit(14) as u8 +
                            (4..25).step_by(5).filter(|&i| down.bit(i)).count() as u8
                    } else if bit == 12 {
                        0
                    } else {
                        this.bit(bit - 1) as u8 + this.bit(bit + 1) as u8
                    }
                };
                if neighbors == 1 || (!this.bit(bit) && neighbors == 2) {
                    new_bugs.set_bit(bit, true);
                }
            }
            depths[depth] = new_bugs;
        }
    }

    let total = depths.iter().map(|v| v.count_ones()).sum::<u32>();
    advtools::verify("Bugs after 200 minutes", total, 1896);
}
