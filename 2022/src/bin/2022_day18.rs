use advtools::input;
use advtools::prelude::HashSet;
use advtools::vecs::i32::*;

const LINE: &str = r"(\d+),(\d+),(\d+)";

fn main() {
    // Parse the cube coordinates, keeping track of min/max coordinates.  Use an
    // offset of +/-1 to keep a full layer of air in our "operating area".
    let mut min = Vec3::splat(i32::MAX);
    let mut max = Vec3::splat(i32::MIN);
    let cubes = input::rx_lines::<(i32, i32, i32)>(LINE).map(|(x, y, z)| {
        let pos = vec3(x, y, z);
        min = min.zip(pos, |old, new| old.min(new - 1));
        max = max.zip(pos, |old, new| old.max(new + 1));
        pos
    }).collect::<HashSet<_>>();

    // A simple function to iterate over all of a cube's neighbors that are
    // within the operating area.
    let for_neighbors = |pt: Vec3| {
        let mut r = Vec::with_capacity(6);
        if pt.x > min.x { r.push(pt - X3); }
        if pt.x < max.x { r.push(pt + X3); }
        if pt.y > min.y { r.push(pt - Y3); }
        if pt.y < max.y { r.push(pt + Y3); }
        if pt.z > min.z { r.push(pt - Z3); }
        if pt.z < max.z { r.push(pt + Z3); }
        r.into_iter()
    };

    // Part 1: check each cube side for being not another cube.
    let surface = cubes.iter().map(
        |&pt| for_neighbors(pt).filter(|npt| !cubes.contains(npt)).count()
    ).sum::<usize>();
    advtools::verify("Initial surface area", surface, 3494);

    // Find all possible coordinates within the operating area that the water
    // can reach.
    let mut water = HashSet::new();
    let mut queue = HashSet::new();
    queue.insert(min);
    while !queue.is_empty() {
        for pt in std::mem::take(&mut queue) {
            water.insert(pt);
            queue.extend(for_neighbors(pt).filter(
                |npt| !water.contains(npt) && !cubes.contains(npt)
            ));
        }
    }

    // Part 2: check each cube side for being water-reachable.
    let surface = cubes.iter().map(
        |&pt| for_neighbors(pt).filter(|npt| water.contains(npt)).count()
    ).sum::<usize>();
    advtools::verify("Outer surface area", surface, 2062);
}
