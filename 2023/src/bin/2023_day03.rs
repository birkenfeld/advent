use advtools::input;
use advtools::prelude::{HashMap, HashSet};
use advtools::itertools::chain;
use advtools::grid::Grid;

fn main() {
    let grid = Grid::new(input::lines().map(|l| l.chars().chain(['.'])));
    // Record adjacent number starting points for every gear (*) in the input.
    let mut gears = HashMap::<_, HashSet<_>>::new();
    // Record the parsed number for every number starting point.
    let mut part_nos = HashMap::new();
    let mut pos_iter = grid.positions::<u32>();

    // Go through the grid, finding number starting points.
    while let Some(start_pos) = pos_iter.next() {
        if grid[start_pos].is_digit(10) {
            let mut digits = String::new();
            let mut is_part_no = false;
            // Iterate and consume the rest of the digits.
            let rest = pos_iter.by_ref().take_while(|&p| grid[p].is_digit(10));
            for pos in chain([start_pos], rest) {
                digits.push(grid[pos]);
                // Check all neighbors for symbols and gears.
                for n in grid.neighbors_diag(pos) {
                    if !grid[n].is_digit(10) && grid[n] != '.' {
                        // If there's a symbol, it's a part number.
                        is_part_no = true;
                    }
                    if grid[n] == '*' {
                        // If there's a gear, record it.
                        gears.entry(n).or_default().insert(start_pos);
                    }
                }
            }
            // If it's a part number, record its value.
            if is_part_no {
                part_nos.insert(start_pos, digits.parse::<u32>().unwrap());
            }
        }
    }

    // Part 1: just the sum of all part numbers.
    let part_no_sum = part_nos.values().sum::<u32>();

    // Part 2: the sum of the part numbers adjacent to all gears with 2 number.
    let gear_ratio_sum = gears.values().filter(|ps| ps.len() == 2)
        .map(|ps| ps.iter().map(|p| part_nos[p]).product::<u32>())
        .sum::<u32>();

    advtools::verify("Sum of part numbers", part_no_sum, 527364);
    advtools::verify("Sum of gear ratios", gear_ratio_sum, 79026871);
}
