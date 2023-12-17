use advtools::input;
use advtools::grid::{Grid, Dir, Pos};
use advtools::rayon::prelude::*;
use advtools::prelude::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    SplitH,
    SplitV,
    MirrorLB,
    MirrorLT,
}

// Apply beam movement or splitting action to a beam at *pos* going *dir*.
fn apply_tile(beams: &mut Vec<(Pos<usize>, Dir)>, tile: Tile, pos: Pos<usize>, dir: Dir) {
    match (tile, dir) {
        (Tile::SplitH, Dir::U | Dir::D) => {
            beams.push((pos, Dir::L));
            beams.push((pos, Dir::R));
        }
        (Tile::SplitV, Dir::L | Dir::R) => {
            beams.push((pos, Dir::U));
            beams.push((pos, Dir::D));
        }
        (Tile::MirrorLT, Dir::L | Dir::R) => beams.push((pos, dir.left())),
        (Tile::MirrorLT, Dir::U | Dir::D) => beams.push((pos, dir.right())),
        (Tile::MirrorLB, Dir::L | Dir::R) => beams.push((pos, dir.right())),
        (Tile::MirrorLB, Dir::U | Dir::D) => beams.push((pos, dir.left())),
        (_, _) => beams.push((pos, dir))
    }
}

// Walk the beam(s) and count the number of lit-up tiles.
fn walk(grid: &Grid<Tile>, pos: Pos<usize>, dir: Dir) -> usize {
    let mut beams = vec![];
    apply_tile(&mut beams, grid[pos], pos, dir);

    let mut visited = HashSet::new();
    while !beams.is_empty() {
        for (pos, dir) in std::mem::take(&mut beams) {
            if visited.insert((pos, dir)) {
                if let Some(new_pos) = grid.neighbor(pos, dir) {
                    apply_tile(&mut beams, grid[new_pos], new_pos, dir);
                }
            }
        }
    }
    let lit_up = visited.iter().map(|(pos, _)| pos).collect::<HashSet<_>>();
    lit_up.len()
}

fn main() {
    let grid = Grid::new(input::lines().map(|line| line.chars().map(|ch| match ch {
        '.' => Tile::Empty,
        '-' => Tile::SplitH,
        '|' => Tile::SplitV,
        '/' => Tile::MirrorLT,
        '\\' => Tile::MirrorLB,
        _ => unreachable!()
    })));

    // Part 1: just one walk from the top left corner.
    advtools::verify("Energized tiles", walk(&grid, Pos(0, 0), Dir::R), 6906);

    // Part 2: walk from every possible edge.
    let x_max = (0..grid.width()).into_par_iter().flat_map(|x| {
        [walk(&grid, Pos(x, 0), Dir::D),
         walk(&grid, Pos(x, grid.height()-1), Dir::U)]
    }).max().unwrap();
    let y_max = (0..grid.height()).into_par_iter().flat_map(|y| {
        [walk(&grid, Pos(0, y), Dir::R),
         walk(&grid, Pos(grid.width()-1, y), Dir::L)]
    }).max().unwrap();
    advtools::verify("Maximum energization", x_max.max(y_max), 7330);
}
