use advtools::input;

fn fight(boss_hp: i32, boss_dmg: i32, dmg_per_turn: i32) -> i32 {
    let mut stack = vec!((true, boss_hp, 50_i32, 500_i32, [0; 3], 0_i32, 0_i32));
    let mut min_mana = i32::max_value();
    while let Some((my_turn, mut boss_hp, mut hp, mut mana, mut effects, mana_used, rnds)) = stack.pop() {
        if mana_used > min_mana || rnds > 20 {
            continue;
        }
        let mut def = 0;
        if effects[0] > 0 {
            def += 7;
            effects[0] -= 1;
        }
        if effects[1] > 0 {
            boss_hp -= 3;
            if boss_hp <= 0 {
                min_mana = min_mana.min(mana_used);
                continue;
            }
            effects[1] -= 1;
        }
        if effects[2] > 0 {
            mana += 101;
            effects[2] -= 1;
        }
        if !my_turn {
            hp -= (boss_dmg - def).max(1);
            if hp <= 0 {
                continue;
            }
            stack.push((!my_turn, boss_hp, hp, mana, effects, mana_used, rnds + 1));
        } else {
            hp -= dmg_per_turn;
            if hp <= 0 {
                continue;
            }
            if mana >= 53 {  // Missile
                let new_boss_hp = boss_hp - 4;
                if new_boss_hp < 0 {
                    min_mana = min_mana.min(mana_used);
                } else {
                    stack.push((!my_turn, new_boss_hp, hp, mana - 53,
                                effects, mana_used + 53, rnds + 1));
                }
            }
            if mana >= 73 {  // Drain
                let new_boss_hp = boss_hp - 2;
                if new_boss_hp < 0 {
                    min_mana = min_mana.min(mana_used);
                } else {
                    stack.push((!my_turn, new_boss_hp, hp + 2, mana - 73,
                                effects, mana_used + 73, rnds + 1));
                }
            }
            for (i, &(cost, last)) in [(113, 6), (173, 6), (229, 5)].iter().enumerate() {
                if mana >= cost && effects[i] == 0 {
                    let mut new_effects = effects;
                    new_effects[i] = last;
                    stack.push((!my_turn, boss_hp, hp, mana - cost, new_effects,
                                mana_used + cost, rnds + 1));
                }
            }
        }
    }
    min_mana
}

fn main() {
    let (hp, dmg) = input::rx_parse(r"Hit Points: (\d+)\nDamage: (\d+)");

    advtools::verify("Min mana for win", fight(hp, dmg, 0), 1269);
    advtools::verify("Min mana for win with 1 dmg/turn", fight(hp, dmg, 1), 1309);
}
