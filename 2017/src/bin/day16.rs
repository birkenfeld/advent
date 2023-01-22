use advtools::prelude::{HashSet, Itertools};
use advtools::input;

enum Move {
    RotLeft(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

/// Execute one dance on the list of dancers.
fn dance_one(dance: &[Move], dancers: &mut [u8]) {
    for move_ in dance {
        match *move_ {
            Move::RotLeft(n) => dancers.rotate_left(n),
            Move::Exchange(i, j) => dancers.swap(i, j),
            Move::Partner(a, b) => for place in dancers.iter_mut() {
                if *place == a { *place = b; }
                else if *place == b { *place = a; }
            }
        }
    }
}

/// Format dancers for printout.
fn as_string(dancers: &[u8]) -> String {
    dancers.iter().map(|b| (b + b'a') as char).collect()
}

fn main() {
    // Parse the dance steps.
    let dance = input::string().split(',').map(|mov| {
        if let Some(n) = mov.strip_prefix('s') {
            Move::RotLeft(16 - n.parse::<usize>().unwrap())
        } else if let Some(n) = mov.strip_prefix('x') {
            let (pos1, pos2) = n.split('/').collect_tuple().unwrap();
            Move::Exchange(pos1.parse().unwrap(), pos2.parse().unwrap())
        } else {
            let (prog1, prog2) = mov[1..].split('/')
                .map(|s| s.chars().next().unwrap() as u8 - b'a')
                .collect_tuple().unwrap();
            Move::Partner(prog1, prog2)
        }
    }).collect_vec();

    // Executing one billion dances is quite a lengthy task.  The idea here is
    // that after the same state is reached a second time, all further states
    // are known.
    let mut seen = HashSet::new();
    let mut dancers = (0..16).collect_vec();
    while seen.insert(dancers.clone()) {
        dance_one(&dance, &mut dancers);
    }
    let cycle_len = seen.len();

    // Part 1: execute one dance (the current `dancers` is the state after the
    // cycle, so it's the original state).
    dance_one(&dance, &mut dancers);
    advtools::verify("Order after 1 dance", as_string(&dancers), "kpfonjglcibaedhm");

    // Part 2: execute dances to reach 1bn (mod cycle_len).
    for _ in 0..(1_000_000_000 % cycle_len) - 1 {
        dance_one(&dance, &mut dancers);
    }
    advtools::verify("Order after 1 billion dances", as_string(&dancers), "odiabmplhfgjcekn");
}
