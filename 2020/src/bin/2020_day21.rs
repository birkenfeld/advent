use advtools::prelude::{HashMap, HashSet, Itertools};
use advtools::input;

fn main() {
    // Parse input
    let prods = input::rx_lines::<(input::Set<&str, ' '>, input::Set<&str>)>(
        r"(.+) \(contains (.+)\)").collect_vec();

    let mut allg_map = HashMap::new();
    let mut candidates: HashMap<_, HashSet<_>> = HashMap::new();

    // For every ingredient, find the possible allergens
    for (ingrs, allgs) in &prods {
        for &ingr in &ingrs.set {
            let for_ingr = candidates.entry(ingr).or_default();
            for &allg in &allgs.set {
                // To be a candidate, for all other products, the ingredient
                // must be present, or the allergen must NOT be present
                if prods.iter().all(|(other_ingrs, other_allgs)| {
                    other_ingrs.set.contains(ingr) || !other_allgs.set.contains(allg)
                }) {
                    for_ingr.insert(allg);
                }
            }
        }
    }

    let count = candidates
        .iter()
        .filter(|(_, cand)| cand.is_empty())
        .map(|(ingr, _)| prods.iter().filter(|(ingrs, _)| ingrs.set.contains(ingr)).count())
        .sum::<usize>();
    advtools::verify("Occurrence of non-allergics", count, 2627);

    // Loop through the candidates and assign allergens that only have one candidate;
    // if the input is unambiguous, this will terminate
    'main: loop {
        for (&ingr, cand) in candidates.iter() {
            if cand.len() == 1 {
                let allg = cand.iter().cloned().next().unwrap();
                allg_map.insert(allg, ingr);
                for (_, cand) in &mut candidates {
                    cand.remove(&allg);
                }
                continue 'main;
            }
        }
        break;
    }
    let ingr_list = allg_map.into_iter().sorted().map(|(_, ingr)| ingr).format(",");
    advtools::verify("Ingredient list", ingr_list,
                     "hn,dgsdtj,kpksf,sjcvsr,bstzgn,kmmqmv,vkdxfj,bsfqgb");
}
