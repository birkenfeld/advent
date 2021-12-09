use advtools::input::iter_input;

fn main() {
    // Go through the input, collecting both part 1 and part 2.
    // The "depth" of part 1 is the same as the "aim" of part 2.
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
