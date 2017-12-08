const GOAL: u32 = 33_100_000;
const N: u32 = GOAL / 10;

fn find(maxhouses: u32, multiplier: u32) -> usize {
    let mut presents = vec![1u32; N as usize];
    for elf in 2..N {
        for house in 1..maxhouses.min(N/elf) {
            presents[(house * elf) as usize] += multiplier * elf;
        }
    }
    presents.into_iter().position(|p| p >= GOAL).unwrap()
}

fn main() {
    println!("Visiting all houses: {}", find(N, 10));
    println!("Visiting only 50 houses: {}", find(50, 11));
}
