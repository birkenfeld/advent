use advtools::input::iter_input;

fn main() {
    let (length, aim, depth) = iter_input::<(String, i32)>()
        .fold((0, 0, 0), |(length, aim, depth), (action, n)| {
            match &*action {
                "down"    => (length, aim + n, depth),
                "up"      => (length, aim - n, depth),
                "forward" => (length + n, aim, depth + aim*n),
                _         => unreachable!()
            }
        });

    advtools::verify("Position 1", aim * length, 1459206);
    advtools::verify("Position 2", depth * length, 1320534480);
}
