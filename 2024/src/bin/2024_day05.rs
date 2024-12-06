use advtools::input;

const RX: &str = r"(?:(\d+)\|(\d+))|(.*)";

fn fix_plan(conds: &[(u32, u32)], mut plan: Vec<u32>) -> (bool, u32) {
    let mut good = true;
    'lp: loop {
        for &(first, second) in conds {
            if let (Some(ix1), Some(ix2)) = (plan.iter().position(|&v| v == first),
                                             plan.iter().position(|&v| v == second)) {
                if ix1 > ix2 {
                    good = false;
                    plan.swap(ix1, ix2);
                    continue 'lp;
                }
            }
        }
        break;
    }
    (good, plan[plan.len() / 2])
}

fn main() {
    let mut conds = Vec::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input::rx_lines(RX) {
        let (cond, plan): (Option<(u32, u32)>, &str) = line;
        if let Some(cond) = cond {
            conds.push(cond);
        } else {
            let plan = plan.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
            let (good, middle) = fix_plan(&conds, plan);
            if good {
                part1 += middle;
            } else {
                part2 += middle;
            }
        }
    }
    advtools::verify("Good updates", part1, 4185);
    advtools::verify("Fixed updates", part2, 4480);
}
