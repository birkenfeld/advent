use advtools::input;
use advtools::prelude::{HashSet, Itertools};

fn sum_dist(pos: &[u64]) -> (u64, u64) {
    // Collect the indices of "empty" coordinates.
    let max = pos.iter().max().unwrap();
    let empty = (0..*max).filter(|&i| !pos.iter().any(|&gi| gi == i)).collect::<HashSet<_>>();
    // For each pair of coordinates, count the number of "normal" and "expanded"
    // steps in between.
    pos.iter().tuple_combinations().fold((0, 0), |(normal, expanded), (&ga, &gb)| {
        let exp = (ga.min(gb)..ga.max(gb)).filter(|i| empty.contains(i)).count() as u64;
        (normal + ga.abs_diff(gb) - exp, expanded + exp)
    })
}

fn main() {
    // Get the galaxy positions.
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (y, line) in input::lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                xs.push(x as u64);
                ys.push(y as u64);
            }
        }
    }

    // Calculate the distances. X and Y are completely independent due to Manhattan.
    let (norm_x, exp_x) = sum_dist(&xs);
    let (norm_y, exp_y) = sum_dist(&ys);

    let part1 = norm_x + norm_y + 2*(exp_x + exp_y);
    advtools::verify("Sum of distances", part1, 10885634);

    let part2 = norm_x + norm_y + 1000000*(exp_x + exp_y);
    advtools::verify("Older universe distances", part2, 707505470642u64);
}
