use advtools::prelude::Itertools;
use advtools::input;
use advtools::grid::Grid;

const MAX_STEPS: usize = 50;

fn main() {
    let mut iter = input::lines();
    let algo = iter.next().unwrap().chars().map(|ch| ch == '#').collect_vec();
    // If completely-dark areas are flipped on, they must be flipped back off
    // in the next step.
    if algo[0] {
        assert!(!algo[algo.len() - 1]);
    }

    // Read and enlarge grid by N on each side.
    let mut grid = Grid::new(iter.map(|line| line.chars().map(|ch| ch == '#').collect()));
    grid.enlarge(MAX_STEPS + 1, false);

    for step in 1..=MAX_STEPS {
        let mut new_grid = grid.clone();
        for pos in grid.positions::<usize>() {
            // The edge positions are either flipped or stay, depending on algo[0].
            if pos.x == 0 || pos.x == grid.width() - 1 ||
                pos.y == 0 || pos.y == grid.height() - 1 {
                    new_grid[pos] ^= algo[0];
                    continue;
                }
            // Otherwise, look up the new pixel.
            let index = [grid[pos.left().up()],
                         grid[pos.up()],
                         grid[pos.right().up()],
                         grid[pos.left()],
                         grid[pos],
                         grid[pos.right()],
                         grid[pos.left().down()],
                         grid[pos.down()],
                         grid[pos.right().down()]]
                .into_iter()
                .fold(0, |acc, b| (acc << 1) | b as usize);
            new_grid[pos] = algo[index];
        }
        grid = new_grid;

        if step == 2 {
            advtools::verify("Lit after 2 steps", grid.count(|x| *x), 5498);
        }
    }
    advtools::verify("Lit after 50 steps", grid.count(|x| *x), 16014);
}
