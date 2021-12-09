use odds::vec::VecExt;
use advtools::itertools::Itertools;
use advtools::input::{iter_lines, to_i32};

fn main() {
    let mut input = iter_lines();

    let draws = input.next().unwrap().split(',').map(to_i32).collect_vec();
    let mut boards = vec![];

    loop {
        let board = input.by_ref().take(5).map(|line| {
            line.split_whitespace().map(|i| (to_i32(i), false)).collect_vec()
        }).flatten().collect_vec();
        if board.is_empty() {
            break;
        }
        boards.push(board);
    }

    let mut first = None;
    let mut last = None;

    for draw in draws {
        boards.retain_mut(|board| {
            board.iter_mut().for_each(|(n, marked)| if *n == draw { *marked = true; });
            let has_bingo = (0..5).any(|i| {
                (0..5).all(|j| board[5*i + j].1) || (0..5).all(|j| board[i + 5*j].1)
            });
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
