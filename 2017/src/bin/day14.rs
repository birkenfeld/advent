#![feature(i128_type)]

extern crate advtools;
extern crate advent17;
use advtools::prelude::*;

const INPUT: &str = "jxqlasbh";

fn sweep(disk: &mut [u128], x: usize, y: usize) -> bool {
    if disk[y] & (1 << x) != 0 {
        disk[y] &= !(1 << x);
        if x > 0 {
            sweep(disk, x-1, y);
        }
        if x < 127 {
            sweep(disk, x+1, y);
        }
        if y > 0 {
            sweep(disk, x, y-1);
        }
        if y < 127 {
            sweep(disk, x, y+1);
        }
        true
    } else {
        false
    }
}

fn main() {
    let mut disk = (0..128).map(|row| {
        let hash = advent17::knot_hash(format!("{}-{}", INPUT, row));
        hash.into_iter().rev().enumerate().map(|(ofs, val)| (val as u128) << (ofs*8)).sum::<u128>()
    }).collect_vec();
    println!("Used bits: {}", disk.iter().map(|b| b.count_ones()).sum::<u32>());

    let regions = (0..128).cartesian_product(0..128).filter(|&(y, x)| sweep(&mut disk, x, y)).count();
    println!("Number of regions: {}", regions);
}
