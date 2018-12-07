const ROW: usize = 2981;
const COL: usize = 3075;
const FIRST: u64 = 20_151_125;
const FACTOR: u64 = 252_533;
const DIVISOR: u64 = 33_554_393;

fn main() {
    let mut col = 1;
    let mut row = 1;
    let mut code = FIRST;
    loop {
        if row == 1 {
            row = col + 1;
            col = 1;
        } else {
            row -= 1;
            col += 1;
        }
        code = (code * FACTOR) % DIVISOR;
        if row == ROW && col == COL {
            println!("Code at ({},{}): {}", ROW, COL, code);
            break;
        }
    }
}
