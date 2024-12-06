use advtools::input;
use advtools::grid::{Grid, Pos};

const UL: Pos = Pos(-1, -1);
const UU: Pos = Pos(-1, 0);
const UR: Pos = Pos(-1, 1);
const LL: Pos = Pos(0, -1);
const RR: Pos = Pos(0, 1);
const DL: Pos = Pos(1, -1);
const DD: Pos = Pos(1, 0);
const DR: Pos = Pos(1,  1);

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    let grid = Grid::new(input::lines().map(|line| line.chars()));
    for pos in grid.positions::<i32>() {
        if grid[pos] == 'X' {
            for dir in [UL, UU, UR, LL, RR, DL, DD, DR] {
                if grid.get(pos + dir) == Some(&'M') &&
                    grid.get(pos + dir*2) == Some(&'A') &&
                    grid.get(pos + dir*3) == Some(&'S')
                {
                    part1 += 1;
                }
            }
        }

        if grid[pos] == 'A' {
            match ((grid.get(pos + UL), grid.get(pos + DR)),
                   (grid.get(pos + UR), grid.get(pos + DL))) {
                ((Some(&'M'), Some(&'S')) | (Some(&'S'), Some(&'M')),
                 (Some(&'M'), Some(&'S')) | (Some(&'S'), Some(&'M'))) => {
                    part2 += 1;
                }
                _ => {}
            }
        }
    }
    advtools::verify("Number of XMAS", part1, 2462);
    advtools::verify("Number of X-MAS", part2, 1877);
}
