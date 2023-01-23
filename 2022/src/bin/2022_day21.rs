use advtools::input;
use advtools::prelude::HashMap;

const RX: &str = r"(....): (?:(\d+)|(....) ([-+*/]) (....))";

type Name = &'static str;

#[derive(Clone, Copy)]
enum Op { Add, Sub, Mul, Div }

#[derive(Clone, Copy)]
enum Var {
    Value(f64),
    Op(Op, Name, Name),
}

#[derive(Clone)]
struct Task {
    map: HashMap<Name, Var>,
    root1: Name,
    root2: Name,
}

fn eval(map: &HashMap<Name, Var>, which: Name) -> f64 {
    match map[&which] {
        Var::Value(v) => v,
        Var::Op(op, n1, n2) => {
            let v1 = eval(map, n1);
            let v2 = eval(map, n2);
            match op {
                Op::Add => v1 + v2,
                Op::Sub => v1 - v2,
                Op::Mul => v1 * v2,
                Op::Div => v1 / v2,
            }
        }
    }
}

fn difference(task: &mut Task, human: f64) -> f64 {
    task.map.insert("humn", Var::Value(human));
    eval(&task.map, task.root1) - eval(&task.map, task.root2)
}

fn main() {
    // Parse the input.
    let mut task = Task { map: HashMap::new(), root1: "", root2: "" };
    for (var, val, c1, op, c2) in input::rx_lines::<(&str, Option<f64>, &str, &str, &str)>(RX) {
        task.map.insert(var, match op {
            "+" => Var::Op(Op::Add, c1, c2),
            "-" => Var::Op(Op::Sub, c1, c2),
            "*" => Var::Op(Op::Mul, c1, c2),
            "/" => Var::Op(Op::Div, c1, c2),
            _ => Var::Value(val.unwrap()),
        });
        if var == "root" {
            (task.root1, task.root2) = (c1, c2);
        }
    }

    // Part 1: just evaluate the root monkey.
    advtools::verify("Root result", eval(&task.map, "root"), "66174565793494");

    // Part 2: we defined difference() to be the difference of the two monkeys to
    // compare.  Therefore we need to find the root of this function, which we
    // can do using Newton's method.
    let mut x = 0.;
    loop {
        // Evaluate the function at the best guess position.
        let y_x = difference(&mut task, x);
        if y_x == 0. {
            // We have the solution!
            advtools::verify("Human number", x, "3327575724809");
            return;
        }
        // Evaluate the function at a nearby value.
        let y_x2 = difference(&mut task, x + 0.1);
        // Calculate the new best guess from the slope at this position,
        // assuming locally linear behavior of the function.
        x -= y_x / ((y_x2 - y_x) / 0.1);
    }
}
