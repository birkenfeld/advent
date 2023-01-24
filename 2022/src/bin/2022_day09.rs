use advtools::input;
use advtools::prelude::HashSet;
use advtools::grid::{Dir, Pos};

fn run(n: usize) -> usize {
    // The positions of all the knots.
    let mut pos = vec![Pos(0i32, 0i32); n];
    // The positions visited by the last knot.
    let mut visited = HashSet::new();
    visited.insert(Pos(0, 0));

    for (dir, ndir) in input::rx_lines::<(char, i32)>(r"(.) (\d+)") {
        for _ in 0..ndir {
            // The first knot steps in the given direction.
            pos[0] = pos[0].to(Dir::from_char(dir));
            for i in 1..n {
                let deltax = pos[i-1].x - pos[i].x;
                let deltay = pos[i-1].y - pos[i].y;
                // If one coordinate differs from the previous by 2 or more,
                // step *both* coordinates towards it.
                if deltax.abs() > 1 || deltay.abs() > 1 {
                    pos[i].x += deltax.signum();
                    pos[i].y += deltay.signum();
                }
            }
            visited.insert(pos[n-1]);
        }
    }
    visited.len()
}

fn main() {
    advtools::verify("2 knots", run(2), 6067);
    advtools::verify("10 knots", run(10), 2471);
}
