extern crate advtools;
extern crate serde_json;

use serde_json::{Value, from_reader};
use serde_json::Value::*;

fn sum_up(val: Value, no_reds: bool) -> i64 {
    match val {
        Number(v) => v.as_i64().unwrap(),
        String(_) | Bool(_) | Null => 0,
        Array(vs) => vs.into_iter().map(|v| sum_up(v, no_reds)).sum(),
        Object(map) => map.into_iter().fold((false, 0), |acc, kv| {
            let has_red = acc.0 || (no_reds && kv.1 == String("red".into()));
            (has_red, if has_red { 0 } else { acc.1 + sum_up(kv.1, no_reds) })
        }).1,
    }
}

fn main() {
    let doc: Value = from_reader(advtools::input_file()).unwrap();
    println!("Sum with reds: {:?}", sum_up(doc.clone(), false));
    println!("Sum without reds: {:?}", sum_up(doc, true));
}
