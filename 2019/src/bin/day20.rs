use std::{io::BufRead, mem::replace};
use advtools::prelude::{Itertools, HashSet, HashMap};
use advtools::input::input_file;

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Wall,
    Free,
    Portal,
    Entry,
    Exit,
}
use Cell::*;

type XY = (usize, usize);

fn main() {
    let maze_chars = input_file().lines().map(
        |line| line.unwrap().chars().collect_vec()
    ).collect_vec();
    let ny = maze_chars.len();
    let nx = maze_chars[0].len();

    let mut maze = Vec::new();
    let mut portal_pos = HashMap::<_, Vec<_>>::new();
    let mut entry = (0, 0);
    for (y, row) in maze_chars.iter().enumerate() {
        maze.push(row.iter().enumerate().map(|(x, &ch)| match ch {
            '.' => Free,
            // Keep track of the positions of all portals by name, and if they
            // are inner or outer ones.
            'A'..='Z' => {
                let (name, outer) = if y < ny-1 && maze_chars[y+1][x] == '.' {
                    ((maze_chars[y-1][x], ch), y < 4 || y > ny - 4)
                } else if y > 0 && maze_chars[y-1][x] == '.' {
                    ((ch, maze_chars[y+1][x]), y < 4 || y > ny - 4)
                } else if x < nx-1 && maze_chars[y][x+1] == '.' {
                    ((maze_chars[y][x-1], ch), x < 4 || x > nx - 4)
                } else if x > 0 && maze_chars[y][x-1] == '.' {
                    ((ch, maze_chars[y][x+1]), x < 4 || x > nx - 4)
                } else {
                    return Wall;
                };
                if name == ('A', 'A') {
                    entry = (x, y);
                    return Entry;
                } else if name == ('Z', 'Z') {
                    return Exit;
                }
                portal_pos.entry(name).or_default().push(((x, y), outer));
                Portal
            }
            _ => Wall,
        }).collect_vec());
    }

    // Connect same-name portals in a new mapping.
    let mut portals = HashMap::new();
    for (_, xys) in portal_pos {
        portals.insert(xys[0].0, xys[1]);
        portals.insert(xys[1].0, xys[0]);
    }

    prune_maze(&mut maze);

    advtools::print("Steps to walk to exit", walk(&maze, &portals, entry, false));
    advtools::print("Steps to walk with depth", walk(&maze, &portals, entry, true));
}

/// Prune any cul-de-sacs from the maze.
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

fn walk(maze: &[Vec<Cell>], portals: &HashMap<XY, (XY, bool)>, start: XY, recursive: bool) -> u32 {
    let mut known = HashSet::with_capacity(1<<17);
    let mut queue = vec![(start.0, start.1, 0)];
    known.insert(queue[0]);

    for steps in 1.. {
        for (x, y, depth) in replace(&mut queue, Vec::with_capacity(64)) {
            let new_xy = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
            for &(nx, ny) in &new_xy {
                match maze[ny][nx] {
                    Free => (),
                    Wall | Entry => continue,
                    Portal => {
                        let ((px, py), outer) = portals[&(nx, ny)];

                        // Find the corridor next to the exit in this step, since
                        // teleporting from corridor to corridor only counts as 1 step.
                        let &(nx, ny) = [(px, py - 1), (px, py + 1), (px - 1, py), (px + 1, py)]
                            .iter().find(|&&(xx, yy)| maze[yy][xx] == Free).unwrap();

                        let new_depth = if recursive {
                            if outer {
                                depth + 1 // going inner->outer: deeper in the recursion
                            } else {
                                depth - 1 // going back
                            }
                        } else {
                            0
                        };
                        if new_depth >= 0 {
                            if known.insert((nx, ny, new_depth)) {
                                queue.push((nx, ny, new_depth));
                            }
                            continue;
                        }
                    },
                    Exit => {
                        // Goal only counts if we are at the top level.
                        // Otherwise it's a dead end.
                        if depth == 0 {
                            return steps - 2;
                        }
                    }
                }
                if known.insert((nx, ny, depth)) {
                    queue.push((nx, ny, depth));
                }
            }
        }
    }
    unreachable!()
}
