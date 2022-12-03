use advtools::input;

fn main() {
    let scores = input::lines().fold((0, 0), |scores, pairing| {
        let add = match pairing {
            "A X" => (1 + 3, 3 + 0),
            "A Y" => (2 + 6, 1 + 3),
            "A Z" => (3 + 0, 2 + 6),
            "B X" => (1 + 0, 1 + 0),
            "B Y" => (2 + 3, 2 + 3),
            "B Z" => (3 + 6, 3 + 6),
            "C X" => (1 + 6, 2 + 0),
            "C Y" => (2 + 0, 3 + 3),
            "C Z" => (3 + 3, 1 + 6),
            _ => unreachable!("invalid pairing")
        };
        (scores.0 + add.0, scores.1 + add.1)
    });
    advtools::verify("Part 1 score", scores.0, 14163);
    advtools::verify("Part 2 score", scores.1, 12091);
}
