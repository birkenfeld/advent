use advtools::input::input_string;
use advent19::Machine;

const AIR_COND: i64 = 1;
const RADIATOR: i64 = 5;

fn main() {
    let code = Machine::parse(&input_string());

    let out = Machine::new(&code).with_input(AIR_COND).last().unwrap();
    advtools::print("Output for air conditioner", out);

    let out = Machine::new(&code).next_with(RADIATOR).unwrap();
    advtools::print("Output for radiator controller", out);
}
