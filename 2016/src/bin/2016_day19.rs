use advtools::input;

fn calc_1(n: u64) -> u64 {
    let pot = (0..).find(|&m| 2u64.pow(m) > n).map_or(0, |m| 2u64.pow(m - 1));
    2*(n - pot) + 1
}

fn calc_2(n: u64) -> u64 {
    let pot = (0..).find(|&m| 3u64.pow(m) >= n).map_or(0, |m| 3u64.pow(m - 1));
    if n <= 2*pot {
        n - pot
    } else {
        2*n - 3*pot
    }
}

fn main() {
    let elves = input::parse();
    advtools::verify("First mode", calc_1(elves), 1834903);
    advtools::verify("Second mode", calc_2(elves), 1420280);
}
