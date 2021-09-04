use std::mem::replace;
use advtools::prelude::{Itertools, HashMap, FromIterator, binary_search};
use advtools::input::iter_lines;

const STOCK: u64 = 1_000_000_000_000;  // 1 trillion

fn quantity(s: &str) -> (&str, u64) {
    let (n, el) = s.split(' ').collect_tuple().unwrap();
    (el, n.parse().unwrap())
}

fn main() {
    let input = iter_lines().collect_vec();
    let mut recipes = HashMap::new();

    for line in &input {
        let (input, output) = line.split(" => ").collect_tuple().unwrap();
        let (prod_el, prod_count) = quantity(&output);
        let reagents = input.split(", ").map(quantity).collect_vec();
        recipes.insert(prod_el, (prod_count, reagents));
    }

    // Determine the ore needed to produce a certain amount of fuel.
    let ore_to_produce_fuel = |fuel_to_produce: u64| {
        let mut ore_needed = 0;
        // Keep track of elements still required
        let mut to_produce = HashMap::from_iter(Some(("FUEL", fuel_to_produce)));
        // and leftovers from previous reactions that weren't used up.
        let mut left_over = HashMap::new();
        while !to_produce.is_empty() {
            for (product, mut count) in replace(&mut to_produce, HashMap::new()) {
                // See if some leftover can be used for this product.
                let count_left = left_over.entry(product).or_default();
                let transfer = count.min(*count_left);
                count -= transfer;
                *count_left -= transfer;
                if count > 0 {
                    // If we still need to produce, determine the number of reactions
                    // to get the required number and keep track of the leftovers.
                    let (prod_count, reagents) = &recipes[&product];
                    let recipe_times = 1 + (count - 1)/prod_count;
                    *count_left += recipe_times*prod_count - count;
                    for (reagent, ingr_count) in reagents {
                        if reagent == &"ORE" {
                            // Ore is just counted.
                            ore_needed += recipe_times*ingr_count;
                        } else {
                            *to_produce.entry(*reagent).or_default() += recipe_times*ingr_count;
                        }
                    }
                }
            }
        }
        ore_needed
    };

    // Part 1: Just produce one fuel.
    let single_fuel = ore_to_produce_fuel(1);
    advtools::verify("Required ore for 1 fuel", single_fuel, 346961);

    // Part 2: Make a binary search.  Initial bounds are given by what could
    // be expected from the single-fuel case (which underestimates severely)
    // and twice that amount.
    let exp = STOCK/single_fuel;
    let result = binary_search(exp, 2*exp, |f| ore_to_produce_fuel(f) > STOCK);
    advtools::verify("Fuel produced with 1tn ore", result - 1, 4065790);
}
