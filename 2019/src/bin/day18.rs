use std::mem::replace;
use advtools::prelude::{Itertools, HashSet, HashMap};
use advtools::input::iter_input;

const ALL_KEYS: u32 = (1 << 26) - 1;

enum Cell {
    Wall,
    Free,
    Key(u8),
    Door(u8),
}
use Cell::*;

fn main() {
    let mut grid = Vec::new();
    for line in iter_input::<String>() {
        grid.push(line.trim().chars().map(|ch| match ch {
            '#' => Wall,
            '.' | '@' => Free,
            'a'..='z' => Key(ch as u8 - b'a'),
            'A'..='Z' => Door(ch as u8 - b'A'),
            _ => panic!("invalid char in maze")
        }).collect_vec());
    }

    let ny = grid.len()/2;
    let nx = grid[0].len()/2;

    advtools::print("Fewest steps with 1 robot", visit_1(&grid, (nx, ny)));

    grid[ny][nx] = Wall;
    grid[ny-1][nx] = Wall;
    grid[ny][nx-1] = Wall;
    grid[ny+1][nx] = Wall;
    grid[ny][nx+1] = Wall;

    let start = [(nx-1, ny-1), (nx-1, ny+1), (nx+1, ny-1), (nx+1, ny+1)];
    advtools::print("Fewest steps with 4 robots", visit_4(&grid, start));
}

fn visit_1(maze: &[Vec<Cell>], start: (usize, usize)) -> u32 {
    let mut known = HashSet::new();
    // known states: position + found keys
    let start = (start.0, start.1, 0);
    known.insert(start);
    let mut queue = vec![start];
    for steps in 1.. {
        for (x, y, keys) in std::mem::replace(&mut queue, Vec::new()) {
            // Go through every (potential) new location and find out if
            // we can go there.
            let new_coords = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
            for &(xn, yn) in &new_coords {
                match maze[yn][xn] {
                    Wall => continue,
                    Free => (),
                    Key(this_key) => if keys & (1 << this_key) == 0 {
                        // found a new key!
                        let new_keys = keys | (1 << this_key);

                        if new_keys == ALL_KEYS {
                            return steps;
                        }

                        known.insert((xn, yn, new_keys));
                        queue.push((xn, yn, new_keys));
                        continue;
                    }
                    Door(this_door) => if keys & (1 << this_door) == 0 {
                        // no such key...
                        continue;
                    }
                }
                if known.insert((xn, yn, keys)) {
                    queue.push((xn, yn, keys));
                }
            }
        }
        if queue.is_empty() {
            // We already filled everything last time.
            panic!("goal not found");
        }
    }
    unreachable!()
}

fn visit_4(maze: &[Vec<Cell>], start: [(usize, usize); 4]) -> u32 {
    let mut fastest = HashMap::new();
    let mut possible = HashMap::new();
    fastest.insert(0, 0);
    let start = (start, 0, 0);
    let mut queue = vec![start];
    let mut min_steps = u32::max_value();

    let mut tmp = HashSet::default();
    loop {
        for (xys, keys, steps) in replace(&mut queue, Vec::new()) {
            if steps > min_steps {
                continue;
            }
            if keys == ALL_KEYS {
                min_steps = min_steps.min(steps);
                continue;
            }

            for (i, &xy) in xys.iter().enumerate() {
                let pk = possible.entry((xy, keys)).or_insert_with(
                    || possible_keys(&mut tmp, &maze, xy, keys));
                for &(nxy, nkeys, ksteps) in pk.iter() {
                    match fastest.get(&nkeys) {
                        Some(&n) if n <= steps + ksteps => (),
                        _ => {
                            fastest.insert(nkeys, steps + ksteps);
                            let mut new_xys = xys;
                            new_xys[i] = nxy;
                            queue.push((new_xys, nkeys, steps + ksteps));
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

fn possible_keys(known: &mut HashSet<(usize, usize)>, maze: &[Vec<Cell>], start: (usize, usize), keys: u32)
                 -> Vec<((usize, usize), u32, u32)> {
    // print!(".");
    known.clear();
    known.insert(start);
    let mut keys_found = keys;
    let mut res = Vec::with_capacity(4);
    let mut queue = vec![start];

    for steps in 1.. {
        for (x, y) in replace(&mut queue, Vec::with_capacity(16)) {
            let new_xy = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
            for &(nx, ny) in &new_xy {
                match maze[ny][nx] {
                    Free => (),
                    Wall => continue,
                    Key(this_key) => if keys_found & (1 << this_key) == 0 {
                        keys_found |= 1 << this_key;
                        res.push(((nx, ny), keys | (1 << this_key), steps));
                        known.insert((nx, ny));
                        continue;
                    },
                    Door(this_door) => if keys & (1 << this_door) == 0 {
                        continue;
                    },
                }
                if known.insert((nx, ny)) {
                    queue.push((nx, ny));
                }
            }
        }
        if queue.is_empty() {
            return res;
        }
    }
    unreachable!()
}
