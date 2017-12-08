extern crate advtools;
use advtools::prelude::*;

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
        let tok = line.split_whitespace().collect::<Vec<_>>();
        let name = tok[0].trim_matches(':').to_owned();
        let count = tok[1].parse::<i32>().unwrap();
        needle.insert((name.clone(), count));
        needle_map.insert(name, count);
    }
    let mut haystack = Vec::new();
    for tok in iter_input::<Vec<String>>() {
        let mut set = HashSet::new();
        for i in 1..4 {
            set.insert((tok[2*i].trim_matches(':').to_owned(),
                        to_i32(tok[2*i+1].trim_matches(','))));
        }
        haystack.push(set);
    }
    for (i, hay) in haystack.into_iter().enumerate() {
        if hay.is_subset(&needle) {
            println!("Preliminary aunt: {}", i+1);
        }
        let all_ok = hay.iter().all(|&(ref name, count)| {
            match &**name {
                "cats" | "trees" => count > needle_map[name],
                "pomeranians" | "goldfish" => count < needle_map[name],
                _ => count == needle_map[name]
            }
        });
        if all_ok {
            println!("Real aunt: {}", i+1);
        }
    }
}
