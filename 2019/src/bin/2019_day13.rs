use advtools::prelude::Itertools;
use advtools::input;
use advent_2019::Machine;

const BLOCK: i64 = 2;
const PADDLE: i64 = 3;
const BALL: i64 = 4;

fn main() {
    let code = Machine::parse(input::string());

    // Part 1: Count all the blocks drawn.
    let blocks = Machine::new(&code).tuples().filter(|(_, _, tile)| tile == &BLOCK);
    advtools::verify("Blocks on screen", blocks.count(), 369);

    let mut paddle_x = 0;
    let mut score = 0;
    let mut machine = Machine::new(&code);
    // Set amount of quarters to 2.
    machine.set_mem(0, 2);
    while let Some((x, y, out)) = machine.next_tuple() {
        // Keep track of the score, the position of the paddle and the ball.
        if (x, y) == (-1, 0) {
            score = out;
        } else if out == PADDLE {
            paddle_x = x;
        } else if out == BALL {
            // After the ball is drawn, supply input to move the paddle in the
            // direction of the ball.
            machine = machine.with_input((x - paddle_x).signum());
        }
    }
    advtools::verify("Final score", score, 19210);
}
