use advtools::input;
use serde_json::{Value, Value::*, from_str};

fn sum_up(val: &Value, no_reds: bool) -> i64 {
    match val {
        Number(v) => v.as_i64().unwrap(),
        String(_) | Bool(_) | Null => 0,
        Array(vs) => vs.iter().map(|v| sum_up(v, no_reds)).sum(),
        Object(map) => map.iter().fold((false, 0), |acc, kv| {
            let has_red = acc.0 || (no_reds && kv.1 == "red");
            (has_red, if has_red { 0 } else { acc.1 + sum_up(kv.1, no_reds) })
        }).1,
    }
}

fn main() {
    let doc: Value = from_str(input::string()).expect("input not valid JSON");
    advtools::verify("Sum with reds", sum_up(&doc, false), 191164);
    advtools::verify("Sum without reds", sum_up(&doc, true), 87842);
}
