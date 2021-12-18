use advtools::itertools::Itertools;
use advtools::input;

fn main() {
    let corpus = input::lines().map(|line| u32::from_str_radix(line, 2).unwrap())
                               .collect_vec();
    let num_bits = 32 - corpus.iter().max().unwrap().leading_zeros();

    // Checks if at position `bit`, we have at least as many ones as zeros over
    // the whole corpus.
    let more_ones = |corpus: &[u32], bit: u32| {
        2 * corpus.iter().filter(|&s| s & (1 << bit) != 0).count() >= corpus.len()
    };

    // Finds the remaining value for part 2, with `ones` selecting the criterion.
    let find = |mut corpus: Vec<u32>, ones: bool| {
        for bit in (0..num_bits).rev() {
            let goal = more_ones(&corpus, bit) == ones;
            corpus.retain(|el| (el & (1 << bit) != 0) == goal);
            if corpus.len() == 1 {
                break;
            }
        }
        corpus[0]
    };

    let gamma: u32 = (0..num_bits).map(|bit| (more_ones(&corpus, bit) as u32) << bit).sum();
    let epsilon = (1 << num_bits) - 1 - gamma;
    advtools::verify("Power consumption", gamma * epsilon, 4103154);

    let oxygen = find(corpus.clone(), true);
    let scrubber = find(corpus, false);
    advtools::verify("Life support rating", oxygen * scrubber, 4245351);
}
