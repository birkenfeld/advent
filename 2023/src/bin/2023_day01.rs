use advtools::input;

const DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six",
                           "seven", "eight", "nine"];

fn calibration(f: impl Fn(char, &str) -> Option<u32>) -> u32 {
    input::lines().map(|line| {
        // Filter input for digits, then take first and last.
        let mut digits = line.char_indices().filter_map(|(i, c)| f(c, &line[i..]));
        let first = digits.next().unwrap();
        // If there is only one digit, it will be first and last at the same time.
        let last = digits.next_back().unwrap_or(first);
        10*first + last
    }).sum()
}

fn main() {
    // Part 1: only digits are valid.
    advtools::verify("Calibration sum", calibration(|c, _| c.to_digit(10)), 55029);

    // Part 2: digits can also be in text form.
    advtools::verify("With text digits", calibration(|c, rest| c.to_digit(10).or_else(
        || DIGITS.iter().position(|d| rest.starts_with(d)).map(|i| i as u32 + 1)
    )), 55686);
}
