use advtools::prelude::{HashMap, HashSet};
use advtools::{Uids, input::iter_input_regex};

fn calc_indirect(orbits: &HashMap<usize, usize>, indirect: &mut HashMap<usize, i32>, obj: usize) {
    if indirect.get(&obj).is_none() {
        calc_indirect(orbits, indirect, orbits[&obj]);
        indirect.insert(obj.into(), indirect[&orbits[&obj]] + 1);
    }
}

fn main() {
    // Create a map of all "orbits" relations from object to its center.
    let mut ids = Uids::<String>::new();
    let orbits: HashMap<_, _> = iter_input_regex(r"(\w+)\)(\w+)").map(|(c, o)| {
        (ids.get_id(o), ids.get_id(c))
    }).collect();
    // Extract ids for known objects.
    let (com, you, san) = (ids["COM"], ids["YOU"], ids["SAN"]);

    // Determine indirect orbit lengths for each object.
    let mut indirect = std::iter::once((com, 0)).collect();
    orbits.keys().for_each(|&obj| calc_indirect(&orbits, &mut indirect, obj));
    advtools::print("First round", indirect.values().sum::<i32>());

    let walk_centers = |mut obj, f: &mut dyn FnMut(usize) -> bool| loop {
        if obj == com || !f(obj) {
            return obj;
        }
        obj = orbits[&obj];
    };

    // List all indirect orbit centers for YOU and compare with SAN to get
    // the path length between the two and the common center.
    let mut my_centers = HashSet::new();
    walk_centers(you, &mut |p| my_centers.insert(p));
    let p = walk_centers(san, &mut |p| !my_centers.contains(&p));
    let path = indirect[&you] + indirect[&san] - 2*indirect[&p] - 2;
    advtools::print("Second round", path);
}
