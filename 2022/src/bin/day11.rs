use advtools::input;
use advtools::prelude::Itertools;

#[derive(Clone)]
enum Op {
    Add(i64),
    Mul(i64),
    Square,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    div: i64,
    next: [usize; 2],
    actions: i64,
}

fn run(monkeys: &mut [Monkey], n: usize, transform: impl Fn(i64) -> i64) -> i64 {
    for _ in 0..n {
        for i in 0..monkeys.len() {
            for cur_worry in std::mem::take(&mut monkeys[i].items) {
                monkeys[i].actions += 1;
                // Apply new worry level and caller's transform function.
                let new_worry = match monkeys[i].op {
                    Op::Add(n) => cur_worry + n,
                    Op::Mul(n) => cur_worry * n,
                    Op::Square => cur_worry * cur_worry,
                };
                let new_worry = transform(new_worry);
                // Decide where to throw.
                let index = monkeys[i].next[(new_worry % monkeys[i].div == 0) as usize];
                monkeys[index].items.push(new_worry);
            }
        }
    }

    monkeys.iter().map(|m| m.actions).sorted_by_key(|x| -x)
                                     .next_tuple()
                                     .map(|(x, y)| x * y).unwrap()
}

fn main() {
    let mut monkeys = vec![];
    for block in input::string().split("\n\n") {
        let mut lines = block.lines().skip(1);
        let mut after = |by: &str| lines.next().unwrap().split(by).nth(1).unwrap();
        let items = after(": ").split(", ").map(|x| x.parse().unwrap())
                                           .collect_vec();
        let op = match after(" = old ").split(" ").next_tuple().unwrap() {
            ("*", "old") => Op::Square,
            ("+", n) => Op::Add(n.parse().unwrap()),
            ("*", n) => Op::Mul(n.parse().unwrap()),
            _ => unreachable!()
        };
        let div = after(" by ").parse().unwrap();
        let next_1 = after(" monkey ").parse().unwrap();
        let next_0 = after(" monkey ").parse().unwrap();
        monkeys.push(Monkey { items, op, div, next: [next_0, next_1], actions: 0 })
    }

    // Part 1: the worry level is just divided by 3 each round.
    let part1 = run(&mut monkeys.clone(), 20, |n| n / 3);
    advtools::verify("Monkey business after 20", part1, 112221);

    // Part 2: the product of all test-divisors is safe to mod-reduce the worry
    // level by on each round.
    let divprod = monkeys.iter().map(|m| m.div).product::<i64>();
    let part2 = run(&mut monkeys, 10_000, |n| n % divprod);
    advtools::verify("Monkey business after 10000", part2, 25272176808_i64);
}
