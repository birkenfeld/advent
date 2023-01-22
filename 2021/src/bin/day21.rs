use advtools::prelude::HashMap;
use advtools::input;

const FIELDS: u64 = 10;
const OUTCOMES: &[(u64, u64)] = &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Default)]
struct Player {
    to_win: u64,
    wins: u64,
    univ: HashMap<(u64, u64), u64>,
}

impl Player {
    fn new(to_win: u64, init_pos: u64) -> Self {
        let mut univ = HashMap::new();
        univ.insert((init_pos, 0), 1);
        Player {
            to_win, univ, wins: 0,
        }
    }

    // Do a round for the player with a given number of universes, updating the
    // points and universe counts.  Wins are removed from the `pos` list and
    // recorded in `wins`.  Return the remaining number of universes.
    fn play_round(&mut self, universes: u64, dice_outcomes: &[(u64, u64)]) -> u64 {
        for ((pos, score), n) in std::mem::take(&mut self.univ) {
            for &(pts, m) in dice_outcomes {
                let new_pos = ((pos - 1) + pts) % FIELDS + 1;
                let new_score = score + new_pos;
                if new_score >= self.to_win {
                    self.wins += n*m * universes;
                } else if n > 0 {
                    *self.univ.entry((new_pos, new_score)).or_default() += n*m;
                }
            }
        }
        self.univ.values().sum()
    }

    fn max_score(&self) -> u64 {
        *self.univ.keys().map(|(_, score)| score).max().unwrap()
    }
}

fn main() {
    let start: (u64, u64) = input::rx_parse(r".*: (\d+)\n.*: (\d+)");

    // Part 1: Keep track of the number of dice casts.
    let mut casts = 0;
    let mut roll = || { let eyes = (casts % 100) + 1; casts += 1; eyes };
    let mut p1 = Player::new(1000, start.0);
    let mut p2 = Player::new(1000, start.1);
    loop {
        if p1.play_round(1, &[(roll() + roll() + roll(), 1)]) == 0 {
            break;
        }
        assert!(p2.play_round(1, &[(roll() + roll() + roll(), 1)]) == 1);
    }
    advtools::verify("Score after win", p2.max_score() * casts, 888735);

    // Part 2: Keep track of the number of remaining universes.
    let mut univ_count = 1;
    let mut p1 = Player::new(21, start.0);
    let mut p2 = Player::new(21, start.1);
    while univ_count > 0 {
        univ_count = p1.play_round(univ_count, OUTCOMES);
        univ_count = p2.play_round(univ_count, OUTCOMES);
    }
    advtools::verify("Universes for winner", p1.wins.max(p2.wins), 647608359455719u64);
}
