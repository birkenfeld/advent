use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufReader, BufRead};

const NEEDLE: &'static str = "\
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
    for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let tok = line.split_whitespace().collect::<Vec<_>>();
        let mut set = HashSet::new();
        for i in 1..4 {
            set.insert((tok[2*i].trim_matches(':').to_owned(),
                        tok[2*i+1].trim_matches(',').parse::<i32>().unwrap()));
        }
        haystack.push(set);
    }
    for (i, hay) in haystack.into_iter().enumerate() {
        if hay.is_subset(&needle) {
            println!("Preliminary aunt: {}", i+1);
        }
        if hay.iter().all(|&(ref name, count)| {
            match &**name {
                "cats" | "trees" => count > needle_map[name],
                "pomeranians" | "goldfish" => count < needle_map[name],
                _ => count == needle_map[name]
            }
        }) {
            println!("Real aunt: {}", i+1);
        }
    }
}
