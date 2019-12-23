use advtools::input::input_string;
use advent19::Machine;

const TESTMODE: i64 = 1;
const SENSORMODE: i64 = 2;

fn main() {
    let code = Machine::parse(&input_string());

    let out = Machine::new(&code).next_with(TESTMODE).unwrap();
    advtools::print("Boost keycode", out);

    let out = Machine::new(&code).next_with(SENSORMODE).unwrap();
    advtools::print("Distress coordinates", out);
}
