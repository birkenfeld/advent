use std::mem::replace;
use advtools::prelude::{HashSet, HashMap};
use advtools::input::iter_input;
use advtools::grid::{Grid, Pos};
use generic_array::{GenericArray, ArrayLength, arr, sequence::GenericSequence};

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Wall,
    Free,
    Key(u8),
    Door(u8),
}
use Cell::*;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Loc {
    Start(usize),
    Key(u8),
}

fn main() {
    let mut key_pos = HashMap::new();
    let mut all_keys = 0;
    // Keep track of the maze, as well as the positions of all keys.
    let mut maze = Grid::new(iter_input::<String>().enumerate().map(|(y, line)| {
        line.trim().chars().enumerate().map(|(x, ch)| match ch {
            '#' => Wall,
            '.' | '@' => Free,
            'a'..='z' => {
                let nkey = ch as u8 - b'a';
                key_pos.insert(nkey, Pos(x as i32, y as i32));
                all_keys |= 1 << nkey;
                Key(nkey)
            }
            'A'..='Z' => Door(ch as u8 - b'A'),
            _ => panic!("invalid char in maze")
        }).collect()
    }));
    let center = Pos(maze.width() as i32/2, maze.height() as i32/2);

    advtools::verify("Fewest steps with 1 robot",
                    visit_n(&maze, &key_pos, all_keys, arr![Pos; center]), 3918);

    maze.for_neighbors(center, |p| *p = Wall);

    let start = arr![Pos; center.left().down(), center.left().up(),
                     center.right().down(), center.right().up()];
    advtools::verify("Fewest steps with 4 robots",
                    visit_n(&maze, &key_pos, all_keys, start), 2004);
}

/// Visit a maze with N robots.
fn visit_n<N>(maze: &Grid<Cell>, key_pos: &HashMap<u8, Pos>, all_keys: u32,
              start_pos: GenericArray<Pos, N>) -> u32
    where N: ArrayLength<Pos> + ArrayLength<Loc>
{
    let mut fastest = HashMap::new();
    let mut min_steps = u32::max_value();

    // Calculate the neighboring keys for each starting position and each key.
    let mut key_edges = HashMap::new();
    for (i, &pos) in start_pos.iter().enumerate() {
        key_edges.insert(Loc::Start(i), neighbor_keys(maze, 0, pos));
    }
    for (&key, &pos) in key_pos {
        key_edges.insert(Loc::Key(key), neighbor_keys(maze, 1 << key, pos));
    }

    // This will be much easier with const generics.
    let start_pos: GenericArray<Loc, N> = GenericArray::generate(Loc::Start);
    let start = (start_pos, 0, 0);
    let mut queue = vec![start];

    loop {
        queue.sort_by_key(|k| k.2);
        for (at_keys, keys, steps) in replace(&mut queue, Vec::new()) {
            // Found all keys? Find the minimum steps to reach this state.
            if keys == all_keys {
                min_steps = min_steps.min(steps);
                continue;
            }

            // Check all robots for which key to consider next.
            for (i, at_key) in at_keys.iter().enumerate() {
                for &(new_key, req_keys, ksteps) in key_edges[at_key].iter() {
                    // Consider this edge if we have all required keys.
                    if keys | req_keys == keys {
                        let new_keys = keys | (1 << new_key);
                        let new_steps = steps + ksteps;
                        // Check if we arrived at this state (i.e., collected
                        // this new key with the same total set of keys) before.
                        match fastest.get(&(new_key, new_keys)) {
                            Some(&prev_steps) if prev_steps <= new_steps => (),
                            _ => {
                                // This is the fastest way here. Go on.
                                fastest.insert((new_key, new_keys), new_steps);
                                let mut new_at_keys = at_keys.clone();
                                new_at_keys[i] = Loc::Key(new_key);
                                queue.push((new_at_keys, new_keys, new_steps));
                            }
                        }
                    }
                }
            }
        }
        if queue.is_empty() {
            return min_steps;
        }
    }
}

/// Get a list of keys directly neighboring the given position,
/// together with the required keys to pass any doors encountered.
fn neighbor_keys(maze: &Grid<Cell>, keys_ignore: u32, start: Pos) -> Vec<(u8, u32, u32)> {
    let mut known = HashSet::with_capacity(4096);
    known.insert(start);
    let mut res = Vec::with_capacity(4);
    let mut queue = vec![(start, 0)];

    for steps in 1.. {
        for (pos, req_keys) in replace(&mut queue, Vec::with_capacity(16)) {
            for new_pos in pos.neighbors() {
                match maze[new_pos] {
                    Free => (),
                    Wall => continue,
                    Key(this_key) => if keys_ignore & (1 << this_key) == 0 {
                        res.push((this_key, req_keys, steps));
                        known.insert(new_pos);
                        continue;
                    },
                    Door(this_door) => if req_keys & (1 << this_door) == 0 {
                        queue.push((new_pos, req_keys | (1 << this_door)));
                        known.insert(new_pos);
                        continue;
                    },
                }
                if known.insert(new_pos) {
                    queue.push((new_pos, req_keys));
                }
            }
        }
        if queue.is_empty() {
            return res;
        }
    }
    unreachable!()
}
