use advtools::input;
use advtools::prelude::Itertools;

const KEY: i64 = 811589153;

fn run(numbers: &[i64], key: i64, iters: usize) -> i64 {
    // Next position cycles by n-1 since the current number doesn't take part.
    let cycle = numbers.len() as i64 - 1;
    // Save the initial index together with the value, so that we can later
    // find the order of processing.
    let mut current = numbers.iter().enumerate().map(|(i, x)| (x * key, i)).collect_vec();

    for _ in 0..iters {
        for init_index in 0..current.len() {
            // Find current position of the element.
            let cur_pos = current.iter().position(|x| x.1 == init_index).unwrap();
            let shift = current[cur_pos].0;
            // Calculate target position.
            let new_pos = ((cur_pos as i64) + shift).rem_euclid(cycle) as usize;
            // Do the movement by rotating the element into place.
            if new_pos < cur_pos {
                current[new_pos..=cur_pos].rotate_right(1);
            } else {
                current[cur_pos..=new_pos].rotate_left(1);
            }
        }
    }

    // Add up the 1000/2000/3000th element after zero.
    current.iter().cycle().map(|x| x.0)
                          .skip_while(|&x| x != 0)
                          .step_by(1000).skip(1).take(3)
                          .sum::<i64>()
}

fn main() {
    let input = input::parse_lines::<i64>().map(|x| x).collect_vec();

    advtools::verify("Grove coordinates", run(&input, 1, 1), 7228);
    advtools::verify("With decryption key", run(&input, KEY, 10), 4526232706281_i64);
}
