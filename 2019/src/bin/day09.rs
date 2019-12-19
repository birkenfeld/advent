use advtools::input::input_string;
use advent19::I64Machine;

const TESTMODE: i64 = 1;
const BOOST: i64 = 2;

fn main() {
    let code = I64Machine::parse(&input_string());

    let out = I64Machine::new(&code).run(Some(TESTMODE)).unwrap();
    advtools::print("Boost keycode", out);

    let out = I64Machine::new(&code).run(Some(BOOST)).unwrap();
    advtools::print("Distress coordinates", out);
}
