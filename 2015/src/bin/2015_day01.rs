use advtools::input;

fn main() {
    let res = input::chars().enumerate().fold(
        (0, None), |(level, basement), (i, ch)| {
            match ch {
                '(' => (level + 1, basement),
                ')' => (level - 1, if level == 0 { basement.or(Some(i + 1)) } else { basement }),
                _   => (level, basement),
            }
        });
    advtools::verify("Resulting floor", res.0, 232);
    advtools::verify("Basement", res.1.unwrap(), 1783);
}
