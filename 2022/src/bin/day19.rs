use advtools::input;
use advtools::prelude::{Itertools, HashMap};

const RX: &str = r".* (\d+) ore.* (\d+) ore.* (\d+) ore.* (\d+) clay.* (\d+) ore.* (\d+)";

/// Any quantity which is present for all four resource types.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Amounts {
    ore: i16,
    clay: i16,
    obs: i16,
    geode: i16,
}

impl Amounts {
    fn change_ore(&self, delta: i16) -> Self {
        Amounts { ore: self.ore + delta, ..*self }
    }
    fn change_clay(&self, delta: i16) -> Self {
        Amounts { clay: self.clay + delta, ..*self }
    }
    fn change_obs(&self, delta: i16) -> Self {
        Amounts { obs: self.obs + delta, ..*self }
    }
    fn change_geode(&self, delta: i16) -> Self {
        Amounts { geode: self.geode + delta, ..*self }
    }
}

#[derive(Clone, Copy)]
struct Blueprint {
    ore_ore: i16,
    clay_ore: i16,
    obs_ore: i16,
    obs_clay: i16,
    geode_ore: i16,
    geode_obs: i16,
    max: Amounts,
}

const INIT_MATERIALS: Amounts = Amounts { ore: 0, clay: 0, obs: 0, geode: 0 };
const INIT_ROBOTS: Amounts = Amounts { ore: 1, clay: 0, obs: 0, geode: 0 };

impl Blueprint {
    fn new(costs: [i16; 6]) -> Self {
        // Maximum number of useful robots for each resource is given by the
        // maximum cost (of the different robot types) of that resource.
        let max = Amounts {
            ore: costs[0].max(costs[1]).max(costs[2]).max(costs[4]),
            clay: costs[3],
            obs: costs[5],
            geode: 0,
        };
        Blueprint {
            ore_ore: costs[0],
            clay_ore: costs[1],
            obs_ore: costs[2],
            obs_clay: costs[3],
            geode_ore: costs[4],
            geode_obs: costs[5],
            max
        }
    }

    fn run(&self, limit: i16) -> i16 {
        let mut best_geodes = 0;
        let mut seen = HashMap::new();
        dfs(limit, self, &mut best_geodes, &mut seen, 0, INIT_MATERIALS, INIT_ROBOTS);
        best_geodes
    }
}

fn dfs(limit: i16, bp: &Blueprint, best: &mut i16, seen: &mut HashMap<(Amounts, Amounts), i16>,
       step: i16, materials: Amounts, robots: Amounts) {
    // Check if time is up.
    if step == limit {
        return;
    }

    // Check if the current state has been reached before in the same or better time.
    if seen.insert((materials, robots), step).unwrap_or(limit) <= step {
        return;
    }

    // Check if the maximum geodes we can make even hits the current best.
    let rest = limit - step;
    let max_geodes = materials.geode + rest*robots.geode + rest*(rest - 1) / 2;
    if max_geodes <= *best {
        return;
    }

    // Accumulate new materials.
    let new_materials = Amounts {
        ore: materials.ore + robots.ore,
        clay: materials.clay + robots.clay,
        obs: materials.obs + robots.obs,
        geode: materials.geode + robots.geode,
    };

    *best = new_materials.geode.max(*best);

    // Build one of the different robot kinds if possible.
    if materials.ore >= bp.ore_ore && robots.ore < bp.max.ore {
        dfs(limit, bp, best, seen, step + 1,
            new_materials.change_ore(-bp.ore_ore), robots.change_ore(1));
    }

    if materials.ore >= bp.clay_ore && robots.clay < bp.max.clay {
        dfs(limit, bp, best, seen, step + 1,
            new_materials.change_ore(-bp.clay_ore), robots.change_clay(1));
    }

    if materials.ore >= bp.obs_ore && materials.clay >= bp.obs_clay && robots.obs < bp.max.obs {
        dfs(limit, bp, best, seen, step + 1,
            new_materials.change_ore(-bp.obs_ore).change_clay(-bp.obs_clay),
            robots.change_obs(1));
    }

    if materials.ore >= bp.geode_ore && materials.obs >= bp.geode_obs {
        dfs(limit, bp, best, seen, step + 1,
            new_materials.change_ore(-bp.geode_ore).change_obs(-bp.geode_obs),
            robots.change_geode(1));
    } else {
        // Try doing nothing, but only if we can't build a geode robot.
        dfs(limit, bp, best, seen, step + 1, new_materials, robots);
    }
}

fn main() {
    let blueprints = input::rx_lines::<[i16; 6]>(RX).map(Blueprint::new).collect_vec();

    // Part 1: collect quality levels for all blueprints at 24 steps.
    let quality = blueprints.iter().enumerate().map(|(i, bp)| {
        bp.run(24) * (i as i16 + 1)
    }).sum::<i16>();
    advtools::verify("Quality level", quality, 1262);

    // Part 2: collect first three blueprints at 32 steps.
    let product = blueprints.iter().take(3).map(|bp| bp.run(32) as i32).product::<i32>();
    advtools::verify("Best geode product", product, 37191);
}
