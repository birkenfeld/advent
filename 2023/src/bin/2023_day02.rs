use advtools::input;
use advtools::itertools::Itertools;

const COLORS: [&str; 3] = ["red", "green", "blue"];
const PRESENT: [u32; 3] = [12, 13, 14];

fn main() {
    let mut id_sum = 0;
    let mut power_sum = 0;
    for (i, line) in input::lines().flat_map(|l| l.split(": ").nth(1)).enumerate() {
        let mut possible = true;
        let mut min = [0, 0, 0];
        for round in line.split("; ").flat_map(|r| r.split(", ")) {
            let (num, color) = round.split(' ').collect_tuple().unwrap();
            let num = num.parse().unwrap();
            let color = COLORS.iter().position(|&p| p == color).unwrap();
            // Part 1: Game is possible if all cube numbers are below given.
            possible &= num <= PRESENT[color];
            // Part 2: Determine minimum cube numbers for all colors.
            min[color] = min[color].max(num);
        }
        id_sum += (i + 1) * possible as usize;
        power_sum += min[0] * min[1] * min[2];
    }

    advtools::verify("Possible game sum", id_sum, 2563);
    advtools::verify("Power sum", power_sum, 70768);
}
