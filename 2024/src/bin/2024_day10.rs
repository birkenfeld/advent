use advtools::input;
use advtools::grid::Grid;
use advtools::prelude::HashSet;

fn main() {
    let grid = Grid::new(input::lines().map(
        |line| line.chars().map(|ch| ch as u8 - b'0')));

    let mut score = 0;   // part 1
    let mut rating = 0;  // part 2
    for trailhead in grid.positions::<i32>().filter(|&pos| grid[pos] == 0) {
        let mut found_ends = HashSet::new();
        let mut queue = vec![trailhead];

        while !queue.is_empty() {
            for pos in std::mem::take(&mut queue) {
                for nbpos in grid.neighbors(pos) {
                    if grid[nbpos] == grid[pos] + 1 {
                        if grid[nbpos] == 9 {
                            rating += 1;
                            if found_ends.insert(nbpos) {
                                score += 1;
                            }
                        } else {
                            queue.push(nbpos);
                        }
                    }
                }
            }
        }
    }

    advtools::verify("Total score", score, 737);
    advtools::verify("Total rating", rating, 1619);
}
