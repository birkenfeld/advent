extern crate advtools;

fn main() {
    let input: Vec<_> = advtools::input_string().trim().chars().collect();
    let captcha = |offset| input.iter().zip(input.iter().cycle().skip(offset))
                                       .filter(|&(a, b)| a == b)
                                       .map(|(a, _)| a.to_digit(10).unwrap())
                                       .sum::<u32>();
    println!("First round: {}", captcha(1));
    println!("Second round: {}", captcha(input.len() / 2));
}
