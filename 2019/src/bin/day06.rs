use advtools::prelude::HashMap;
use advtools::input::iter_input_regex;

fn main() {
    let mut graph = petgraph::Graph::new_undirected();
    let mut nodes = HashMap::new();

    // Create the graph.
    for (a, b) in iter_input_regex::<(String, String)>(r"(\w+)\)(\w+)") {
        let nodea = *nodes.entry(a).or_insert_with(|| graph.add_node(0));
        let nodeb = *nodes.entry(b).or_insert_with(|| graph.add_node(0));
        graph.add_edge(nodea, nodeb, 1.0);
    }

    // Part 1: sum length of paths from root to all nodes.
    let (weights, _) = petgraph::algo::bellman_ford(&graph, nodes["COM"]).unwrap();
    advtools::print("First round", weights.into_iter().sum::<f64>());

    // Part 2: find length of path from YOU to SAN.
    let map = petgraph::algo::dijkstra(&graph, nodes["YOU"],
                                       Some(nodes["SAN"]), |e| *e.weight());
    advtools::print("Second round", map[&nodes["SAN"]] - 2.0);
}
