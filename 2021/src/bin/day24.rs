use advtools::prelude::Itertools;
use advtools::input;

fn main() {
    let prog = input::parse_vec::<Vec<&str>>();

    // The program is built from 14 blocks of 18 instructions, each of which
    // has the same basic structure.  They differ in three places:
    //
    // - Instruction 4 has either "div z 1" or "div z 26".
    //   When viewed as a stack of values stored as `a_i * 26**i`,
    //   the blocks with "div z 1" push a value onto the stack.
    //   The blocks with "div z 26" pull a value from the stack.
    //   There are seven pushes and seven pulls.
    //
    // - In push blocks, instruction 15 has a number added to the the current
    //   serial number digit before it is pushed to the stack with "add z y".
    //
    // - In pull blocks, instruction 5 has a number added to the value from the
    //   stack before it is compared with the current serial number digit.
    //
    // To keep the stack in order, and have zero in register z in the end,
    // after each pull, the value added in the last instruction "add z y"
    // must be zero.  This means that the comparison "eql x w" must be true,
    // i.e. the digit from the "push" block + the two numbers added to it
    // in instructions 5 and 15 must equal the digit from the corresponding
    // "pull" block.
    let mut pushes = vec![];
    let pairs = (0..14).filter_map(|ix| {
        // Is the block a push block?
        if prog[18*ix + 4][2] == "1" {
            // Record the step, and the first added number.
            let num1 = prog[18*ix + 15][2].parse::<i32>().unwrap();
            pushes.push((ix, num1));
            None
        } else {
            // It is a pull block, record the step and the second added number.
            let num2 = prog[18*ix + 5][2].parse::<i32>().unwrap();
            let (push_ix, num1) = pushes.pop().unwrap();
            Some((ix, push_ix, num1 + num2))
        }
    }).collect_vec();

    // Now go through the push-pull pairs and calculate the extremally possible
    // digits with a difference equal to the sum of the two added numbers.
    let mut max_digits = vec![0; 14];
    let mut min_digits = vec![0; 14];
    for (pull_ix, push_ix, difference) in pairs {
        max_digits[pull_ix] = (9 + difference).min(9);
        max_digits[push_ix] = (9 - difference).min(9);
        min_digits[pull_ix] = (1 + difference).max(1);
        min_digits[push_ix] = (1 - difference).max(1);
    }

    advtools::verify("Maximum serial", max_digits.iter().format(""), "51983999947999");
    advtools::verify("Minimum serial", min_digits.iter().format(""), "11211791111365");
}
