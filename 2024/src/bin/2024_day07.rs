use advtools::input;

fn concat(a: u64, b: u64) -> u64 {
    let mut p = 1;
    while p <= b {
        p *= 10;
    }
    a * p + b
}

fn test(result: u64, current: u64, operands: &[u64], with_concat: bool) -> bool {
    if operands.is_empty() {
        current == result
    } else if current <= result {
        test(result, current + operands[0], &operands[1..], with_concat) ||
            test(result, current * operands[0], &operands[1..], with_concat) ||
            (with_concat && test(result, concat(current, operands[0]), &operands[1..], with_concat))
    } else {
        false
    }
}

fn main() {
    let mut tot1 = 0;
    let mut tot2 = 0;
    for line in input::rx_lines::<(u64, input::Sep<u64, ' '>)>(r"(\d+): (.*)") {
        let (result, operands) = line;
        if test(result, operands.vec[0], &operands.vec[1..], false) { tot1 += result; }
        if test(result, operands.vec[0], &operands.vec[1..], true) { tot2 += result; }
    }
    advtools::verify("Calibration result", tot1, 4998764814652_u64);
    advtools::verify("With concatenation", tot2, 37598910447546_u64);
}
