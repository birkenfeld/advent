use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut set = HashSet::new();
    let mut xs = 0;
    let mut ys = 0;
    let mut xr = 0;
    let mut yr = 0;
    let mut robo = false;
    set.insert((0, 0));
    let mut input = String::new();
    File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();
    for ch in input.chars() {
        match ch {
            '<' => if robo { xr -= 1 } else { xs -= 1 },
            '>' => if robo { xr += 1 } else { xs += 1 },
            'v' => if robo { yr -= 1 } else { ys -= 1 },
            '^' => if robo { yr += 1 } else { ys += 1 },
            _ => ()
        }
        set.insert(if robo { (xr, yr) } else { (xs, ys) });
        robo = !robo;
    }
    println!("# houses: {}", set.len());
}
