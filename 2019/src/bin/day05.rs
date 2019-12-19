use advtools::input::input_string;
use advent19::I32Machine;

const AIR_COND: i32 = 1;
const RADIATOR: i32 = 5;

fn main() {
    let code = I32Machine::parse(&input_string());

    let out = I32Machine::new(&code).with_input(Some(AIR_COND)).last().unwrap();
    advtools::print("Output for air conditioner", out);

    let out = I32Machine::new(&code).run(Some(RADIATOR)).unwrap();
    advtools::print("Output for radiator controller", out);
}
