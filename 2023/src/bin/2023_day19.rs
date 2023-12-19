use advtools::input;
use advtools::prelude::{HashMap, Itertools};

const REGEX: &str = r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}|(.+)\{(.+)\}";

#[derive(Clone, Copy)]
enum Action {
    Flow(&'static str, usize),
    Accept,
    Reject,
}

fn count_accepted(workflows: &HashMap<&str, Vec<(usize, u32, (Action, Action))>>, wf: &str, wfi: usize, range: [(u32, u32); 4]) -> u64 {
    let (index, split, actions) = workflows[wf][wfi];
    let sub_range = |range: [(u32, u32); 4], action| {
        if range[index].1 >= range[index].0 {
            match action {
                Action::Reject => 0,
                Action::Accept => range.iter().map(|(min, max)| (max - min + 1) as u64).product::<u64>(),
                Action::Flow(new_wf, start) => count_accepted(workflows, new_wf, start, range)
            }
        } else { 0 }
    };
    let (mut range1, mut range2) = (range, range);
    range1[index].1 = split.min(range1[index].1);
    range2[index].0 = (split+1).max(range2[index].0);
    sub_range(range1, actions.0) + sub_range(range2, actions.1)
}

fn main() {
    let action = |item| match item {
        "A" => Action::Accept,
        "R" => Action::Reject,
        wf  => Action::Flow(wf, 0),
    };

    let mut flows = HashMap::new();
    let mut parts = Vec::new();
    for (rating, name, flow) in input::rx_lines::<(Option<(u32, u32, u32, u32)>, &str, &str)>(REGEX) {
        if let Some(rating) = rating {
            parts.push(rating);
            continue;
        }
        let flow = flow.split(',').enumerate().map(|(i, item)| {
            if let Some((condstr, next)) = item.split(':').collect_tuple() {
                let (var, val) = condstr.split(['<', '>']).collect_tuple().unwrap();
                let index = ["x", "m", "a", "s"].iter().position(|&v| v == var).unwrap();
                let val = val.parse().unwrap();
                if condstr.contains('>') {
                    (index, val, (Action::Flow(name, i+1), action(next)))
                } else {
                    (index, val-1, (action(next), Action::Flow(name, i+1)))
                }
            } else {
                (0, 0, (action(item), action(item)))
            }
        }).collect_vec();
        flows.insert(name, flow);
    }

    // Part 1: check for acceptance of given items.
    let accepted = parts.into_iter().map(|(x, m, a, s)| {
        count_accepted(&flows, "in", 0, [(x, x), (m, m), (a, a), (s, s)]) as u32 * (x + m + a + s)
    }).sum::<u32>();
    advtools::verify("Rating sum of accepted", accepted, 376008);

    // Part 2: check the whole range.
    let accepted = count_accepted(&flows, "in", 0, [(1, 4000); 4]);
    advtools::verify("All accepted in range", accepted, 124078207789312_u64);
}

