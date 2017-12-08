extern crate advtools;
extern crate rand;

use advtools::prelude::*;
use rand::{thread_rng, Rng};

fn make_one_replacement(initial: &str, trans: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut variants = HashSet::new();
    for (key, repls) in trans {
        for (i, _) in initial.match_indices(key) {
            for repl in repls {
                variants.insert(initial[..i].to_owned() + repl + &initial[i+key.len()..]);
            }
        }
    }
    variants
}

fn find_steps(initial: &str, target: &str, rtbl: &HashMap<String, String>) -> Option<usize> {
    let mut repls: Vec<_> = rtbl.keys().collect();
    thread_rng().shuffle(&mut repls);
    let mut steps = 0;
    let mut cur = initial.to_owned();
    loop {
        let old_steps = steps;
        for i in (0..cur.len() - 2).rev() {
            for repl in &repls {
                if cur[i..].starts_with(*repl) {
                    cur = cur[..i].to_owned() + &rtbl[*repl] + &cur[i+repl.len()..];
                    steps += 1;
                }
            }
        }
        if old_steps == steps {
            return None;
        } else if cur == target {
            return Some(steps);
        }
    }
}

fn main() {
    let mut trans: HashMap<String, Vec<String>> = HashMap::new();
    let mut rtrans = HashMap::new();
    let mut target = String::new();
    for mut line in iter_input::<Vec<String>>() {
        if line.len() == 3 {
            let key = line.remove(0);
            let val = line.remove(1);
            trans.entry(key.clone()).or_insert(vec![]).push(val.clone());
            rtrans.insert(val, key);
        } else {
            target = line.remove(0);
        }
    }
    let variants = make_one_replacement(&target, &trans);
    println!("# distinct molecules for calibration: {}", variants.len());
    loop {
        if let Some(steps) = find_steps(&target, "e", &rtrans) {
            println!("# steps for making target: {}", steps);
            break;
        }
    }
}
