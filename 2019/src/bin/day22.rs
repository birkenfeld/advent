use advtools::prelude::Itertools;
use advtools::input::iter_input;

#[derive(Clone, Copy)]
enum Op {
    Deal(i128),
    Cut(i128),
    NewStack,
}

const CARDS_1: i128 = 10007;
const TGT_CARD_1: i128 = 2019;

const CARDS_2: i128 = 119315717514047;
const TIMES_2: i128 = 101741582076661;
const TGT_POS_2: i128 = 2020;

/// Exponentiate two integers modulo m by repeated squaring.
fn mod_exp(a: i128, e: i128, m: i128) -> i128 {
    if e == 1 { a } else {
        let x = mod_exp(a, e/2, m);
        let r = (x * x) % m;
        if e & 1 == 1 { (r * a) % m } else { r }
    }
}

/// "Divide" two integers modulo m, by finding the multiplicative
/// inverse. Assumes that m is prime!
fn mod_div(a: i128, b: i128, m: i128) -> i128 {
    (a * mod_exp(b, m - 2, m)).rem_euclid(m)
}

fn main() {
    let ops = iter_input::<String>().map(|line| {
        let split = line.split_whitespace().collect_vec();
        match (split[0], split[1]) {
            ("cut", x) => Op::Cut(x.parse().unwrap()),
            ("deal", "into") => Op::NewStack,
            _ => Op::Deal(split[3].parse().unwrap())
        }
    }).collect_vec();

    // For part 1, apply the operations forward to find where a particular card
    // ends up. We combine all operations into a total linear transformation of
    // the form (a*x + b) on the original card's position.
    let mut a = 1;
    let mut b = 0;
    for &op in &ops {
        match op {
            Op::NewStack => { a = -a; b = -b - 1; }
            Op::Cut(n) => { b -= n; }
            Op::Deal(n) => { a *= n; b *= n; }
        };
        // Keep coefficients small to avoid overflow.
        a %= CARDS_1;
        b %= CARDS_1;
    }
    let pos_2019 = (a*TGT_CARD_1 + b).rem_euclid(CARDS_1);
    advtools::verify("Position of card 2019", pos_2019, 8379);

    // For part 2, apply the operations backward to find where a particular
    // card position at the end comes from.
    let mut a = 1;
    let mut b = 0;
    for &op in ops.iter().rev() {
        match op {
            Op::NewStack => { a = -a; b = -b - 1; }
            Op::Cut(n) => { b += n; }
            Op::Deal(n) => { a = mod_div(a, n, CARDS_2);
                             b = mod_div(b, n, CARDS_2); }
        };
        a %= CARDS_2;
        b %= CARDS_2;
    }
    // Now take the linear transformation to power N = TIMES_2. This results in
    //     a^N x + b*sum(1, a, a^2, ..., a^(N-1))
    // the latter of which can be calculated as a geometric series sum.
    let a_n = mod_exp(a, TIMES_2, CARDS_2);
    let b_n = mod_div((b * (a_n - 1)) % CARDS_2, a - 1, CARDS_2);

    // Now apply the full sequence to the target position, which gives us the
    // original position in the sorted sequence.
    let in_pos_2020 = (a_n*TGT_POS_2 + b_n).rem_euclid(CARDS_2);
    advtools::verify("Ends up in pos 2020", in_pos_2020,
                     96959315590030_i128);
}
