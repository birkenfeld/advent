use advtools::input;
use advtools::prelude::Itertools;

// Find all the possible mirror axis positions.
fn mirrors(lines: &[u128], factor: u32) -> impl Iterator<Item=u32> + '_ {
    (0..lines.len()-1)
        .filter(move |i| {
            let max_j = (lines.len() - i - 1).min(i + 1);
            (0..max_j).all(|j| lines[i-j] == lines[i+j+1])
        })
        .map(move |i| (i + 1) as u32 * factor)
}

// Try to smudge every pixel and see if it finds a new mirror axis.
fn smudged(rows: &mut [u128], cols: &mut [u128], original: u32) -> u32 {
    (0..rows.len()).cartesian_product(0..cols.len()).find_map(|(i, j)| {
        rows[i] ^= 1 << j;
        cols[j] ^= 1 << i;
        let result = mirrors(&rows, 100).chain(mirrors(&cols, 1))
                                        .find(|&n| n != original);
        rows[i] ^= 1 << j;
        cols[j] ^= 1 << i;
        result
    }).unwrap()
}


fn main() {
    // Parse input and transpose it.
    // Since lines are short, they are represented as u128.
    let mut grids = input::string().split("\n\n").map(|pattern| {
        let mut row_len = 0;
        let rows = pattern.split('\n').map(|v| {
            row_len = v.len();
            v.chars().enumerate().map(|(i, c)| ((c == '#') as u128) << i).sum::<u128>()
        }).collect_vec();
        let cols = (0..row_len).map(|i| {
            rows.iter().enumerate().map(|(j, r)| ((r & (1 << i) != 0) as u128) << j).sum::<u128>()
        }).collect_vec();
        (rows, cols)
    }).collect_vec();

    // Get the original mirror axes for part 1, with the given grids.
    let original = grids.iter().map(|(rows, cols)| {
        mirrors(rows, 100).chain(mirrors(cols, 1)).exactly_one().ok().unwrap()
    }).collect_vec();
    advtools::verify("Clean summary", original.iter().sum::<u32>(), 29213);

    // Find the smudge point in each grid.
    let smudged = grids.iter_mut().zip(original).map(|((rows, cols), orig)| {
        smudged(rows, cols, orig)
    }).sum::<u32>();
    advtools::verify("Smudged summary", smudged, 37453);
}
