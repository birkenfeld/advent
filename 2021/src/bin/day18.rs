use advtools::prelude::Itertools;
use advtools::input::iter_lines;

#[derive(Clone, Copy, Debug)]
enum El {
    Nest,
    Denest,
    Dig(u8),
}

fn add(n1: Vec<El>, n2: Vec<El>) -> Vec<El> {
    let mut num = vec!(El::Nest);
    num.extend(n1);
    num.extend(n2);
    num.push(El::Denest);

    'outer: loop {
        // Handle exploding.
        let mut level = 0;
        for i in 0..num.len() {
            match num[i] {
                El::Nest => if level == 4 {
                    // Replace the [Nest, Digit, Digit, Denest] with a zero.
                    if let Some((_, El::Dig(first), El::Dig(second), _)) =
                        num.splice(i..i+4, [El::Dig(0)]).collect_tuple()
                    {
                        // Find the previous digit, add the first one to it.
                        if let Some(El::Dig(n)) = num[..i].iter_mut().rev().find(|el| matches!(el, El::Dig(_))) {
                            *n += first;
                        }
                        // Find the next digit, add the second one to it.
                        if let Some(El::Dig(n)) = num[i+1..].iter_mut().find(|el| matches!(el, El::Dig(_))) {
                            *n += second;
                        }
                        // Start again applying rules.
                        continue 'outer;
                    }
                } else {
                    level += 1;
                }
                El::Denest => level -= 1,
                El::Dig(_) => ()
            }
        }

        // Handle splitting.
        for i in 0..num.len() {
            match num[i] {
                El::Dig(n) if n >= 10 => {
                    // Replace Digit by [Nest, Digit, Digit, Denest].
                    num.splice(i..i+1, [El::Nest, El::Dig(n/2), El::Dig(n  - n/2), El::Denest]);
                    // Start again applying rules.
                    continue 'outer;
                }
                _ => ()
            }
        }

        return num;
    }
}

fn magnitude(n: &[El]) -> u32 {
    let mut stack = vec![];
    for el in n {
        match el {
            El::Nest => (),
            El::Denest => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(3*left + 2*right)
            }
            El::Dig(n) => stack.push(*n as u32)
        }
    }
    stack[0]
}

fn main() {
    let nums = iter_lines().map(|line| {
        line.chars().filter_map(|ch| match ch {
            '0' ..= '9' => Some(El::Dig(ch as u8 - b'0')),
            '[' => Some(El::Nest),
            ']' => Some(El::Denest),
            _ => None
        }).collect_vec()
    }).collect_vec();

    let total_sum = nums.iter().cloned().reduce(add).unwrap();
    advtools::verify("Total of sum", magnitude(&total_sum), 4469);

    let largest_sum = nums.iter().permutations(2).map(|nums| {
        magnitude(&add(nums[0].clone(), nums[1].clone()))
    }).max().unwrap();
    advtools::verify("Maximum sum of 2", largest_sum, 4770);
}
