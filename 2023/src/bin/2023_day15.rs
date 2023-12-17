use advtools::input;
use advtools::prelude::Itertools;

fn main() {
    // Definition of the hash function.
    let hash = |input: &str| input.chars().fold(0, |h, ch| (h + ch as usize) * 17) & 0xFF;

    // Part 1: just sum all the instructions.
    let hash_sum = input::string().split(',').map(hash).sum::<usize>();
    advtools::verify("Hash sum", hash_sum, 498538);

    // Part 2: do the HASHMAP algorithm.
    let mut boxes = vec![vec![]; 256];
    for part in input::string().split(',') {
        if part.ends_with('-') {
            let id = &part[..part.len()-1];
            boxes[hash(id)].retain(|(box_id, _)| box_id != &id);
        } else {
            let (id, val) = part.split('=').collect_tuple().unwrap();
            let val: usize = val.parse().unwrap();
            if let Some(entry) = boxes[hash(id)].iter_mut().find(|(box_id, _)| box_id == &id) {
                entry.1 = val;
            } else {
                boxes[hash(id)].push((id, val));
            }
        }
    }
    let focus_power = boxes.iter().enumerate().flat_map(|(i, bx)| {
        // Focusing power is box * slot * length.
        bx.iter().enumerate().map(move |(j, (_, length))| (i+1) * (j+1) * length)
    }).sum::<usize>();
    advtools::verify("Focusing power", focus_power, 286278);
}
