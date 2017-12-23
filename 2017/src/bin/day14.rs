#![feature(i128_type)]

extern crate advtools;
extern crate advent17;
use advtools::prelude::*;

const INPUT: &str = "jxqlasbh";

/// Zero out all one-bits in the region containing (x,y).
fn zero_fill(disk: &mut [u128], x: usize, y: usize) -> bool {
    if disk[y] & (1 << x) != 0 {
        disk[y] &= !(1 << x);
        if x > 0 {
            zero_fill(disk, x-1, y);
        }
        if x < 127 {
            zero_fill(disk, x+1, y);
        }
        if y > 0 {
            zero_fill(disk, x, y-1);
        }
        if y < 127 {
            zero_fill(disk, x, y+1);
        }
        true
    } else {
        false
    }
}

fn main() {
    // Build up the disk from 128 rows of 128 cells.  We represent each cell as
    // a bit in a row with type u128.
    let mut disk = (0..128).map(|row| {
        let hash = advent17::knot_hash(format!("{}-{}", INPUT, row));
        hash.into_iter().rev().enumerate().map(|(ofs, val)| (val as u128) << (ofs*8)).sum::<u128>()
    }).collect_vec();
    // Part 1: Count the number of one-bits.
    println!("Used bits: {}", disk.iter().map(|b| b.count_ones()).sum::<u32>());

    // Part 2: Count the number of regions of one-bits.  For each one-bit encountered, call
    // `zero_fill` to zero out all bits in the same region before continuing.
    let regions = (0..128).cartesian_product(0..128).filter(
        |&(y, x)| zero_fill(&mut disk, x, y)
    ).count();
    println!("Number of regions: {}", regions);
}
