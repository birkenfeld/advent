use advtools::input;
use advtools::rayon::prelude::*;
use advtools::prelude::{Itertools, HashSet};
use advtools::vecs::i64::*;

const RX: &str = r"Sensor at x=(\d+), y=(\d+): .*? at x=(-?\d+), y=(-?\d+)";

const PT1_ROW: i64 = 2_000_000;
const MAX_ROW: i64 = 4_000_000;

/// Find the intervals of excluded x positions at the given y row,
/// adjusting coordinates by the given closure.
fn run(sensors: &[(Vec2, i64)], y: i64, fixup_coord: impl Fn(i64) -> i64)
       -> impl Iterator<Item=(i64, i64)> {
    sensors.iter().filter_map(|&(s, dist)| {
        // Determine if the sensor's exclusion area takes part in this row.
        let n = dist - (s.y - y).abs();
        // If yes, also clip the x positions by the fixup function (for part 2).
        (n >= 0).then(|| (fixup_coord(s.x - n), fixup_coord(s.x + n)))
    }).sorted().coalesce(|first, second| {
        // Since the invtervals are now sorted by beginning, coalesce them
        // by merging overlapping intervals.
        if second.0 > first.1 + 1 {
            Err((first, second))
        } else {
            Ok((first.0, first.1.max(second.1)))
        }
    })
}

fn main() {
    let mut sensors = vec![];
    let mut pt1_beacons = HashSet::new();
    for (sensor, beacon) in input::rx_lines::<(Vec2, Vec2)>(RX) {
        let dist = (sensor - beacon).manhattan();
        sensors.push((sensor, dist));
        if beacon.y == PT1_ROW {
            pt1_beacons.insert(beacon);
        }
    }
    sensors.sort_by_key(|v| v.0.x);

    // Part 1: find the number of excluded positions.
    // Need to subtract the actual beacons, since they are in the exclusion
    // area but still beacons.
    let excluded_positions = run(&sensors, PT1_ROW, |x| x)
        .map(|v| v.1 - v.0 + 1)
        .sum::<i64>() - pt1_beacons.len() as i64;
    advtools::verify("No beacons possible", excluded_positions, 4793062);

    // Part 2: find the coordinates of the beacon (in parallel).
    let tuning_freq = (0..=MAX_ROW).into_par_iter().find_map_any(|y| {
        // When `run()` returns a second result, it means that the excluded area
        // is not a single continuous interval -> it is the row we seek.
        run(&sensors, y, |x| x.clamp(0, MAX_ROW))
            .nth(1)
            .map(|interval| y + (interval.0 - 1) * 4000000)
    }).unwrap();
    advtools::verify("Tuning frequency", tuning_freq, 10826395253551i64);
}
