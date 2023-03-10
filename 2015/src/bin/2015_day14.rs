use advtools::input;

const INPUT: u32 = 2503;

fn main() {
    let mut deer = Vec::new();
    for row in input::rx_lines(r"(.*) can fly (\d+) km/s for (\d+) .* for (\d+) .*") {
        let (name, speed, fly_time, rest_time): (&str, u32, u32, u32) = row;
        deer.push((name, speed, fly_time, fly_time + rest_time, 0, 0));
    }

    let winner = deer.iter().map(|(name, speed, fly_time, cycle_time, _, _)| {
        let cycles = INPUT / cycle_time;
        let rest_time = INPUT % cycle_time;
        (name, speed * (cycles * fly_time + rest_time.min(*fly_time)))
    }).max_by_key(|v| v.1).unwrap();
    advtools::verify("Traditional", winner.1, 2640);

    for time in 0..INPUT {
        let best = deer.iter_mut().map(|(_, speed, fly_time, cycle_time, dist, _)| {
            if time % *cycle_time < *fly_time {
                *dist += *speed;
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
    advtools::verify("New-style", winner.5, 1102);
}
