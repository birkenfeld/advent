use advtools::input;
use advtools::itertools::{Itertools, repeat_n};
use advtools::memoize::memoize;
use advtools::rayon::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Good,
    Bad,
    Unknown,
}

// Check if the target pattern could have a damaged group of length *damaged*
// after *before* empty positions, and afterwards at least *after* empty positions.
fn matches_spring(condition: &[Spring], before: usize, damaged: usize, after: usize) -> bool {
    let mut it = condition.iter();
    it.by_ref().take(before).all(|&s| s != Spring::Bad) &&
        it.by_ref().take(damaged).all(|&s| s != Spring::Good) &&
        it.take(after).all(|&s| s != Spring::Bad)
}

// Main recursive counting function.
#[memoize]
fn count_arrangements(groups: Vec<usize>, condition: Vec<Spring>, slack: usize) -> usize {
    // Try all arrangements of the first group with different amounts of "slack"
    // (i.e. additional non-damaged springs between groups).
    (0..slack+1).map(|m| {
        if groups.len() == 1 {
            matches_spring(&condition, m, groups[0], usize::MAX) as usize
        } else if matches_spring(&condition, m, groups[0], 1) {
            count_arrangements(groups[1..].into(),
                               condition[m + groups[0] + 1..].into(),
                               slack - m)
        } else {
            0
        }
    }).sum()
}

fn count(all_springs: Vec<(Vec<Spring>, Vec<usize>)>) -> usize {
    all_springs.into_par_iter().map(|(springs, groups)| {
        let slack = springs.len() - (groups.iter().sum::<usize>() + groups.len() - 1);
        count_arrangements(groups, springs, slack)
    }).sum()
}

fn main() {
    // Parse the input.
    let springs = input::parse_lines::<(&str, &str)>().map(|(pos, counts)| {
        let pos = pos.chars().map(|ch| match ch {
            '.' => Spring::Good,
            '#' => Spring::Bad,
            _   => Spring::Unknown
        }).collect_vec();
        let counts = counts.split(',').map(|v| v.parse::<usize>().unwrap()).collect_vec();
        (pos, counts)
    }).collect_vec();

    advtools::verify("Combinations", count(springs.clone()), 7633);

    // Part 2: multiply the input.
    let new_springs = springs.into_iter().map(|(pos, counts)| {
        (repeat_n(pos, 5).interleave(repeat_n(vec![Spring::Unknown], 4)).concat(),
         repeat_n(counts, 5).concat())
    }).collect();

    advtools::verify("5x Combinations", count(new_springs), 23903579139437_usize);
}
