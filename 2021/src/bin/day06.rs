use advtools::input::{input_string, to_i32};

fn main() {
    let mut ages = [0u64; 9];
    for age in input_string().trim().split(',').map(to_i32) {
        ages[age as usize] += 1;
    }

    let mut count_80 = 0u64;
    for gen in 0..256 {
        ages = [
            ages[1],
            ages[2],
            ages[3],
            ages[4],
            ages[5],
            ages[6],
            ages[7] + ages[0],
            ages[8],
            ages[0],
        ];

        if gen == 79 {
            count_80 = ages.iter().sum();
        }
    }
    let count_256: u64 = ages.iter().sum();

    advtools::verify("After 80 days", count_80, 350605);
    advtools::verify("After 256 days", count_256, 1592778185024u64);
}
