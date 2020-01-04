use advtools::input::{input_string, to_u32};

fn find(goal: u32, maxhouses: u32, multiplier: u32, n: u32) -> usize {
    let mut presents = vec![1u32; n as usize];
    for elf in 2..n {
        for house in 1..maxhouses.min(n/elf) {
            presents[(house * elf) as usize] += multiplier * elf;
        }
    }
    presents.into_iter().position(|p| p >= goal).unwrap()
}

fn main() {
    let goal = to_u32(input_string().trim());
    let n = goal / 10;
    advtools::verify("Visiting all houses", find(goal, n, 10, n), 776160);
    advtools::verify("Visiting only 50 houses", find(goal, 50, 11, n), 786240);
}
