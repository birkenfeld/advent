use std::mem::replace;
use advtools::prelude::{Itertools, HashSet, HashMap};
use advtools::input::iter_input;
use generic_array::{GenericArray, ArrayLength, arr};

const ALL_KEYS: u32 = (1 << 26) - 1;

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Wall,
    Free,
    Key(u8),
    Door(u8),
}
use Cell::*;

type XY = (usize, usize);

fn main() {
    let mut maze = Vec::new();
    for line in iter_input::<String>() {
        maze.push(line.trim().chars().map(|ch| match ch {
            '#' => Wall,
            '.' | '@' => Free,
            'a'..='z' => Key(ch as u8 - b'a'),
            'A'..='Z' => Door(ch as u8 - b'A'),
            _ => panic!("invalid char in maze")
        }).collect_vec());
    }

    let my = maze.len()/2;
    let mx = maze[0].len()/2;

    prune_maze(&mut maze);

    advtools::print("Fewest steps with 1 robot", visit_n(&maze, arr![XY; (mx, my)]));

    maze[my][mx] = Wall;
    maze[my-1][mx] = Wall;
    maze[my][mx-1] = Wall;
    maze[my+1][mx] = Wall;
    maze[my][mx+1] = Wall;

    let start = arr![XY; (mx-1, my-1), (mx-1, my+1), (mx+1, my-1), (mx+1, my+1)];
    advtools::print("Fewest steps with 4 robots", visit_n(&maze, start));
}

/// Prune any cul-de-sacs without keys or doors from the maze.
fn prune_maze(maze: &mut [Vec<Cell>]) {
    loop {
        let mut changed = 0;
        for y in 0..maze.len() {
            for x in 0..maze[0].len() {
                if maze[y][x] == Free {
                    let neighbors = [maze[y-1][x],
                                     maze[y+1][x],
                                     maze[y][x-1],
                                     maze[y][x+1]];
                    if neighbors.iter().filter(|&n| n == &Wall).count() == 3 &&
                        neighbors.iter().filter(|&n| n == &Free).count() == 1
                    {
                        maze[y][x] = Wall;
                        changed += 1;
                    }
                }
            }
        }
        if changed == 0 {
            break;
        }
    }
}

/// Visit a maze with N robots.
fn visit_n<N>(maze: &[Vec<Cell>], start: GenericArray<XY, N>) -> u32
    where N: ArrayLength<XY>
{
    let mut fastest = HashMap::new();
    let mut next = HashMap::new();
    fastest.insert(((0, 0), 0), 0);
    let start = (start, 0, 0);
    let mut queue = vec![start];
    let mut min_steps = u32::max_value();

    loop {
        queue.sort_by_key(|k| k.2);
        // println!("{}", queue.len());
        for (xys, keys, steps) in replace(&mut queue, Vec::new()) {
            if keys == ALL_KEYS {
                min_steps = min_steps.min(steps);
                continue;
            }

            for (i, &xy) in xys.iter().enumerate() {
                let pk = next.entry((xy, keys)).or_insert_with(|| next_keys(&maze, xy, keys));
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

/// Find all keys reachable from the given position, with the given set of keys
/// already in possession.  Keys that lie behind other new keys are not considered.
fn next_keys(maze: &[Vec<Cell>], start: XY, keys: u32) -> Vec<(XY, u32, u32)> {
    let mut known = HashSet::with_capacity(4096);
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
