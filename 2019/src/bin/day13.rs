use advtools::prelude::Itertools;
use advtools::input::input_string;
use advent19::I32Machine;

const BLOCK: i32 = 2;
const PADDLE: i32 = 3;
const BALL: i32 = 4;

fn main() {
    let code = I32Machine::parse(&input_string());

    // Part 1: Count all the blocks drawn.
    let blocks = I32Machine::new(&code).tuples().filter(|(_, _, tile)| tile == &BLOCK);
    advtools::print("Blocks on screen", blocks.count());

    let mut ball_x;
    let mut paddle_x = 0;
    let mut score = 0;
    let mut machine = I32Machine::new(&code);
    // Set amount of quarters to 2.
    machine.set_mem(0, 2);
    while let Some((x, y, out)) = machine.next_tuple() {
        // Keep track of the score, the position of the paddle and the ball.
        if (x, y) == (-1, 0) {
            score = out;
        } else if out == PADDLE {
            paddle_x = x;
        } else if out == BALL {
            ball_x = x;
            // After the ball is drawn, supply input to move the paddle in the
            // direction of the ball.
            machine = machine.with_input(Some((ball_x - paddle_x).signum()));
        }
    }
    advtools::print("Final score", score);
}
