use advtools::input;
use advtools::prelude::Regex;

const RX: &str = r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)";

fn main() {
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut enable = true;

    // find all matches in the input string and process enables for part 2
    for m in Regex::new(RX).unwrap().captures_iter(input::string()) {
        if &m[0] == "do()" {
            enable = true;
        } else if &m[0] == "don't()" {
            enable = false;
        } else {
            let n = m[1].parse::<u32>().unwrap() * m[2].parse::<u32>().unwrap();
            sum1 += n;
            if enable {
                sum2 += n;
            }
        }
    }
    advtools::verify("Sum of multiplications", sum1, 183788984);
    advtools::verify("Sum of multiplications with conditionals", sum2, 62098619);
}
