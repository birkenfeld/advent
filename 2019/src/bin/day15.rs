use advtools::prelude::HashSet;
use advtools::input::input_string;
use advent19::Machine;

const EMPTY: i64 = 1;
const GOAL: i64 = 2;

// TODO: use Dir

/// Visit the maze in BFS order, so we have the shortest way as soon as we
/// find the goal.  Also returns when no more places to explore.
fn visit(start: (i32, i32, Machine)) -> (i32, Option<(i32, i32, Machine)>) {
    let mut known = HashSet::new();
    known.insert((start.0, start.1));
    let mut queue = vec![start];
    for steps in 1.. {
        for (x, y, machine) in std::mem::replace(&mut queue, Vec::new()) {
            // Go through every (potential) new location and find out if
            // we can go there.
            let new_coords = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
            for (ndir, &(xn, yn)) in new_coords.iter().enumerate() {
                if known.contains(&(xn, yn)) {
                    continue;
                }
                // Get a new machine for each direction, so they can continue
                // independently.
                let mut machn = machine.clone();
                match machn.next_with(ndir as i64 + 1).unwrap() {
                    GOAL => return (steps, Some((xn, yn, machn))),
                    EMPTY => queue.push((xn, yn, machn)),
                    _ => ()
                }
                known.insert((xn, yn));
            }
        }
        if queue.is_empty() {
            // We already filled everything last time.
            return (steps - 1, None);
        }
    }
    unreachable!()
}

fn main() {
    let code = Machine::parse(&input_string());
    let machine = Machine::new(&code);

    // Part 1: find the number of steps to goal.
    let (steps, goal_state) = visit((0, 0, machine));
    advtools::print("Steps to reach oxygen system", steps);

    // Part 2: find the number of steps to reach every place in the maze.
    // Won't find the goal since we're starting there.
    let (steps, _) = visit(goal_state.unwrap());
    advtools::print("Steps to fill space with oxygen", steps);
}
