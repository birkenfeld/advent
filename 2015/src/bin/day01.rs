use advtools::input::input_string;

fn main() {
    let res = input_string().chars().enumerate().fold(
        (0, None), |(level, basement), (i, ch)| {
            match ch {
                '(' => (level + 1, basement),
                ')' => (level - 1, if level == 0 { basement.or(Some(i + 1)) } else { basement }),
                _   => (level, basement),
            }
        });
    advtools::print("Resulting floor", res.0);
    advtools::print("Basement", res.1.unwrap());
}
