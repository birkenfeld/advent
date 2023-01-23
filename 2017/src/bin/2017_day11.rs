use advtools::input;

/// Find distance of a point from the origin in hex steps, max(|x|, |y|, |x+y|)
/// |x+y| is the maximum if the two coordinates have the same sign: (1,1) is
/// two steps.  In contrast, (1,-1) is just a single step.
fn dist(x: i32, y: i32) -> i32 {
    x.abs().max(y.abs()).max((x + y).abs())
}

fn main() {
    // We keep track of coordinates using a non-orthogonal system of basis
    // vectors.  e_y points to the north hex, e_x to the north-east hex.
    let (mut x, mut y) = (0, 0);
    let mut furthest = 0;
    for dir in input::string().split(',') {
        match dir {
            "n"  => { y += 1 }
            "ne" => { x += 1 }
            "se" => { x += 1; y -= 1 }
            "s"  => { y -= 1 }
            "sw" => { x -= 1 }
            "nw" => { x -= 1; y += 1 }
            _    => unreachable!()
        }
        furthest = furthest.max(dist(x, y));
    }
    // Part 1: distance after all steps.
    advtools::verify("Distance", dist(x, y), 722);
    // Part 2: furthest distance during any step.
    advtools::verify("Furthest", furthest, 1551);
}
