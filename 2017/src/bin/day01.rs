extern crate advtools;

fn main() {
    let input: Vec<_> = advtools::input_string().trim().chars().collect();
    let len = input.len();
    let captcha = |offset| (0..len).filter(|&ix| input[ix] == input[(ix + offset) % len])
                                   .map(|ix| input[ix].to_digit(10).unwrap())
                                   .sum::<u32>();
    println!("First round: {}", captcha(1));
    println!("Second round: {}", captcha(len/2));
}
