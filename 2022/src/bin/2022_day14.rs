use advtools::input;
use advtools::grid::{Grid, Pos};
use advtools::prelude::{Itertools, iproduct};

const START_X: usize = 500;

fn run(grid: &mut Grid<bool>) -> usize {
    let mut sand_placed = 0;
    'outer: loop {
        let mut x = START_X;
        // Try to find the X position for each Y, dropping down.
        for y in 0..grid.height()-1 {
            // Check the possible X positions in the next Y level.
            match (grid[Pos(x, y+1)], grid[Pos(x-1, y+1)], grid[Pos(x+1, y+1)]) {
                (false, _, _) => (),
                (_, false, _) => x -= 1,
                (_, _, false) => x += 1,
                (_, _, true)  => {
                    // Not found: enter the new filled position in the grid.
                    grid[Pos(x, y)] = true;
                    sand_placed += 1;
                    if y == 0 {
                        // Placed sand at (500, 0) - exit condition for part 2.
                        return sand_placed;
                    }
                    continue 'outer;
                }
            }
        }
        // Sand has not found resting place - exit condition for part 1.
        return sand_placed;
    }
}

fn main() {
    let mut grid = Grid::fill(false, 1000, 200);
    let mut max_y = 0;

    for line in input::lines() {
        for (from, to) in line.split(" -> ").tuple_windows() {
            let (fx, fy) = from.split(',').map(|p| p.parse::<i32>().unwrap())
                                          .collect_tuple().unwrap();
            let (tx, ty) = to.split(',').map(|p| p.parse::<i32>().unwrap())
                                        .collect_tuple().unwrap();
            for (x, y) in iproduct!(fx.min(tx)..=fx.max(tx), fy.min(ty)..=fy.max(ty)) {
                grid[Pos(x, y)] = true;
            }
            max_y = max_y.max(fy).max(ty);
        }
    }

    advtools::verify("Sand coming to rest", run(&mut grid.clone()), 719);

    // Add the floor for part 2.
    (0..1000).for_each(|x| grid[Pos(x, max_y + 2)] = true);
    advtools::verify("Total sand with floor", run(&mut grid), 23390);
}
