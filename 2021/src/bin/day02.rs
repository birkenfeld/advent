use advtools::input::iter_input;

fn main() {
    let (horiz, aim, depth) = iter_input::<(String, i32)>()
        .fold((0, 0, 0), |(horiz, aim, depth), (action, n)| {
            match &*action {
                "down"    => (horiz, aim + n, depth),
                "up"      => (horiz, aim - n, depth),
                "forward" => (horiz + n, aim, depth + aim*n),
                _         => unreachable!()
            }
        });

    advtools::verify("Position 1", aim*horiz, 1459206);
    advtools::verify("Position 2", depth*horiz, 1320534480);
}
