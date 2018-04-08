extern crate advtools;
use advtools::prelude::{HashSet, Itertools};
use advtools::input::iter_input;

type Pos = (i32, i32, u8);
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

#[derive(PartialEq)]
enum Loc {
    Wall,
    Free,
    POI(u8),
}

fn find_steps(initial: Pos, final_: Option<Pos>, maze: &[Vec<Loc>]) -> usize {
    let mut seen = HashSet::default();
    let mut positions = vec![initial];
    let mut generation = 0;

    loop {
        generation += 1;
        let mut new_positions = vec![];
        for (x, y, pois) in positions {
            for &(dx, dy) in &DIRECTIONS {
                let new_pos = match maze[(y + dy) as usize][(x + dx) as usize] {
                    Loc::Wall => continue,
                    Loc::Free => (x + dx, y + dy, pois),
                    Loc::POI(n) => (x + dx, y + dy, pois | (1 << n))
                };
                if match final_ {
                    Some(p) => new_pos == p,
                    None => new_pos.2 == 255,
                } {
                    return generation;
                }
                if seen.insert(new_pos) {
                    new_positions.push(new_pos);
                }
            }
        }
        positions = new_positions;
    }
}

fn main() {
    let mut maze = Vec::new();
    for line in iter_input::<String>() {
        maze.push(line.chars().map(|ch| match ch {
            '#' => Loc::Wall,
            '.' => Loc::Free,
            '0' ..= '7' => Loc::POI((ch as u8) - b'0'),
            _ => panic!("invalid input: {}", ch)
        }).collect_vec());
    }
    let mut initial = None;
    'outer:
    for (y, row) in maze.iter().enumerate() {
        for (x, loc) in row.iter().enumerate() {
            if loc == &Loc::POI(0) {
                initial = Some((x as i32, y as i32, 1));
                break 'outer;
            }
        }
    }
    let initial = initial.unwrap();

    let steps = find_steps(initial, None, &maze);
    println!("Steps to reach all POIs: {}", steps);
    let steps = find_steps(initial, Some((initial.0, initial.1, 255)), &maze);
    println!("Including return: {}", steps);
}
