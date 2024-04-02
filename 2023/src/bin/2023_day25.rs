use advtools::input;
use advtools::grid::{Grid, Pos, Dir};
use advtools::prelude::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Forest,
    Path,
    Slope(Dir),
}

fn dfs(grid: &Grid<Tile>, pos: Pos, target: Pos, steps: usize,
       longest: &mut usize, seen: &mut HashSet<Pos>, consider_slopes: bool) {
    seen.insert(pos);
    for nbpos in grid.neighbors(pos) {
        if seen.contains(&nbpos) {
            continue;
        }
        match grid[nbpos] {
            Tile::Forest => continue,
            Tile::Path => {
                if nbpos == target {
                    *longest = (*longest).max(steps + 1);
                    println!("{}", longest);
                } else {
                    dfs(grid, nbpos, target, steps + 1, longest, seen, consider_slopes);
                }
            }
            Tile::Slope(slopedir) => {
                if !consider_slopes || slopedir == nbpos.dir_from(pos) {
                    dfs(grid, nbpos, target, steps + 1, longest, seen, consider_slopes);
                }
            }
        }
    }
    seen.remove(&pos);
}

fn main() {
//     input::set("\
// #.#####################
// #.......#########...###
// #######.#########.#.###
// ###.....#.>.>.###.#.###
// ###v#####.#v#.###.#.###
// ###.>...#.#.#.....#...#
// ###v###.#.#.#########.#
// ###...#.#.#.......#...#
// #####.#.#.#######.#.###
// #.....#.#.#.......#...#
// #.#####.#.#.#########v#
// #.#...#...#...###...>.#
// #.#.#v#######v###.###v#
// #...#.>.#...>.>.#.###.#
// #####v#.#.###v#.#.###.#
// #.....#...#...#.#.#...#
// #.#########.###.#.#.###
// #...###...#...#...#.###
// ###.###.#.###v#####v###
// #...#...#.#.>.>.#.>.###
// #.###.###.#.###.#.#v###
// #.....###...###...#...#
// #####################.#
// ");
    let grid = Grid::new(input::lines().map(|line| line.chars().map(|ch| match ch {
        '#' => Tile::Forest,
        '.' => Tile::Path,
        sl  => Tile::Slope(Dir::from_char(sl)),
    })));
    let start = Pos(1, 0);
    let target = Pos(grid.width() as i32 - 2, grid.height() as i32 - 1);

    let mut longest = 0;
    dfs(&grid, start, target, 0, &mut longest, &mut HashSet::new(), true);
    advtools::verify("Longest hike", longest, 2402);

    dfs(&grid, start, target, 0, &mut longest, &mut HashSet::new(), false);
    advtools::verify("Longest hike without slopes", longest, 6450);
}
