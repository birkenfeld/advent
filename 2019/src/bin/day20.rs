use std::{io::BufRead, mem::replace};
use advtools::prelude::{Itertools, HashSet, HashMap};
use advtools::input::input_file;
use advtools::grid::{Grid, Pos};

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Wall,
    Free,
    Portal,
    Entry,
    Exit,
}
use Cell::*;

fn main() {
    let maze_chars = input_file().lines().map(
        |line| line.unwrap().chars().collect_vec()
    ).collect_vec();
    let ny = maze_chars.len();
    let nx = maze_chars[0].len();

    let mut portal_pos = HashMap::<_, Vec<_>>::new();
    let mut entry = Pos(0, 0);
    let mut maze = Grid::new(maze_chars.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, &ch)| match ch {
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
                    entry = Pos(x as i32, y as i32);
                    return Entry;
                } else if name == ('Z', 'Z') {
                    return Exit;
                }
                portal_pos.entry(name).or_default().push((Pos(x as i32, y as i32), outer));
                Portal
            }
            _ => Wall,
        }).collect()
    }));

    // Connect same-name portals in a new mapping.
    let mut portals = HashMap::new();
    for (_, xys) in portal_pos {
        portals.insert(xys[0].0, xys[1]);
        portals.insert(xys[1].0, xys[0]);
    }

    prune_maze(&mut maze);

    advtools::verify("Steps to walk to exit", walk(&maze, &portals, entry, false), 632);
    advtools::verify("Steps to walk with depth", walk(&maze, &portals, entry, true), 7162);
}

/// Prune any cul-de-sacs from the maze.
fn prune_maze(maze: &mut Grid<Cell>) {
    loop {
        let mut changed = 0;
        for pos in maze.positions() {
            if maze[pos] == Free {
                let mut wall_count = 0;
                let mut free_count = 0;
                maze.for_neighbors(pos, |p| match *p {
                    Wall => wall_count += 1,
                    Free => free_count += 1,
                    _ => ()
                });
                if wall_count == 3 && free_count == 1 {
                    maze[pos] = Wall;
                    changed += 1;
                }
            }
        }
        if changed == 0 {
            break;
        }
    }
}

fn walk(maze: &Grid<Cell>, portals: &HashMap<Pos, (Pos, bool)>, start: Pos, recursive: bool) -> u32 {
    let mut known = HashSet::with_capacity(1<<17);
    let mut queue = vec![(start, 0)];
    known.insert(queue[0]);

    for steps in 1.. {
        for (pos, depth) in replace(&mut queue, Vec::with_capacity(64)) {
            for new_pos in pos.neighbors() {
                match maze[new_pos] {
                    Free => (),
                    Wall | Entry => continue,
                    Portal => {
                        let (portal_pos, outer) = portals[&new_pos];

                        // Find the corridor next to the exit in this step, since
                        // teleporting from corridor to corridor only counts as 1 step.
                        let actual_pos = portal_pos.neighbors().find(|&xy| maze[xy] == Free).unwrap();

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
                            if known.insert((actual_pos, new_depth)) {
                                queue.push((actual_pos, new_depth));
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
                if known.insert((new_pos, depth)) {
                    queue.push((new_pos, depth));
                }
            }
        }
    }
    unreachable!()
}
