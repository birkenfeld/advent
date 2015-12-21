use std::cmp::max;

const BOSS_HP: i32 = 103;
const BOSS_DMG: i32 = 9;
const BOSS_DEF: i32 = 2;

type Info = (i32, i32, i32);

const WEAPONS: [Info; 5] = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
const ARMOR:   [Info; 5] = [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
const RINGS:   [Info; 7] = [(0, 0, 0), (20, 0, 1), (25, 1, 0), (40, 0, 2),
                            (50, 2, 0), (80, 0, 3), (100, 3, 0)];

fn fight_to_the_death(mut my_hp: i32, my_dmg: i32, my_def: i32) -> bool {
    let mut boss_hp = BOSS_HP;
    loop {
        boss_hp -= max(1, my_dmg - BOSS_DEF);
        if boss_hp <= 0 { return true; }
        my_hp -= max(1, BOSS_DMG - my_def);
        if my_hp <= 0 { return false; }
    }
}

fn main() {
    let mut min_gold = 0;
    let mut max_gold = 0;
    for &w in &WEAPONS {
        for &a in &ARMOR {
            for &r1 in &RINGS {
                for &r2 in &RINGS {
                    if r1.0 > 0 && r1 == r2 { continue; }
                    let cost = w.0 + a.0 + r1.0 + r2.0;
                    let result = fight_to_the_death(100, w.1 + r1.1 + r2.1,
                                                    a.2 + r1.2 + r2.2);
                    if result && (cost < min_gold || min_gold == 0) {
                        min_gold = cost;
                    }
                    if !result && cost > max_gold {
                        max_gold = cost;
                    }
                }
            }
        }
    }
    println!("Min gold for win: {}", min_gold);
    println!("Max gold for loss: {}", max_gold);
}
