use advtools::prelude::HashMap;
use advtools::input;
use advtools::petgraph::prelude::*;

const FORMAT: &str = r"([a-z]+) \((\d+)\)(?: -> (.+))?";

fn main() {
    let mut name2ix = HashMap::new();
    let mut graph = Graph::new();
    // Parse input into a directed graph.  The node weight is the weight of the program.
    // The edge weights are later used for cumulative weights of program + children.
    // `name2ix` maps program names to graph Index values.
    for (name, weight, children) in input::rx_lines::<(&str, i32, input::Csv<&str>)>(FORMAT) {
        // Since some nodes are mentioned first as children and some as parents,
        // we must check if we need to insert them as a new node.
        let ix = *name2ix.entry(name).or_insert_with(|| graph.add_node(0));
        graph[ix] = weight;
        // Add edges to children (and the child nodes if necessary).  Weight 0 is used
        // until we know the program's weight from its own entry.
        for childname in children.vec {
            let cix = *name2ix.entry(childname).or_insert_with(|| graph.add_node(0));
            graph.add_edge(ix, cix, 0i32);
        }
    }
    // Part 1: The graph root (the node with no incoming edge) is the bottom program.
    // We assume there is exactly one.
    let root = graph.externals(Incoming).next().unwrap();
    advtools::verify("Bottom program", name2ix.iter().find(|e| e.1 == &root).unwrap().0, "hmvwl");

    // Part 2: Walk the graph using DFS to find the program whose children are not
    // balanced.
    let mut dfs = DfsPostOrder::new(&graph, root);
    let mut req_weight = 0;
    while let Some(src_ix) = dfs.next(&graph) {
        let mut weights = HashMap::new();
        let mut walker = graph.neighbors(src_ix).detach();
        while let Some((edge_ix, tgt_ix)) = walker.next(&graph) {
            // Find cumulative weights of all children of this node, and assign them
            // to the edge weights.
            let edge_wt = graph[tgt_ix] + graph.edges(tgt_ix).map(|e| e.weight()).sum::<i32>();
            // Count the number of children for each cumulative weight.  In the
            // normal case, all should be equal.
            *weights.entry(edge_wt).or_insert(0) += 1;
            graph[edge_ix] = edge_wt;
        }
        // We've collected all weights in the map.  If the nodes are unbalanced,
        // there is one entry with count 1.
        if let Some((offending_weight, _)) = weights.into_iter().find(|&(_, c)| c == 1) {
            // Find the nodes with normal weight too.
            let (offending, normal): (Vec<_>, Vec<_>) =
                graph.edges(src_ix).partition(|ex| *ex.weight() == offending_weight);
            // Calculate required weight using the difference between offending
            // and normal cumulative weight.
            req_weight = graph[normal[0].id()] - graph[offending[0].id()] + graph[offending[0].target()];
            break;
        }
    }
    advtools::verify("Required weight", req_weight, 1853);
}
