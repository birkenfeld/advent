extern crate advtools;
extern crate petgraph;
use advtools::prelude::Itertools;
use advtools::input::iter_input_trim;
use petgraph::prelude::*;

fn main() {
    let mut graph = Graph::new_undirected();
    let input = iter_input_trim::<(usize, (), Vec<usize>)>(",").collect_vec();
    // Collect input into a graph.  We don't have any useful weights to assign.
    // This will add all nodes with indices from 0 to len-1.
    for _ in 0..input.len() {
        graph.add_node(0);
    }
    // Add the edges from the input.
    for (from, _, tos) in input {
        for to in tos {
            graph.add_edge(NodeIndex::new(from), NodeIndex::new(to), 0);
        }
    }
    // Find strongly connected components, which correspond to "groups".
    let components = petgraph::algo::kosaraju_scc(&graph);
    // Part 1: Find group with the program 0.
    for comp in &components {
        if comp.contains(&NodeIndex::new(0)) {
            println!("Programs talking to 0: {}", comp.len());
            break;
        }
    }
    // Part 2: Just the number of components.
    println!("Number of groups: {}", components.len());
}
