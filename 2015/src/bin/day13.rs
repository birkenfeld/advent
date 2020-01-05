use advtools::prelude::{Itertools, Uids};
use advtools::input::iter_input_regex;

fn most_happiness(n: usize, table: &[[i16; 9]; 9]) -> i16 {
    (0..n).permutations(n).map(|p| {
        p.iter().zip(p.iter().cycle().skip(n - 1)).map(
            |(&p1, &p2)| table[p1][p2] + table[p2][p1]
        ).sum()
    }).max().unwrap()
}

fn main() {
    let mut table = [[0i16; 9]; 9];
    let mut map = Uids::new();
    for row in iter_input_regex(r"(.*) would (.*) (\d+) happiness .* to (.*)\.") {
        let (p1, verb, val, p2): (String, String, i16, String) = row;
        let val = if verb == "gain" { val } else { -val };
        let p1_id = map.get_id(p1);
        let p2_id = map.get_id(p2);
        table[p1_id][p2_id] = val;
    }
    advtools::verify("Most happiness", most_happiness(8, &table), 664);
    advtools::verify("Most happiness including self", most_happiness(9, &table), 640);
}
