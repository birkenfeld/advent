use advtools::input;
use advtools::prelude::HashSet;

const LINE: &str = r"(\d+),(\d+),(\d+)";

fn main() {
    // Parse the cube coordinates, keeping track of min/max coordinates.  Use an
    // offset of +/-1 to keep a full layer of air in our "operating area".
    let (mut minx, mut maxx, mut miny, mut maxy, mut minz, mut maxz) = (100, 0, 100, 0, 100, 0);
    let cubes = input::rx_lines::<(i32, i32, i32)>(LINE).inspect(|&(x, y, z)| {
        minx = minx.min(x - 1);
        maxx = maxx.max(x + 1);
        miny = miny.min(y - 1);
        maxy = maxy.max(y + 1);
        minz = minz.min(z - 1);
        maxz = maxz.max(z + 1);
    }).collect::<HashSet<_>>();

    // A simple function to iterate over all of a cube's neighbors that are
    // within the operating area.
    let for_neighbors = |(x, y, z)| {
        let mut v = Vec::with_capacity(6);
        if x > minx { v.push((x - 1, y, z)); }
        if x < maxx { v.push((x + 1, y, z)); }
        if y > miny { v.push((x, y - 1, z)); }
        if y < maxy { v.push((x, y + 1, z)); }
        if z > minz { v.push((x, y, z - 1)); }
        if z < maxz { v.push((x, y, z + 1)); }
        v.into_iter()
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
    queue.insert((minx, miny, minz));
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
