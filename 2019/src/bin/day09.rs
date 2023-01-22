use advtools::input;
use advent19::Machine;

const TESTMODE: i64 = 1;
const SENSORMODE: i64 = 2;

fn main() {
    let code = Machine::parse(input::string());

    let out = Machine::new(&code).next_with(TESTMODE).unwrap();
    advtools::verify("Boost keycode", out, 3546494377_i64);

    let out = Machine::new(&code).next_with(SENSORMODE).unwrap();
    advtools::verify("Distress coordinates", out, 47253);
}
