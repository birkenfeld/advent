use advtools::prelude::{HashMap, HashSet, FromIterator};
use advtools::input::{iter_input_trim, parse_parts_trim};

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

fn main() {
    let mut needle = HashSet::new();
    let mut needle_map = HashMap::new();
    for line in NEEDLE.lines() {
        let (name, count): (String, i32) = parse_parts_trim(&line, [0, 1], ":");
        needle.insert((name.clone(), count));
        needle_map.insert(name, count);
    }

    let mut haystack = Vec::new();
    for tok in iter_input_trim::<Vec<(String, i32)>>(":,") {
        haystack.push(HashSet::from_iter(tok.into_iter().skip(1)));
    }

    let mut real_aunt = 0;
    for (i, hay) in haystack.into_iter().enumerate() {
        if hay.is_subset(&needle) {
            advtools::verify("Preliminary aunt", i+1, 373);
        }
        let all_ok = hay.iter().all(|(name, count)| {
            match &**name {
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
