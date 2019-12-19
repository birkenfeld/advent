use std::mem::replace;
use advtools::prelude::{Itertools, HashSet, HashMap};
use advtools::input::iter_input;
use generic_array::{GenericArray, ArrayLength, arr};

const ALL_KEYS: u32 = (1 << 26) - 1;

enum Cell {
    Wall,
    Free,
    Key(u8),
    Door(u8),
}
use Cell::*;

type XY = (usize, usize);

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

    advtools::print("Fewest steps with 1 robot", visit_n(&grid, arr![XY; (nx, ny)]));

    grid[ny][nx] = Wall;
    grid[ny-1][nx] = Wall;
    grid[ny][nx-1] = Wall;
    grid[ny+1][nx] = Wall;
    grid[ny][nx+1] = Wall;

    let start = arr![XY; (nx-1, ny-1), (nx-1, ny+1), (nx+1, ny-1), (nx+1, ny+1)];
    advtools::print("Fewest steps with 4 robots", visit_n(&grid, start));
}

fn visit_n<N>(maze: &[Vec<Cell>], start: GenericArray<XY, N>) -> u32
    where N: ArrayLength<XY>
{
    let mut fastest = HashMap::new();
    let mut possible = HashMap::new();
    fastest.insert(((0, 0), 0), 0);
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
                    match fastest.get(&(nxy, nkeys)) {
                        Some(&n) if n <= steps + ksteps => (),
                        _ => {
                            fastest.insert((nxy, nkeys), steps + ksteps);
                            let mut new_xys = xys.clone();
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

fn possible_keys(known: &mut HashSet<XY>, maze: &[Vec<Cell>], start: XY, keys: u32) -> Vec<(XY, u32, u32)> {
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
