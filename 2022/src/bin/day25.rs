use advtools::input;

fn main() {
    // Parse and sum the balanced-quinary numbers.
    let mut sum = input::lines().map(|num| num.chars().fold(0, |n, c| 5*n + match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => unreachable!()
    })).sum::<i64>();

    // Convert the sum back to balanced quinary.
    let mut result = String::new();
    while sum != 0 {
        result.push(match sum % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => { sum += 5; '=' },
            4 => { sum += 5; '-' },
            _ => unreachable!()
        });
        sum /= 5;
    }
    result = result.chars().rev().collect();
    advtools::verify("Sum in balanced quinary", result, "2-2=12=1-=-1=000=222");
}
