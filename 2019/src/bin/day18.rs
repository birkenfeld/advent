use std::mem::replace;
use advtools::prelude::{Itertools, HashSet, HashMap};
use advtools::input::iter_input;
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
enum Pos {
    Start(usize),
    Key(u8),
}

type XY = (usize, usize);

fn main() {
    let mut maze = Vec::new();
    let mut key_pos = HashMap::new();
    let mut all_keys = 0;
    // Keep track of the maze, as well as the positions of all keys.
    for (y, line) in iter_input::<String>().enumerate() {
        maze.push(line.trim().chars().enumerate().map(|(x, ch)| match ch {
            '#' => Wall,
            '.' | '@' => Free,
            'a'..='z' => {
                let nkey = ch as u8 - b'a';
                key_pos.insert(nkey, (x, y));
                all_keys |= 1 << nkey;
                Key(nkey)
            }
            'A'..='Z' => Door(ch as u8 - b'A'),
            _ => panic!("invalid char in maze")
        }).collect_vec());
    }

    let my = maze.len()/2;
    let mx = maze[0].len()/2;

    advtools::print("Fewest steps with 1 robot",
                    visit_n(&maze, &key_pos, all_keys, arr![XY; (mx, my)]));

    maze[my-1][mx] = Wall;
    maze[my][mx-1] = Wall;
    maze[my+1][mx] = Wall;
    maze[my][mx+1] = Wall;

    let start = arr![XY; (mx-1, my-1), (mx-1, my+1), (mx+1, my-1), (mx+1, my+1)];
    advtools::print("Fewest steps with 4 robots",
                    visit_n(&maze, &key_pos, all_keys, start));
}

/// Visit a maze with N robots.
fn visit_n<N>(maze: &[Vec<Cell>], key_pos: &HashMap<u8, XY>, all_keys: u32,
              start_pos: GenericArray<XY, N>) -> u32
    where N: ArrayLength<XY> + ArrayLength<Pos>
{
    let mut fastest = HashMap::new();
    let mut min_steps = u32::max_value();

    // Calculate the neighboring keys for each starting position and each key.
    let mut key_edges = HashMap::new();
    for (i, &pos) in start_pos.iter().enumerate() {
        key_edges.insert(Pos::Start(i), neighbor_keys(maze, 0, pos));
    }
    for (&key, &pos) in key_pos {
        key_edges.insert(Pos::Key(key), neighbor_keys(maze, 1 << key, pos));
    }

    // This will be much easier with const generics.
    let start_pos: GenericArray<Pos, N> = GenericArray::generate(Pos::Start);
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
                                new_at_keys[i] = Pos::Key(new_key);
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
fn neighbor_keys(maze: &[Vec<Cell>], keys_ignore: u32, start: XY) -> Vec<(u8, u32, u32)> {
    let mut known = HashSet::with_capacity(4096);
    known.insert(start);
    let mut res = Vec::with_capacity(4);
    let mut queue = vec![(start.0, start.1, 0)];

    for steps in 1.. {
        for (x, y, req_keys) in replace(&mut queue, Vec::with_capacity(16)) {
            let new_xy = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
            for &(nx, ny) in &new_xy {
                match maze[ny][nx] {
                    Free => (),
                    Wall => continue,
                    Key(this_key) => if keys_ignore & (1 << this_key) == 0 {
                        res.push((this_key, req_keys, steps));
                        known.insert((nx, ny));
                        continue;
                    },
                    Door(this_door) => if req_keys & (1 << this_door) == 0 {
                        queue.push((nx, ny, req_keys | (1 << this_door)));
                        known.insert((nx, ny));
                        continue;
                    },
                }
                if known.insert((nx, ny)) {
                    queue.push((nx, ny, req_keys));
                }
            }
        }
        if queue.is_empty() {
            return res;
        }
    }
    unreachable!()
}
