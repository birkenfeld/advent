extern crate advtools;
use advtools::prelude::*;

fn jump<F: Fn(i32) -> i32>(jumps: &mut [i32], f: F) -> i32 {
    let n = jumps.len() as i32;
    let mut pos = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        let new_pos = pos as i32 + jumps[pos];
        jumps[pos] = f(jumps[pos]);
        if new_pos < 0 || new_pos >= n {
            return steps;
        }
        pos = new_pos as usize;
    }
}

fn main() {
    let mut jumps = iter_input::<i32>().collect_vec();
    let steps1 = jump(&mut jumps.clone(), |ofs| ofs + 1);
    let steps2 = jump(&mut jumps, |ofs| if ofs >= 3 { ofs - 1 } else { ofs + 1 });
    println!("Steps to outside: {}", steps1);
    println!("Steps to outside (modified rule): {}", steps2);
}
