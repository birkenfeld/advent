use advtools::input;
use advtools::prelude::HashMap;
use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

fn main() {
    let mut graph = UnGraph::new_undirected();
    let mut nodes = HashMap::new();

    for line in input::rx_lines(r"(\w+): (.+)") {
        let (first, others): (&str, &str) = line;
        let first = *nodes.entry(first).or_insert_with(|| graph.add_node(()));
        for other in others.split(' ') {
            let other = *nodes.entry(other).or_insert_with(|| graph.add_node(()));
            graph.add_edge(first, other, ());
        }
    }

    let (n, cut) = stoer_wagner_min_cut(&graph, |_| Ok::<_, ()>(1)).unwrap().unwrap();
    assert_eq!(n, 3);

    advtools::verify("Product of remains", cut.len() * (nodes.len() - cut.len()), 600225);
}
