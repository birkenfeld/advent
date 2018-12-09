use advtools::prelude::Itertools;
use advtools::input::iter_input;

/// Jump through the list of offsets, with the closure determining how an offset
/// is modified after it is taken.
fn jump<F: Fn(i32) -> i32>(jumps: &mut [i32], f: F) -> i32 {
    let n = jumps.len() as i32;
    let mut pos = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        let new_pos = pos as i32 + jumps[pos];
        jumps[pos] = f(jumps[pos]);
        // Exit condition: jumped outside the list.
        if new_pos < 0 || new_pos >= n {
            return steps;
        }
        pos = new_pos as usize;
    }
}

fn main() {
    let mut jumps = iter_input::<i32>().collect_vec();

    // Part 1: Taken offsets are increased by one.
    let steps1 = jump(&mut jumps.clone(), |ofs| ofs + 1);
    advtools::print("Steps to outside", steps1);

    // Part 2: Taken offsets are increased or decreased by one.
    let steps2 = jump(&mut jumps, |ofs| if ofs >= 3 { ofs - 1 } else { ofs + 1 });
    advtools::print("Steps to outside (modified rule)", steps2);
}
