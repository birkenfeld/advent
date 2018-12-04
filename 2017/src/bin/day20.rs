extern crate advtools;
use advtools::prelude::Itertools;
use advtools::input::{iter_input, to_i64};
use std::ops::Add;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vector { x: i64, y: i64, z: i64 }

#[derive(Clone, Debug)]
struct Particle {
    p: Vector,
    v: Vector,
    a: Vector,
}

impl Vector {
    fn new<I: Iterator<Item=i64>>(mut it: I) -> Vector {
        Vector { x: it.next().unwrap(), y: it.next().unwrap(), z: it.next().unwrap() }
    }
    fn dist(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

/// Calculate time of crossing of one vector component based on the time
/// dependency: p(t) = p0 + t*v0 + 0.5*t(t+1)*a0.
///
/// Particles only collide if a) the crossing is at an integer time, and
/// b) all three coordinates cross at the same time.
fn cross<F: Fn(&Vector) -> f64>(p1: &Particle, p2: &Particle, f: F) -> f64 {
    let da = f(&p1.a) - f(&p2.a);
    if da != 0. {
        let dv = f(&p1.v) - f(&p2.v);
        let dp = f(&p1.p) - f(&p2.p);
        let b = 0.5 + dv/da;
        let d = b.powi(2) - 2.*dp/da;
        if d >= 0. {
            return (-b + d.sqrt()).max(-b - d.sqrt());
        }
    }
    0.
}

fn main() {
    let mut particles = Vec::new();
    for line in iter_input::<Vec<String>>() {
        let p = Vector::new(line[0][3..line[0].len()-2].split(',').map(to_i64));
        let v = Vector::new(line[1][3..line[1].len()-2].split(',').map(to_i64));
        let a = Vector::new(line[2][3..line[2].len()-1].split(',').map(to_i64));
        particles.push(Particle { p, v, a });
    }

    // Part 1: Determine particle that will stay closest to origin for t -> oo.
    // The most important attribute for staying "close" is acceleration,
    // so consider only those with the minimum possible acceleration.
    let mut particles2 = particles.iter().cloned().enumerate().collect_vec();
    particles2.retain(|&(_, ref p)| p.a.dist() == 1);

    // Let the simulation run for a while, determine closest particle.
    for _ in 0..1000 {
        for &mut (_, ref mut p) in &mut particles2 {
            p.v = p.v + p.a;
            p.p = p.p + p.v;
        }
    }
    let min_p = particles2.into_iter().min_by_key(|&(_, ref p)| p.p.dist()).unwrap();
    println!("Particle nearest origin: {}", min_p.0);

    // Part 2: Determine how many particles are left after all collisions happened.
    let mut max_turn = 0;
    // Determine all possible turns on which collisions could happen, and take
    // the maximum.
    for (p1, p2) in particles.iter().tuple_combinations() {
        let turn_x = cross(p1, p2, |v| v.x as f64);
        let turn_y = cross(p1, p2, |v| v.y as f64);
        // Collisions for x/y/z must be on the same turn.
        if (turn_x - turn_y).abs() < 1. {
            let turn_z = cross(p1, p2, |v| v.z as f64);
            if (turn_y - turn_z).abs() < 1. {
                max_turn = max_turn.max(turn_x as i64 + 1);
            }
        }
    }

    // Now run until the determined max turn, and remove colliding particles.
    for _ in 0..max_turn {
        for p in &mut particles {
            p.v = p.v + p.a;
            p.p = p.p + p.v;
        }

        // Need to remove in descending index order, which BTreeSet makes easy.
        let mut remove = BTreeSet::new();
        for (i, p1) in particles.iter().enumerate() {
            for (j, p2) in particles[i+1..].iter().enumerate() {
                if p1.p == p2.p {
                    remove.insert(i);
                    remove.insert(i+j+1);
                }
            }
        }
        for index in remove.into_iter().rev() {
            particles.remove(index);
        }
    }

    println!("Particles left after collisions: {}", particles.len());
}
