use advtools::input;
use advtools::prelude::Itertools;

const CARDS_1: [char; 13] = ['2', '3', '4', '5', '6', '7', '8',
                             '9', 'T', 'J', 'Q', 'K', 'A'];
const CARDS_2: [char; 13] = ['J', '2', '3', '4', '5', '6', '7',
                             '8', '9', 'T', 'Q', 'K', 'A'];

fn winnings(values: &[char], jokers: Option<usize>) -> u64 {
    let mut hands = Vec::new();
    for (hand, bid) in input::rx_lines::<(&str, u64)>(r"([^ ]+) (\d+)") {
        // Map card names to their rank.
        let hand = hand.chars().map(
            |c| values.iter().position(|&x| x == c).unwrap()
        ).collect_vec();
        // Count and sort occurrences of each card, without jokers.
        let counts = hand.iter().counts().into_iter()
                                         .filter(|&(&c, _)| Some(c) != jokers)
                                         .sorted_by_key(|&(_, c)| c)
                                         .rev()
                                         .collect_vec();
        // Determine hand type. It actually doesn't matter if jokers are used.
        let hand_type = match counts.len() {
            0 | 1 => 6,                    // 5 of a kind
            2 if counts[1].1 == 1 => 5,    // 4 of a kind
            2 => 4,                        // Full house
            3 if counts[1].1 == 1 => 3,    // 3 of a kind
            3 => 2,                        // 2 pairs
            4 => 1,                        // 1 pair
            _ => 0                         // Nothing
        };
        hands.push((hand_type, hand, bid));
    }
    // Sort by type then high card, and calculate bids.
    hands.iter().sorted()
                .enumerate()
                .map(|(i, (_, _, bid))| (1 + i as u64) * bid)
                .sum()
}

fn main() {
    let part1 = winnings(&CARDS_1, None);
    advtools::verify("Total winnings", part1, 247961593);

    let part2 = winnings(&CARDS_2, Some(0));
    advtools::verify("Total winnings with jokers", part2, 248750699);
}
