use advtools::prelude::HashMap;
use advtools::input::iter_input;

fn main() {
    let mut regs = HashMap::new();
    let mut any_largest = 0;
    for line in iter_input::<(String, String, i32, (), String, String, i32)>() {
        let (change_reg, sign, change_amt, _, check_reg, check_cond, check_val) = line;
        // Calculate amount of change.
        let change_amt = change_amt * if sign == "inc" { 1 } else { -1 };
        let check_reg_val = *regs.get(&check_reg).unwrap_or(&0);
        if match &*check_cond {
            "==" => check_reg_val == check_val,
            "!=" => check_reg_val != check_val,
            ">"  => check_reg_val >  check_val,
            ">=" => check_reg_val >= check_val,
            "<"  => check_reg_val <  check_val,
            "<=" => check_reg_val <= check_val,
            _ => panic!("invalid condition"),
        } {
            *regs.entry(change_reg).or_insert(0) += change_amt;
        }
        // Part 2: Largest register value after any instruction.
        any_largest = any_largest.max(*regs.values().max().unwrap());
    }
    // Part 1: Largest register value after all instructions.
    let final_largest = regs.values().max().unwrap();
    advtools::verify("Largest value", final_largest, 7787);
    advtools::verify("Largest value at any time", any_largest, 8997);
}
