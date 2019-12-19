use advtools::prelude::Itertools;
use advtools::input::iter_input;
use float_ord::FloatOrd;

fn iter_asteroids<'s>(map: &'s Vec<Vec<bool>>) -> impl Iterator<Item=(usize, usize)> + 's {
    map.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter(|p| *p.1).map(move |(x, _)| (x, y))
    })
}

fn main() {
    let map = iter_input::<String>().map(
        |s| s.trim().chars().map(|c| c == '#').collect_vec()
    ).collect_vec();

    // Iterate over all possible station positions.  The inner iterator returns
    // all visible asteroids, which we need for part 2.
    let visible = iter_asteroids(&map).map(|(xc, yc)| {
        // Sort all asteroids by their polar angle and distance, then
        // weed out those with the same polar angle.
        iter_asteroids(&map).map(|(x, y)| {
            let yd = y as f64 - yc as f64;
            let xd = x as f64 - xc as f64;
            // We define the polar angle mirrored from the usual definition
            // y.atan2(x), so that we sort by the laser's movement: starting
            // at the y axis and going clockwise.
            (FloatOrd(-xd.atan2(yd)), FloatOrd(yd.hypot(xd)), x, y)
        }).sorted().dedup_by(|x, y| x.0 == y.0).collect_vec()
    }).max_by_key(|v| v.len()).unwrap();

    advtools::print("Maximum visible asteroids", visible.len());
    advtools::print("200th destroyed", 100*visible[199].2 + visible[199].3);
}
