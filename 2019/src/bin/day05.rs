use advtools::input;
use advent19::Machine;

const AIR_COND: i64 = 1;
const RADIATOR: i64 = 5;

fn main() {
    let code = Machine::parse(input::string());

    let out = Machine::new(&code).with_input(AIR_COND).last().unwrap();
    advtools::verify("Output for air conditioner", out, 15314507);

    let out = Machine::new(&code).next_with(RADIATOR).unwrap();
    advtools::verify("Output for radiator controller", out, 652726);
}
