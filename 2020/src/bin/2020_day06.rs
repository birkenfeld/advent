use advtools::prelude::HashSet;
use advtools::input;

fn main() {
    let mut any_count = 0;
    let mut all_count = 0;

    for group in input::string().split("\n\n") {
        let any_yes = HashSet::from_iter(group.chars().filter(|&ch| ch != '\n'));
        any_count += any_yes.len();

        let all_yes = group.lines().map(|l| l.chars().collect::<HashSet<_>>())
            .fold(any_yes, |a, b| &a & &b);
        all_count += all_yes.len();
    }

    advtools::verify("Number of \"any said yes\"", any_count, 6947);
    advtools::verify("Number of \"all said yes\"", all_count, 3398);
}
