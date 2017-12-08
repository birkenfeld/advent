extern crate advtools;
use advtools::prelude::*;

fn main() {
    let mut regs = HashMap::new();
    let mut any_largest = 0;
    for line in iter_input::<Vec<String>>() {
        let change_reg = line[0].to_owned();
        let sign = if line[1] == "inc" { 1 } else { -1 };
        let change_amount = sign * to_i32(&line[2]);
        let check_reg_val = *regs.get(&line[4]).unwrap_or(&0);
        let check_amount = to_i32(&line[6]);
        if match &*line[5] {
            "==" => check_reg_val == check_amount,
            "!=" => check_reg_val != check_amount,
            ">"  => check_reg_val >  check_amount,
            ">=" => check_reg_val >= check_amount,
            "<"  => check_reg_val <  check_amount,
            "<=" => check_reg_val <= check_amount,
            _ => panic!("invalid condition"),
        } {
            *regs.entry(change_reg).or_insert(0) += change_amount;
        }
        any_largest = any_largest.max(*regs.values().max().unwrap());
    }
    let final_largest = regs.values().max().unwrap();
    println!("Largest value: {}", final_largest);
    println!("Largest value at any time: {}", any_largest);
}
