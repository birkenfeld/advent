use advtools::prelude::Itertools;
use advtools::input;

type Values = [i32; 5];

const FORMAT: &str = ".* capacity ([-0-9]+), durability ([-0-9]+), \
                      flavor ([-0-9]+), texture ([-0-9]+), calories ([-0-9]+)";

fn add_up(amounts: &[i32], v: &[Values], select: impl Fn(&Values) -> i32) -> i32 {
    amounts.iter().enumerate().map(|(i, a)| a*select(&v[i])).sum()
}

fn fom(amounts: &[i32], v: &[Values]) -> i32 {
    (0..4).map(|i| add_up(amounts, v, |vi| vi[i]).max(0)).product()
}

fn gen_amounts(sum: usize, n: usize) -> Vec<Vec<i32>> {
    if n == 1 {
        vec![vec![sum as i32]]
    } else {
        let mut res = Vec::new();
        for a in 1..sum-n {
            for mut amnts in gen_amounts(sum - a, n - 1) {
                amnts.push(a as i32);
                res.push(amnts);
            }
        }
        res
    }
}

fn find_best(goalcal: Option<i32>, v: &[Values]) -> Vec<i32> {
    gen_amounts(100, v.len())
        .into_iter()
        .filter(|amounts| if let Some(goal) = goalcal {
            add_up(amounts, v, |vi| vi[4]) == goal
        } else {
            true
        })
        .max_by_key(|amounts| fom(amounts, v)).unwrap()
}

fn main() {
    let v = input::rx_lines(FORMAT).collect_vec();

    let best = find_best(None, &v);
    advtools::verify("Best", fom(&best, &v), 13882464);

    let new_best = find_best(Some(500), &v);
    advtools::verify("Best with 500 cal", fom(&new_best, &v), 11171160);
}
