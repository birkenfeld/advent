extern crate advtools;

use advtools::prelude::*;
use std::ops::*;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vector((i64, i64, i64));

#[derive(Clone, Debug)]
struct Particle {
    p: Vector,
    v: Vector,
    a: Vector,
}

impl Vector {
    fn dist(&self) -> i64 {
        (self.0).0.abs() + (self.0).1.abs() + (self.0).2.abs()
    }
    fn x(&self) -> i64 { (self.0).0 }
    fn y(&self) -> i64 { (self.0).1 }
    fn z(&self) -> i64 { (self.0).2 }
}

impl Mul<i64> for Vector {
    type Output = Vector;
    fn mul(self, other: i64) -> Vector {
        Vector(((self.0).0 * other,
                (self.0).1 * other,
                (self.0).2 * other))
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector(((self.0).0 + (other.0).0,
                (self.0).1 + (other.0).1,
                (self.0).2 + (other.0).2))
    }
}

fn cross<F: Fn(&Vector) -> f64>(p1: &Particle, p2: &Particle, f: F) -> f64 {
    // crossing based on: p(n) = p0 + n*v0 + 0.5*n(n+1)*a0
    let da = f(&p1.a) - f(&p2.a);
    let dv = f(&p1.v) - f(&p2.v);
    let dp = f(&p1.p) - f(&p2.p);
    if da != 0. {
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
        let p = Vector(line[0][3..line[0].len()-2].split(',').map(to_i64).collect_tuple().unwrap());
        let v = Vector(line[1][3..line[1].len()-2].split(',').map(to_i64).collect_tuple().unwrap());
        let a = Vector(line[2][3..line[2].len()-1].split(',').map(to_i64).collect_tuple().unwrap());
        particles.push(Particle { p, v, a });
    }

    let mut particles2 = particles.clone();
    for _ in 0..1000 {
        for p in &mut particles2 {
            p.v = p.v + p.a;
            p.p = p.p + p.v;
        }
    }
    let min_p = particles2.iter().enumerate().min_by_key(|&(_, ref p)| p.p.dist()).unwrap();
    println!("Min position: {}", min_p.0);

    let mut max_turn = 0;
    for (i, p1) in particles.iter().enumerate() {
        for (_, p2) in particles[i+1..].iter().enumerate() {
            let turn_x = cross(p1, p2, |v| v.x() as f64);
            let turn_y = cross(p1, p2, |v| v.y() as f64);
            let turn_z = cross(p1, p2, |v| v.z() as f64);
            if (turn_x - turn_y).abs() < 1. && (turn_y - turn_z).abs() < 1. {
                max_turn = max_turn.max(turn_x as i64 + 1);
            }
        }
    }

    for _ in 0..max_turn {
        for p in &mut particles {
            p.v = p.v + p.a;
            p.p = p.p + p.v;
        }

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

    println!("Particles left: {}", particles.len());
}
