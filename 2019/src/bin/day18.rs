use advtools::prelude::{Itertools, HashSet, HashMap};
use advtools::input::iter_input;

fn main() {
    let mut grid = Vec::new();
    let mut start_pos = (0, 0);
    for (y, line) in iter_input::<String>().enumerate() {
        if let Some(x) = line.trim().chars().position(|ch| ch == '@') {
            start_pos = (x, y);
        }
        grid.push(line.trim().chars().collect_vec());
    }

    advtools::print("Fewest steps with 1 robot", visit_1(&grid, start_pos));

    let ny = grid.len()/2;
    let nx = grid[0].len()/2;

    let mut grid2 = grid.clone();
    grid2[ny][nx] = '#';
    grid2[ny-1][nx] = '#';
    grid2[ny][nx-1] = '#';
    grid2[ny+1][nx] = '#';
    grid2[ny][nx+1] = '#';

    advtools::print("Fewest steps with 4 robots", visit_4(&grid2,
                                                          (nx-1, ny-1),
                                                          (nx-1, ny+1),
                                                          (nx+1, ny-1),
                                                          (nx+1, ny+1)));
}

fn visit_1(maze: &[Vec<char>], start: (usize, usize)) -> u32 {
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
                    '#' => continue,
                    '.' | '@' => (),
                    ch @ 'a'..='z' => {
                        let this_key = ch as u32 - b'a' as u32;
                        if keys & (1 << this_key) == 0 {
                            // found a new key!
                            let new_keys = keys | (1 << this_key);

                            if new_keys == (1 << 26) - 1 {
                                return steps;
                            }

                            known.insert((xn, yn, new_keys));
                            queue.push((xn, yn, new_keys));
                            continue;
                        }
                    }
                    ch @ 'A'..='Z' => {
                        let this_door = ch as u32 - b'A' as u32;
                        if keys & (1 << this_door) == 0 {
                            // no such key...
                            continue;
                        }
                    }
                    _ => panic!("invalid char in maze")
                }
                if known.contains(&(xn, yn, keys)) {
                    continue;
                }
                known.insert((xn, yn, keys));
                queue.push((xn, yn, keys));
            }
        }
        if queue.is_empty() {
            // We already filled everything last time.
            panic!("goal not found");
        }
    }
    unreachable!()
}

fn visit_4(maze: &[Vec<char>], start1: (usize, usize), start2: (usize, usize),
           start3: (usize, usize), start4: (usize, usize)) -> u32 {
    let mut known = HashMap::new();
    known.insert(0, 0);
    let start = (start1, start2, start3, start4, 0, 0);
    let mut queue = vec![start];
    let all_keys = (1 << 26) - 1;
    let mut min_steps = u32::max_value();

    loop {
        for (xy1, xy2, xy3, xy4, keys, steps) in std::mem::replace(&mut queue, Vec::new()) {
            if steps > min_steps {
                continue;
            }
            if keys == all_keys {
                min_steps = min_steps.min(steps);
                continue;
            }
            let p1 = possible_keys(&maze, xy1, keys)
                .map(|(nxy1, nkeys, ksteps)| (nxy1, xy2, xy3, xy4, nkeys, steps + ksteps));
            let p2 = possible_keys(&maze, xy2, keys)
                .map(|(nxy2, nkeys, ksteps)| (xy1, nxy2, xy3, xy4, nkeys, steps + ksteps));
            let p3 = possible_keys(&maze, xy3, keys)
                .map(|(nxy3, nkeys, ksteps)| (xy1, xy2, nxy3, xy4, nkeys, steps + ksteps));
            let p4 = possible_keys(&maze, xy4, keys)
                .map(|(nxy4, nkeys, ksteps)| (xy1, xy2, xy3, nxy4, nkeys, steps + ksteps));
            for possible in p1.chain(p2).chain(p3).chain(p4) {
                match known.get(&possible.4) {
                    Some(&n) if n <= possible.5 => (),
                    _ => {
                        known.insert(possible.4, possible.5);
                        queue.push(possible);
                    }
                }
            }
        }
        if queue.is_empty() {
            return min_steps;
        }
    }
}

fn possible_keys(maze: &[Vec<char>], start: (usize, usize), keys: u32)
                 -> impl Iterator<Item=((usize, usize), u32, u32)> {
    let mut known = HashSet::new();
    let start = (start.0, start.1, keys);
    known.insert(start);
    let mut queue = vec![start];
    let mut res = HashMap::new();

    for steps in 1.. {
        for (x, y, keys) in std::mem::replace(&mut queue, Vec::new()) {
            let new_xy = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
            for &(nx, ny) in &new_xy {
                match maze[ny][nx] {
                    '#' => continue,
                    '.' | '@' => (),
                    ch @ 'a'..='z' => {
                        let this_key = ch as u32 - b'a' as u32;
                        if keys & (1 << this_key) == 0 {
                            // found a new key!
                            let new_keys = keys | (1 << this_key);
                            match res.get(&new_keys) {
                                Some((_, n)) if *n <= steps => (),
                                _ => { res.insert(new_keys, ((nx, ny), steps)); }
                            }

                            known.insert((nx, ny, new_keys));
                            queue.push((nx, ny, new_keys));
                            continue;
                        }
                    }
                    ch @ 'A'..='Z' => {
                        let this_door = ch as u32 - b'A' as u32;
                        if keys & (1 << this_door) == 0 {
                            // no such key...
                            continue;
                        }
                    }
                    _ => panic!("invalid char in maze")
                }
                if known.contains(&(nx, ny, keys)) {
                    continue;
                }
                known.insert((nx, ny, keys));
                queue.push((nx, ny, keys));
            }
        }
        if queue.is_empty() {
            return res.into_iter().map(|(k, v)| (v.0, k, v.1))
        }
    }
    unreachable!()
}
