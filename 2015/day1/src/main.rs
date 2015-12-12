use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();
    let mut level = 0isize;
    let mut basement = false;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => level += 1,
            ')' => {
                level -= 1;
                if level < 0 && !basement {
                    println!("Basement: {}", i+1);
                    basement = true;
                }
            }
            _ => (),
        }
    }
    println!("Result: {}", level);
}
