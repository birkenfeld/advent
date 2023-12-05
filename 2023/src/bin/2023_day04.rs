use advtools::input;
use advtools::prelude::{HashSet, HashMap};

fn main() {
    let mut points = 0;
    let mut instances = HashMap::<usize, usize>::new();

    for (i, (num1, num2)) in input::rx_lines::<(&str, &str)>(r".*: (.*) \| (.*)").enumerate() {
        // Extract the numbers and determine the number of matches.
        let winners = num1.split_whitespace().collect::<HashSet<_>>();
        let commons = num2.split_whitespace().filter(|n| winners.contains(n)).count();

        // Part 1: add up power-of-two points for winning numbers.
        if commons > 0 {
            points += 1 << (commons - 1);
        }

        // Part 2: keep track of card amounts.
        *instances.entry(i).or_default() += 1;
        for j in i+1 .. i+1+commons {
            *instances.entry(j).or_default() += instances[&i];
        }
    }

    advtools::verify("Points total", points, 20855);
    advtools::verify("Final cards", instances.values().sum::<usize>(), 5489600);
}
