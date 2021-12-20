use advtools::rayon::prelude::*;
use advtools::prelude::{iproduct, HashMap, HashSet, Itertools};
use advtools::input;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vector(i32, i32, i32);

impl Vector {
    fn dist(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

const FORMAT: &str = r"([-\d]+),([-\d]+),([-\d]+)|(.*)";
const ORIENT: &[fn(Vector) -> Vector] = &[
    // Two exchanges, two negations
    |Vector(x, y, z)| Vector(x, y, z),
    |Vector(x, y, z)| Vector(x, -y, -z),
    |Vector(x, y, z)| Vector(-x, y, -z),
    |Vector(x, y, z)| Vector(-x, -y, z),
    |Vector(x, y, z)| Vector(y, z, x),
    |Vector(x, y, z)| Vector(y, -z, -x),
    |Vector(x, y, z)| Vector(-y, z, -x),
    |Vector(x, y, z)| Vector(-y, -z, x),
    |Vector(x, y, z)| Vector(z, x, y),
    |Vector(x, y, z)| Vector(z, -x, -y),
    |Vector(x, y, z)| Vector(-z, x, -y),
    |Vector(x, y, z)| Vector(-z, -x, y),
    // One exchange, one or three negations
    |Vector(x, y, z)| Vector(-y, -x, -z),
    |Vector(x, y, z)| Vector(-y, x, z),
    |Vector(x, y, z)| Vector(y, -x, z),
    |Vector(x, y, z)| Vector(y, x, -z),
    |Vector(x, y, z)| Vector(-x, -z, -y),
    |Vector(x, y, z)| Vector(-x, z, y),
    |Vector(x, y, z)| Vector(x, -z, y),
    |Vector(x, y, z)| Vector(x, z, -y),
    |Vector(x, y, z)| Vector(-z, -y, -x),
    |Vector(x, y, z)| Vector(-z, y, x),
    |Vector(x, y, z)| Vector(z, -y, x),
    |Vector(x, y, z)| Vector(z, y, -x),
];

fn main() {
    // Parse the input.
    let mut beacons: Vec<Vec<_>> = vec![];
    for line in input::rx_lines::<Option<(i32, i32, i32)>>(FORMAT) {
        if let Some(c) = line {
            beacons.last_mut().unwrap().push(Vector(c.0, c.1, c.2));
        } else {
            beacons.push(vec![]);  // start of a new scanner
        }
    }
    // Define scanner 0 to be at the origin with known orientation.
    let mut positions: HashMap<_, _> = [(0, Vector(0, 0, 0))].into_iter().collect();
    // Keep track of all beacons relative to the origin.
    let mut all_beacons: HashSet<_> = beacons[0].iter().cloned().collect();
    let mut non_matching: HashSet<_> = HashSet::new();

    // Loop until all scanner locations and orientations are known.
    'outer:
    while positions.len() < beacons.len() {
        for j in (0..beacons.len()).filter(|j| !positions.contains_key(j)) {
            // For each unoriented scanner j, go through the known scanners i
            // and try to find an overlap.
            for &i in positions.keys() {
                // Try each beacon pair only once.
                if non_matching.insert((i, j)) {
                    // Go through all possible orientations of scanner j in parallel.
                    if let Some((orient, pos)) = ORIENT.par_iter().find_map_any(|&orient| {
                        let mut counts = HashMap::<_, u32>::new();
                        // For each pair of beacons, get the relative vector between
                        // the two.  If the scanner ranges overlap, at least 12
                        // beacon pairs will have the same relative vector, which will
                        // also be the position of scanner j (since all known scanners'
                        // beacon coordinates were made relative to the origin).
                        iproduct!(&beacons[i], &beacons[j])
                            .map(|(&bi, &bj)| bi - orient(bj))
                            .find_map(|rel_vec| {
                                let count = counts.entry(rel_vec).or_default();
                                *count += 1;
                                if *count == 12 { Some((orient, rel_vec)) } else { None }
                            })
                    }) {
                        // Found an overlap: convert this scanners' beacon coords to
                        // scanner 0's coordinate system, record its position, and
                        // then start again with the rest.
                        beacons[j].iter_mut().for_each(|p| *p = pos + orient(*p));
                        all_beacons.extend(beacons[j].iter().cloned());
                        positions.insert(j, pos);
                        continue 'outer;
                    }
                }
            }
        }
    }

    // Part 1: count unique coordinates of all beacons.
    advtools::verify("Number of beacons", all_beacons.len(), 419);

    // Part 2: Go over all scanner combinations to get the maximum Manhattan distance.
    let max_dist = positions.values().tuple_combinations()
                                     .map(|(&p1, &p2)| (p1 - p2).dist())
                                     .max().unwrap();
    advtools::verify("Max distance", max_dist, 13210);
}
