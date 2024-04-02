use advtools::input;
use advtools::vecs::i64::Vec3;
use advtools::prelude::Itertools;

const RX: &str = r"(\d+), (\d+), (\d+) @ (-?\d+), (-?\d+), (-?\d+)";

fn main() {
    let stones = input::rx_lines::<(Vec3, Vec3)>(RX).collect_vec();

    for ((p1, v1), (p2, v2)) in stones.iter().tuple_combinations() {
        
    }
    // advtools::verify("Longest hike without slopes", longest, 6450);
}
