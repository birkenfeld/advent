use advtools::prelude::{Itertools, HashMap, HashSet};
use advtools::input::iter_lines;

fn main() {
    // Assign integer indices to cave names, and keep track of which ones are
    // small (lowercase names).
    let mut indices = HashMap::new();
    let mut small = HashSet::new();
    let mut n_ix = 0;
    let mut add_node = |s: &str| {
        let index = *indices.entry(s.to_string()).or_insert_with(|| { n_ix += 1; n_ix });
        if s.chars().next().unwrap().is_ascii_lowercase() { small.insert(index); }
        index
    };

    // Parse the edges between caves.
    let mut edges = HashMap::<i32, HashSet<i32>>::new();
    for line in iter_lines() {
        let (a, b) = line.split('-').collect_tuple().unwrap();
        let ix_a = add_node(a);
        let ix_b = add_node(b);
        edges.entry(ix_a).or_default().insert(ix_b);
        edges.entry(ix_b).or_default().insert(ix_a);
    }

    let start = indices["start"];
    let end = indices["end"];
    // Do not allow edges leading back to start.
    edges.iter_mut().for_each(|(_, v)| { v.take(&start); });

    let paths = cave_dfs(&edges, &small, start, end, &mut HashSet::new(), true);
    advtools::verify("Small caves once", paths, 3887);

    let paths = cave_dfs(&edges, &small, start, end, &mut HashSet::new(), false);
    advtools::verify("One small cave twice", paths, 104834);
}

// Depth-first search through cave paths.
fn cave_dfs(edges: &HashMap<i32, HashSet<i32>>, small: &HashSet<i32>, from: i32, goal: i32,
            visited: &mut HashSet<i32>, visited_twice: bool) -> usize {
    let mut paths = 0;
    // Go through the caves connected to the current one.
    for &next in &edges[&from] {
        if next == goal {
            // If the next cave is the end, we have found one path.
            paths += 1;
        } else if small.contains(&next) && visited.contains(&next) {
            // If we visit a small cave the second time, and we already visited
            // another one twice, ignore this.
            if visited_twice {
                continue;
            }
            // Otherwise continue, but set the "visited twice" flag.
            paths += cave_dfs(edges, small, next, goal, visited, true);
        } else {
            // Visiting a big cave, or a small one the first time.
            visited.insert(next);
            paths += cave_dfs(edges, small, next, goal, visited, visited_twice);
            visited.take(&next);
        }
    }
    paths
}
