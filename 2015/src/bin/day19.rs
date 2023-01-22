use advtools::prelude::{HashMap, HashSet};
use advtools::input;
use rand::{thread_rng, prelude::SliceRandom};

fn make_one_replacement(initial: &str, trans: &HashMap<&str, Vec<&str>>) -> HashSet<String> {
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

fn find_steps(initial: &str, target: &str, rtbl: &HashMap<&str, &str>) -> Option<usize> {
    let mut repls: Vec<_> = rtbl.keys().collect();
    repls.shuffle(&mut thread_rng());
    let mut steps = 0;
    let mut cur = initial.to_owned();
    loop {
        let old_steps = steps;
        for i in (0..cur.len() - 2).rev() {
            for &repl in &repls {
                if cur[i..].starts_with(repl) {
                    cur = cur[..i].to_owned() + rtbl[repl] + &cur[i+repl.len()..];
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
    let mut trans: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut target = "";
    for mut line in input::parse_lines::<Vec<&str>>() {
        if line.len() == 3 {
            let key = line.remove(0);
            let val = line.remove(1);
            trans.entry(key).or_default().push(val);
        } else {
            target = line.remove(0);
        }
    }
    let rtrans = trans.iter().flat_map(|(k, vs)| vs.iter().map(|v| (*v, *k))).collect();
    let variants = make_one_replacement(target, &trans);
    advtools::verify("# distinct molecules for calibration", variants.len(), 509);
    loop {
        if let Some(steps) = find_steps(target, "e", &rtrans) {
            advtools::verify("# steps for making target", steps, 195);
            return;
        }
    }
}
