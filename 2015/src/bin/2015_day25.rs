use advtools::input;

const FIRST: u64 = 20_151_125;
const FACTOR: u64 = 252_533;
const DIVISOR: u64 = 33_554_393;

const RX: &str = r".* at row (\d+), column (\d+).";

fn main() {
    let (target_row, target_col) = input::rx_parse::<(u64, u64)>(RX);

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
        if row == target_row && col == target_col {
            advtools::verify("Code at target", code, 9132360);
            return;
        }
    }
}
