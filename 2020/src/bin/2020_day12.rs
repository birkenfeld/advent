use advtools::grid::{Pos, Dir};
use advtools::input;

fn main() {
    let mut pos = Pos(0i32, 0);
    let mut dir = Dir::R;

    for line in input::rx_lines(r"(.)(\d+)") {
        match line {
            ("F", n) => for _ in 0..n { pos = pos.to(dir); }
            ("N", n) => for _ in 0..n { pos = pos.up(); }
            ("S", n) => for _ in 0..n { pos = pos.down(); }
            ("E", n) => for _ in 0..n { pos = pos.right(); }
            ("W", n) => for _ in 0..n { pos = pos.left(); }
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
            ("N", n) => for _ in 0..n { wpos = wpos.up(); }
            ("S", n) => for _ in 0..n { wpos = wpos.down(); }
            ("E", n) => for _ in 0..n { wpos = wpos.right(); }
            ("W", n) => for _ in 0..n { wpos = wpos.left(); }
            ("L", 90) | ("R", 270) => wpos = Pos(wpos.y, -wpos.x),
            ("R", 90) | ("L", 270) => wpos = Pos(-wpos.y, wpos.x),
            ("R", 180) | ("L", 180) => wpos = Pos(-wpos.x, -wpos.y),
            _ => unreachable!()
        }
    }

    advtools::verify("With waypoint", pos.manhattan(), 106860);
}
