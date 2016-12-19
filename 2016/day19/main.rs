const ELVES: usize = 3014603;

fn calc_1(n: usize) -> usize {
    let pot = (0..).find(|&m| 2usize.pow(m) > n).map_or(0, |m| 2usize.pow(m - 1));
    2*(n - pot) + 1
}

fn calc_2(n: usize) -> usize {
    let pot = (0..).find(|&m| 3usize.pow(m) >= n).map_or(0, |m| 3usize.pow(m - 1));
    if n <= 2*pot {
        n - pot
    } else {
        2*n - 3*pot
    }
}

fn main() {
    println!("First mode: {}", calc_1(ELVES));
    println!("Second mode: {}", calc_2(ELVES));
}
