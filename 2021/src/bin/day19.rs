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

const FORMAT: &str = r"(-?\d+),(-?\d+),(-?\d+)|(.*)";
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
    let mut scanner_beacons: Vec<Vec<_>> = vec![];
    for line in input::rx_lines::<Option<(i32, i32, i32)>>(FORMAT) {
        if let Some(c) = line {
            scanner_beacons.last_mut().unwrap().push(Vector(c.0, c.1, c.2));
        } else {
            scanner_beacons.push(vec![]);
        }
    }
    let mut orients = HashMap::new();
    let mut positions = HashMap::new();
    orients.insert(0, ORIENT[0]);
    positions.insert(0, Vector(0, 0, 0));

    let mut todo: HashSet<_> = (1..scanner_beacons.len()).collect();

    'outer:
    while !todo.is_empty() {
        for &j in &todo {
            for (&i, i_orient) in &orients {
                if let Some((j_orient, rel_vec)) = ORIENT.par_iter().find_map_any(|&j_orient| {
                    let mut counts = HashMap::<_, u8>::new();
                    iproduct!(&scanner_beacons[i], &scanner_beacons[j])
                        .map(|(&bi, &bj)| i_orient(bi) - j_orient(bj))
                        .find_map(|rel_vec| {
                            let count = counts.entry(rel_vec).or_default();
                            *count += 1;
                            if *count == 12 { Some((j_orient, rel_vec)) } else { None }
                        })
                }) {
                    todo.take(&j);
                    orients.insert(j, j_orient);
                    positions.insert(j, positions[&i] + rel_vec);
                    continue 'outer;
                }
            }
        }
    }

    let mut beacons = HashSet::new();
    for (i, coords) in scanner_beacons.iter().enumerate() {
        let pos = positions[&i];
        let orient = orients[&i];
        for &coord in coords {
            beacons.insert(pos + orient(coord));
        }
    }
    advtools::verify("Number of beacons", beacons.len(), 419);

    let max_dist = positions.values().tuple_combinations().map(|(&p1, &p2)| (p1 - p2).dist()).max().unwrap();
    advtools::verify("Max distance", max_dist, 13210);
}
