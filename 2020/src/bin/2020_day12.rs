use advtools::grid::{Pos, Dir};
use advtools::input;

fn main() {
    let mut pos = Pos(0i32, 0);
    let mut dir = Dir::R;

    for line in input::rx_lines(r"(.)(\d+)") {
        match line {
            ("F", n) => for _ in 0..n { pos += dir; }
            ("N", n) => for _ in 0..n { pos += Dir::U; }
            ("S", n) => for _ in 0..n { pos += Dir::D; }
            ("E", n) => for _ in 0..n { pos += Dir::R; }
            ("W", n) => for _ in 0..n { pos += Dir::L; }
            ("L", 90) | ("R", 270) => dir = dir.left(),
            ("R", 90) | ("L", 270) => dir = dir.right(),
            ("R", 180) | ("L", 180) => dir = dir.flip(),
            _ => unreachable!()
        }
    }

    advtools::verify("End position", pos.manhattan(), 1457);

    let mut wpos = Pos(10i32, -1);
    let mut pos = Pos(0i32, 0);

    for line in input::rx_lines(r"(.)(\d+)") {
        match line {
            ("F", n) => pos += wpos * n,
            ("N", n) => for _ in 0..n { wpos += Dir::U; }
            ("S", n) => for _ in 0..n { wpos += Dir::D; }
            ("E", n) => for _ in 0..n { wpos += Dir::R; }
            ("W", n) => for _ in 0..n { wpos += Dir::L; }
            ("L", 90) | ("R", 270) => wpos = Pos(wpos.y, -wpos.x),
            ("R", 90) | ("L", 270) => wpos = Pos(-wpos.y, wpos.x),
            ("R", 180) | ("L", 180) => wpos = Pos(-wpos.x, -wpos.y),
            _ => unreachable!()
        }
    }

    advtools::verify("With waypoint", pos.manhattan(), 106860);
}
