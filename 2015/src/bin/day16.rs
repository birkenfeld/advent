use advtools::prelude::{HashMap, HashSet, Itertools};
use advtools::input;

const NEEDLE: &str = "\
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
";

const FORMAT: &str = r"Sue \d+: (.+): (\d+), (.+): (\d+), (.+): (\d+)";

fn main() {
    let mut needle = HashSet::new();
    let mut needle_map = HashMap::new();
    for line in NEEDLE.lines() {
        let (name, count) = line.split(": ").collect_tuple().unwrap();
        let count = count.parse().unwrap();
        needle.insert((name, count));
        needle_map.insert(name, count);
    }

    let mut haystack = Vec::new();
    for tok in input::rx_lines::<Vec<(&str, i32)>>(FORMAT) {
        haystack.push(HashSet::from_iter(tok.into_iter()));
    }

    let mut real_aunt = 0;
    for (i, hay) in haystack.into_iter().enumerate() {
        if hay.is_subset(&needle) {
            advtools::verify("Preliminary aunt", i+1, 373);
        }
        let all_ok = hay.iter().all(|(name, count)| {
            match *name {
                "cats" | "trees" => *count > needle_map[name],
                "pomeranians" | "goldfish" => *count < needle_map[name],
                _ => *count == needle_map[name]
            }
        });
        if all_ok {
            real_aunt = i+1;
        }
    }

    advtools::verify("Real aunt", real_aunt, 260);
}
