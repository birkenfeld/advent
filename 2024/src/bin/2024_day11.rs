use advtools::input;

fn main() {
    // input::set("125 17");
    let mut stones = input::parse::<Vec<u64>>();

    for _ in 0..25 {
        for stone in std::mem::take(&mut stones) {
            if stone == 0 {
                stones.push(1);
                continue;
            }
            let num_digits = stone.ilog10() + 1;
            if num_digits % 2 == 0 {
                let divisor = 10_u64.pow(num_digits / 2);
                stones.push(stone / divisor);
                stones.push(stone % divisor);
            } else {
                stones.push(stone * 2024);
            }
        }
    }
    advtools::verify("After 25 times", stones.len(), 229043);
    // advtools::verify("With concatenation", tot2, 37598910447546_u64);
}
