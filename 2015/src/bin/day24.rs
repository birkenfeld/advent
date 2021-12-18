use advtools::input;

fn find_batches(v: &[u32], i: usize, cand: u32, rest: u32,
                best_count: &mut u32, batches: &mut Vec<(u64, u32)>) {
    if rest == 0 {
        let pkg_count = cand.count_ones();
        if *best_count == 0 || pkg_count < *best_count {
            batches.clear();
            *best_count = pkg_count;
        }
        if pkg_count == *best_count {
            let qe = v.iter().enumerate().filter(|&(i, _)| cand & (1 << i) != 0)
                                         .map(|(_, v)| *v as u64).product();
            batches.push((qe, cand));
        }
    } else if i < v.len() {
        if rest >= v[i] {
            find_batches(v, i+1, cand | (1 << i), rest - v[i], best_count, batches);
        }
        find_batches(v, i+1, cand, rest, best_count, batches);
    }
}

fn find_configuration(weights: &[u32], batch_weight: u32) -> u64 {
    let mut candidates = Vec::new();
    let mut best_count = 0;
    find_batches(weights, 0, 0, batch_weight, &mut best_count, &mut candidates);
    candidates.sort_unstable();
    candidates[0].0
}

fn main() {
    let mut weights = input::parse_vec::<u32>();
    let total_weight = weights.iter().sum::<u32>();
    weights.reverse();
    advtools::verify(&format!("Lowest QE (batch weight {})", total_weight / 3),
                     find_configuration(&weights, total_weight / 3), 10723906903_u64);
    advtools::verify(&format!("Lowest QE (batch weight {})", total_weight / 4),
                     find_configuration(&weights, total_weight / 4), 74850409);
}
