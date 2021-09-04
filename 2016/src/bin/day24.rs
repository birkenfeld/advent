use advtools::prelude::HashSet;
use advtools::input::iter_lines;
use advtools::grid::{Grid, Pos};

type State = (Pos<usize>, u8);

#[derive(PartialEq)]
enum Loc {
    Wall,
    Free,
    POI(u8),
}

fn find_steps(maze: &Grid<Loc>, initial: State, final_: Option<State>) -> usize {
    let mut seen = HashSet::with_capacity(1000);
    let mut positions = vec![initial];
    let mut generation = 0;

    loop {
        generation += 1;
        let mut new_positions = vec![];
        for (pos, pois) in positions {
            for new_pos in pos.neighbors() {
                let new_pos = match maze[new_pos] {
                    Loc::Wall => continue,
                    Loc::Free => (new_pos, pois),
                    Loc::POI(n) => (new_pos, pois | (1 << n))
                };
                if match final_ {
                    Some(p) => new_pos == p,
                    None => new_pos.1 == 255,
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
    let maze = Grid::new(iter_lines().map(|line| {
        line.chars().map(|ch| match ch {
            '#' => Loc::Wall,
            '.' => Loc::Free,
            '0' ..= '7' => Loc::POI((ch as u8) - b'0'),
            _ => panic!("invalid input: {}", ch)
        }).collect()
    }));

    let initial = maze.find_pos(|p| p == &Loc::POI(0)).unwrap();

    let steps = find_steps(&maze, (initial, 1), None);
    advtools::verify("Steps to reach all POIs", steps, 456);

    let steps = find_steps(&maze, (initial, 1), Some((initial, 255)));
    advtools::verify("Including return", steps, 704);
}
