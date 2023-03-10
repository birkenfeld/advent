use advtools::prelude::{iproduct, Itertools};
use advtools::input;
use advtools::grid::{Grid, Pos};

const N: usize = 140;

fn main() {
    let mut tiles = Grid::fill(false, N, N);
    for line in input::lines() {
        let mut pos = tiles.center::<i32>();
        let mut chars = line.chars();
        while let Some(ch) = chars.next() {
            match ch {
                'e' => pos = pos.right(),
                'w' => pos = pos.left(),
                's' => {
                    pos = pos.down();
                    if chars.next() == Some('w') { pos = pos.left(); }
                }
                'n' => {
                    pos = pos.up();
                    if chars.next() == Some('e') { pos = pos.right(); }
                }
                _ => unreachable!()
            }
        }
        tiles[pos] ^= true;
    }
    advtools::verify("Black tiles at start", tiles.count(|x| *x), 360);

    for _ in 0..100 {
        let flips = iproduct!(1..N as i32-1, 1..N as i32-1).filter_map(|(x, y)| {
            let black = tiles[Pos(x, y)];
            let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1), (-1, 1), (1, -1)]
                .iter().filter(|(dx, dy)| tiles[Pos(x + dx, y + dy)]).count();
            ((black && (neighbors == 0 || neighbors > 2)) || (!black && neighbors == 2))
                .then(|| Pos(x, y))
        }).collect_vec();
        flips.into_iter().for_each(|pos| tiles[pos] ^= true);
    }
    advtools::verify("Black tiles after 100 days", tiles.count(|x| *x), 3924);
}
