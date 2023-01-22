use advtools::input;
use advtools::prelude::HashMap;
use advtools::petgraph::prelude::{*, NodeIndex as Ix};

const RX: &str = r"Valve (..) has flow rate=(\d+); .* valves? (.*)";

fn run<const SWITCH: bool, const MAX_MINUTES: u32>(valves: &HashMap<Ix, (u32, HashMap<Ix, u32>)>,
                                                   start: Ix) -> u32 {
    let mut max_released = 0;
    let mut best_seq = HashMap::with_capacity(100000);
    // State entries: "switched" to elephant, current position, minutes taken,
    // total released pressure with currently open valves, open valves (bitmask).
    let mut queue = vec![(false, start, 0, 0, 1u64 << start.index())];

    while !queue.is_empty() {
        for (switched, pos, minutes, released, open) in std::mem::take(&mut queue) {
            // For each unopened next valve, try to go there and open it.
            for (&new_pos, (flow, costs)) in valves {
                if open & (1 << new_pos.index()) != 0 {
                    continue;
                }
                // Check if this valve can be reached and opened within allotted time.
                let new_minutes = minutes + costs[&pos] + 1;
                if new_minutes >= MAX_MINUTES {
                    continue;
                }
                // "Released" is the total released pressure over the whole
                // 30 minutes, so for each valve we add what it releases
                // from opening to end.
                let new_open = open | (1 << new_pos.index());
                let new_released = released + (MAX_MINUTES - new_minutes) * flow;
                // Check if this is actually a new "best order" to reach this valve
                // with the same other valves already open.
                if best_seq.get(&(new_pos, new_open)).map_or(true, |&v| v < new_released) {
                    // If yes, record it as such and go on from here.
                    best_seq.insert((new_pos, new_open), new_released);
                    queue.push((switched, new_pos, new_minutes, new_released, new_open));
                    max_released = max_released.max(new_released);
                }
            }
            // In part 2, as another possible step, include switching to the
            // elephant's point of view after opening at least one valve.
            if SWITCH && !switched && open > 0 {
                queue.push((true, start, 0, released, open));
            }
        }
    }
    max_released
}

fn main() {
    let mut graph = Graph::<_, (), _>::new_undirected();
    let mut node_ids = HashMap::new();
    let mut valves = HashMap::new();
    let mut edges = Vec::new();
    // Create a graph from the input.
    for (from, flow, tos) in input::rx_lines::<(&str, u32, &str)>(RX) {
        node_ids.insert(from, graph.add_node(flow));
        if flow != 0 || from == "AA" {
            // Collect the "useful" nodes (start or those with a flow rate).
            valves.insert(node_ids[&from], (flow, HashMap::new()));
        }
        edges.extend(tos.split(", ").map(|to| (from, to)));
    }
    graph.extend_with_edges(edges.into_iter().map(|(a, b)| (node_ids[a], node_ids[b])));
    let start = node_ids["AA"];

    // Calculate the shortest paths between all "useful" nodes.
    for (&n1, (_, costs)) in &mut valves {
        costs.extend(advtools::petgraph::algo::dijkstra(&graph, n1, None, |_| 1));
    }

    // Part 1: BFS search for optimal path visiting valves to open.
    let max_released = run::<false, 30>(&valves, start);
    advtools::verify("Max release", max_released, 2181);

    // Part 2: same but with elephant: at some point we add a "switch" where the
    // player stops acting and the elephant starts its "parallel" run.
    let max_released = run::<true, 26>(&valves, start);
    advtools::verify("Max release with elephant", max_released, 2824);
}
