use advtools::input::iter_input;
use advent16::Machine;

fn main() {
    let mut machine = Machine::new(iter_input());
    machine.set_reg(0, 7);
    machine.run();
    advtools::verify("a (7 eggs)", machine.get_reg(0), 10953);

    let mut machine = Machine::new(iter_input());
    machine.set_reg(0, 12);
    machine.run();
    advtools::verify("a (12 eggs)", machine.get_reg(0), 479007513);
}
