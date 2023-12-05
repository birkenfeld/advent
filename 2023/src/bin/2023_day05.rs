use advtools::input;
use advtools::prelude::Itertools;
use std::collections::BTreeSet;

const N_TRANS: usize = 7;

fn lowest_loc(maps: &[BTreeSet<(u64, u64, u64)>], mut ranges: BTreeSet<(u64, u64)>) -> u64 {
    // Go through all 7 mappings.
    for i in 0..N_TRANS {
        // For each, take the current list of ranges and collect a new,
        // potentially bigger, list of ranges, splitting them up wherever they
        // overlap mapped ranges.
        for (mut start, mut len) in std::mem::take(&mut ranges) {
            for &(rng_src, rng_len, rng_dst) in &maps[i] {
                // Any overlap with this mapping range?
                if start + len <= rng_src {
                    break;
                } else if start >= rng_src + rng_len {
                    continue;
                }
                // There is some overlap.  Is there some non-mapped range before?
                if rng_src > start {
                    let len_before = rng_src - start;
                    ranges.insert((start, len_before));
                    start += len_before;
                    len -= len_before;
                }
                // Determine length of overlap.
                let off_ovl = start - rng_src;
                let len_ovl = len.min(rng_len - off_ovl);
                ranges.insert((rng_dst + off_ovl, len_ovl));
                start += len_ovl;
                len -= len_ovl;
            }
            // Map the final part of the source range.
            if len > 0 {
                ranges.insert((start, len));
            }
        }
    }
    ranges.first().unwrap().0
}

fn main() {
    let mut parts = input::string().split("\n\n");

    // Parse the seeds line.
    let seeds = parts.next().unwrap().split_whitespace()
                                     .skip(1)
                                     .map(|s| s.parse::<u64>().unwrap())
                                     .collect::<Vec<_>>();
    // Parse the 7 maps.
    let maps = parts.map(|part| {
        part.lines().skip(1).map(|line| {
            let (dst, src, len) = line.split(' ').map(|v| v.parse::<u64>().unwrap())
                                                 .collect_tuple().unwrap();
            (src, len, dst)
        }).collect::<BTreeSet<_>>()
    }).collect_vec();

    // Part 1: make single-element ranges.
    let part1 = seeds.iter().map(|&s| (s, 1)).collect();
    // Part 2: use the full ranges.
    let part2 = seeds.into_iter().tuples().collect();

    advtools::verify("Lowest location", lowest_loc(&maps, part1), 196167384);
    advtools::verify("Lowest location with ranges", lowest_loc(&maps, part2), 125742456);
}
