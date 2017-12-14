extern crate advtools;
extern crate petgraph;

use advtools::prelude::*;
use petgraph::prelude::*;

fn main() {
    let mut graph = Graph::new_undirected();
    let input = iter_input_trim::<(usize, (), Vec<usize>)>(",").collect_vec();
    for _ in 0..input.len() {
        graph.add_node(0);
    }
    for (from, _, tos) in input {
        for to in tos {
            graph.add_edge(NodeIndex::new(from), NodeIndex::new(to), 0);
        }
    }
    let components = petgraph::algo::kosaraju_scc(&graph);
    for comp in &components {
        if comp.contains(&NodeIndex::new(0)) {
            println!("Programs talking to 0: {}", comp.len());
            break;
        }
    }
    println!("Number of groups: {}", components.len());
}
