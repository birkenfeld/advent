use advtools::input;
use advtools::prelude::Itertools;
use advtools::grid::{Grid, Dir, Pos};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Box2,
    Free,
}

fn main() {
//     input::set("##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########

// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^");
    let mut initpos = Pos(0, 0);
    let (grid_str, move_str) = input::string().split("\n\n").collect_tuple().unwrap();
    let mut grid = Grid::new(grid_str.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, ch)| match ch {
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '.' => Tile::Free,
            '@' => { initpos = Pos(x, y); Tile::Free }
            _ => panic!("impossible"),
        }).collect_vec()
    }));
    let moves = move_str.lines().flat_map(|l| l.chars()).map(Dir::from_char).collect_vec();

    let mut pos = initpos;
    for &dir in &moves {
        let nbpos = pos.to(dir);
        match grid[nbpos] {
            Tile::Free => pos = nbpos,
            Tile::Box => {
                // find the end of a column/row of boxes
                let mut endpos = nbpos;
                loop {
                    endpos = endpos.to(dir);
                    match grid[endpos] {
                        Tile::Wall => break,
                        Tile::Free => {
                            grid[endpos] = Tile::Box;
                            grid[nbpos] = Tile::Free;
                            pos = nbpos;
                            break;
                        }
                        _ => {}
                    }
                }
            }
            _ => {},
        }
    }

    let gps = grid.positions::<u32>().filter(|&pos| grid[pos] == Tile::Box).map(|pos| {
        pos.x + 100*pos.y
    }).sum::<u32>();
    advtools::verify("GPS sum", gps, 1490942);

    // Part 2: double-wide warehouse

    let mut pos = Pos(initpos.x * 2, initpos.y);
    let mut grid = Grid::new(grid_str.lines().map(|line| {
        line.chars().map(|ch| match ch {
            '#' => [Tile::Wall, Tile::Wall],
            'O' => [Tile::Box, Tile::Box2],
            '.' | '@' => [Tile::Free, Tile::Free],
            _ => panic!("impossible"),
        }).flatten()
    }));

    for &dir in &moves {
        let nbpos = pos.to(dir);
        if matches!(dir, Dir::R | Dir::L) {
            match grid[nbpos] {
                Tile::Free => pos = nbpos,
                Tile::Box | Tile::Box2 => {
                    // find the end of a row of boxes
                    let mut endpos = nbpos;
                    loop {
                        endpos = endpos.to(dir);
                        match grid[endpos] {
                            Tile::Wall => break,
                            Tile::Free => {
                                grid[endpos] = Tile::Box;
                                grid[nbpos] = Tile::Free;
                                pos = nbpos;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {},
            }
        } else {
            
        }
    }
}
