use advtools::input::input_string;
use advent19::Machine;

const TESTMODE: i64 = 1;
const BOOST: i64 = 2;

fn main() {
    let code = Machine::parse(&input_string());

    let out = Machine::new(&code).run(Some(TESTMODE)).unwrap();
    advtools::print("Boost keycode", out);

    let out = Machine::new(&code).run(Some(BOOST)).unwrap();
    advtools::print("Distress coordinates", out);
}
