use advtools::prelude::HashSet;
use advtools::input;
use advtools::grid::Pos;
use advent_2019::Machine;

const EMPTY: i64 = 1;
const GOAL: i64 = 2;

/// Visit the maze in BFS order, so we have the shortest way as soon as we
/// find the goal.  Also returns when no more places to explore.
fn visit(start: (Pos, Machine)) -> (i32, Option<(Pos, Machine)>) {
    let mut known = HashSet::new();
    known.insert(start.0);
    let mut queue = vec![start];
    for steps in 1.. {
        for (pos, machine) in std::mem::take(&mut queue) {
            // Go through every (potential) new location and find out if
            // we can go there.
            for (ndir, new_pos) in pos.neighbors().enumerate() {
                if known.contains(&new_pos) {
                    continue;
                }
                // Get a new machine for each direction, so they can continue
                // independently.
                let mut machn = machine.clone();
                match machn.next_with(ndir as i64 + 1).unwrap() {
                    GOAL => return (steps, Some((new_pos, machn))),
                    EMPTY => queue.push((new_pos, machn)),
                    _ => ()
                }
                known.insert(new_pos);
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
    let code = Machine::parse(input::string());
    let machine = Machine::new(&code);

    // Part 1: find the number of steps to goal.
    let (steps, goal_state) = visit((Pos(0, 0), machine));
    advtools::verify("Steps to reach oxygen system", steps, 218);

    // Part 2: find the number of steps to reach every place in the maze.
    // Won't find the goal since we're starting there.
    let (steps, _) = visit(goal_state.unwrap());
    advtools::verify("Steps to fill space with oxygen", steps, 544);
}
