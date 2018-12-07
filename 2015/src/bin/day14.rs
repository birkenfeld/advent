use advtools::input::iter_input_parts;

const INPUT: u32 = 2503;

fn main() {
    let mut deer = Vec::new();
    for row in iter_input_parts((0, 3, 6, 13)) {
        let (name, speed, fly_time, rest_time): (String, u32, u32, u32) = row;
        deer.push((name, speed, fly_time, fly_time + rest_time, 0, 0));
    }

    let winner = deer.iter().map(|(name, speed, fly_time, cycle_time, _, _)| {
        let cycles = INPUT / cycle_time;
        let rest_time = INPUT % cycle_time;
        (name.clone(), speed * (cycles * fly_time + rest_time.min(*fly_time)))
    }).max_by_key(|v| v.1).unwrap();
    println!("Traditional: {} ({} km)", winner.0, winner.1);

    for time in 0..INPUT {
        let best = deer.iter_mut().map(|&mut (_, speed, fly_time, cycle_time, ref mut dist, _)| {
            if time % cycle_time < fly_time {
                *dist += speed;
            }
            *dist
        }).max().unwrap();
        for (_, _, _, _, dist, points) in &mut deer {
            if *dist == best {
                *points += 1;
            }
        }
    }
    let winner = deer.iter().max_by_key(|v| v.5).unwrap();
    println!("New-style: {} ({} points)", winner.0, winner.5);
}
