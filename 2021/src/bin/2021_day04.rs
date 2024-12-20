use odds::vec::VecExt;
use advtools::itertools::Itertools;
use advtools::input;

fn main() {
    // Read first line with the drawn numbers.
    let draws = input::parse_lines::<input::Sep<i32>>().next().unwrap().vec;

    // Read the boards.
    let mut input = input::parse_lines::<Option<[i32; 5]>>().skip(1).peekable();
    let mut boards = vec![];
    while input.peek().is_some() {
        boards.push(input.by_ref().take(5).flat_map(|line| {
            line.unwrap().map(|i| (i, false))
        }).collect_vec());
    }

    let mut first = None;
    let mut last = None;

    for draw in draws {
        // For each number, remove boards with bingos and record the score.
        VecExt::retain_mut(&mut boards, |board| {
            // Mark off the drawn number.
            board.iter_mut().for_each(|(n, marked)| if *n == draw { *marked = true; });
            // Check for bingo: either horizontal or vertical at each position.
            let has_bingo = (0..5).any(|i| {
                (0..5).all(|j| board[5*i + j].1) || (0..5).all(|j| board[i + 5*j].1)
            });
            // Calculate the score and update the first/last score.
            if has_bingo {
                let unmarked_sum: i32 = board.iter().filter(|e| !e.1).map(|e| e.0).sum();
                let score = Some(draw * unmarked_sum);
                first = first.or(score);
                last = score;
                return false;
            }
            true
        });
    }

    advtools::verify("First board score", first.unwrap(), 38594);
    advtools::verify("Last board score", last.unwrap(), 21184);
}
