extern crate advtools;

fn dist(x: i32, y: i32) -> i32 {
    x.abs().max(y.abs()).max((x + y).abs())
}

fn main() {
    let (mut x, mut y) = (0, 0);
    let mut furthest = 0;
    for dir in advtools::input_string().trim().split(',') {
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
    println!("Distance: {}", dist(x, y));
    println!("Furthest: {}", furthest);
}
