use advtools::input::iter_input;

enum Op {
    Inc(usize),
    Half(usize),
    Triple(usize),
    Jump(isize),
    JumpIfEven(usize, isize),
    JumpIfOne(usize, isize),
}

fn run(instrs: &[Op], mut reg: [u32; 2]) -> [u32; 2] {
    let ip_max = instrs.len() as isize;
    let mut ip = 0;
    loop {
        match instrs[ip as usize] {
            Op::Inc(r) => reg[r] += 1,
            Op::Half(r) => reg[r] /= 2,
            Op::Triple(r) => reg[r] *= 3,
            Op::Jump(n) => ip += n,
            Op::JumpIfEven(r, n) => if reg[r] % 2 == 0 { ip += n },
            Op::JumpIfOne(r, n) => if reg[r] == 1 { ip += n },
        }
        ip += 1;
        if ip < 0 || ip >= ip_max {
            return reg;
        }
    }
}

fn read_reg(tok: &str) -> usize {
    match tok.trim_matches(',') {
        "a" => 0,
        "b" => 1,
        _   => panic!("unknown reg: {}", tok)
    }
}

fn read_jump(tok: &str) -> isize {
    tok.parse::<isize>().expect("illegal jump target") - 1
}

fn read_instr(toks: Vec<String>) -> Op {
    match &*toks[0] {
        "inc" => Op::Inc(read_reg(&toks[1])),
        "hlf" => Op::Half(read_reg(&toks[1])),
        "tpl" => Op::Triple(read_reg(&toks[1])),
        "jmp" => Op::Jump(read_jump(&toks[1])),
        "jie" => Op::JumpIfEven(read_reg(&toks[1]), read_jump(&toks[2])),
        "jio" => Op::JumpIfOne(read_reg(&toks[1]), read_jump(&toks[2])),
        opc   => panic!("unknown mnemonic: {}", opc),
    }
}

fn main() {
    let mut instrs = Vec::new();
    for toks in iter_input::<Vec<String>>() {
        instrs.push(read_instr(toks));
    }
    println!("Register b for a = 0: {}", run(&instrs, [0, 0])[1]);
    println!("Register b for a = 1: {}", run(&instrs, [1, 0])[1]);
}
