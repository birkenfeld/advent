use advtools::prelude::Itertools;
use advtools::input;

fn main() {
    let mut easy_digits = 0;
    let mut total_sum = 0;

    // Convert abcdefg notation to integers with single bits representing wires.
    let str_to_int = |s: &&str| s.chars()
                                 .map(|ch| ch as u8 - b'a')
                                 .fold(0u8, |pat, bit| pat | (1 << bit));

    for data in input::parse_lines::<Vec<&str>>() {
        // Get the patterns in this order:
        // [1-pattern, 7-pattern, 4-pattern, (2,5,3)-patterns, (0,6,9)-patterns]
        let all = data[..10].iter().map(str_to_int)
                                   .sorted_by_key(|pat| pat.count_ones()).collect_vec();


        // Find the "c/f" wires: those displayed for 1
        let cf = all[0];
        // Find the "a" wire: displayed for 7 and not for 1
        let a = all[1] - cf;
        // Find the "b/d" wires: displayed for 4 and not for 1
        let bd = all[2] - cf;
        // The "e/g" wires are the rest
        let eg = 0b1111111 - a - bd - cf;

        // Determine which is c/d/e by finding those not present in all 6-wire patterns
        let c = cf - all[6..].iter().fold(cf, |acc, item| acc & item);
        let d = bd - all[6..].iter().fold(bd, |acc, item| acc & item);
        let e = eg - all[6..].iter().fold(eg, |acc, item| acc & item);

        // Now we have all wires: decode the displayed digits
        total_sum += data[11..].iter().map(str_to_int).map(|pat| {
            let n_lit = pat.count_ones();

            // Part 1
            if matches!(n_lit, 2 | 3 | 4 | 7) {
                easy_digits += 1;
            }

            // Part 2: decode digit depending on the lit wires
            match (n_lit, pat & e, pat & c, pat & d) {
                (2, _, _, _) => 1,
                (3, _, _, _) => 7,
                (4, _, _, _) => 4,
                (5, 0, 0, _) => 5,
                (5, 0, _, _) => 3,
                (5, _, _, _) => 2,
                (6, _, 0, _) => 6,
                (6, _, _, 0) => 0,
                (6, _, _, _) => 9,
                (7, _, _, _) => 8,
                _ => unreachable!()
            }
        }).fold(0, |acc, d| 10*acc + d);
    }

    advtools::verify("Number of easy digits", easy_digits, 284);
    advtools::verify("Sum of displays", total_sum, 973499);
}
