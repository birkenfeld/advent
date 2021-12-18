use advtools::itertools::{iproduct, iterate};
use advtools::input;

const FORMAT: &str = r"target area: x=(\d+)\.\.(\d+), y=([-\d]+)\.\.([-\d]+)";

fn main() {
    let (x1, x2, y1, y2) = input::rx_parse::<(i32, i32, i32, i32)>(FORMAT);

    // Theoretically possible vx values are 1 to x2 (otherwise the first step overshoots).
    // For vy, the same holds in the negative direction for y1.  In the positive direction,
    // y1 is also the limit since the probe always comes back to y=0 before going negative.
    let (count, max_y_value) = iproduct!(1 ..= x2, y1 ..= -y1).filter(|&(vx, vy)| {
        // Iterate the x/y positions according to the kinematic rules.
        iterate((0, 0, vx, vy),
                |(x, y, vx, vy)| (x + vx, y + vy, (vx - 1).max(0), vy - 1))
            // Below y1 we can stop checking.
            .take_while(|(_, y, _, _)| y >= &y1)
            // Check if we fall into the target area.
            .any(|(x, y, _, _)| x >= x1 && x <= x2 && y >= y1 && y <= y2)
    }).fold((0, 0), |(count, max_y), (_, vy)| {
        // For found (vx, vy) pairs, record a tally and the maximum y height.
        (count + 1, max_y.max(vy * (vy + 1) / 2))
    });

    advtools::verify("Max y value", max_y_value, 15400);
    advtools::verify("Possible pairs", count, 5844);
}
