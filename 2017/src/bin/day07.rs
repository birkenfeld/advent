extern crate advtools;
extern crate petgraph;

use advtools::prelude::*;
use petgraph::prelude::*;

fn main() {
    let mut name2ix = HashMap::new();
    let mut graph = Graph::new();
    for (name, weight, children) in iter_input_trim::<(String, i32, Vec<String>)>("(),") {
        let ix = match name2ix.entry(name) {
            Entry::Occupied(e) => { graph[*e.get()] = weight; *e.get() },
            Entry::Vacant(e) =>   { let ix = graph.add_node(weight); *e.insert(ix) },
        };
        for childname in children.into_iter().skip(1) {
            let cix = *name2ix.entry(childname).or_insert_with(|| graph.add_node(0));
            graph.add_edge(ix, cix, 0i32);
        }
    }
    let root = graph.externals(Incoming).item();
    println!("Bottom program: {}", name2ix.iter().find(|e| e.1 == &root).unwrap().0);

    let mut dfs = DfsPostOrder::new(&graph, root);
    let mut req_weight = 0;
    while let Some(src_ix) = dfs.next(&graph) {
        let mut weights = HashMap::new();
        let mut walker = graph.neighbors(src_ix).detach();
        while let Some((edge_ix, tgt_ix)) = walker.next(&graph) {
            let edge_wt = graph[tgt_ix] + graph.edges(tgt_ix).map(|e| e.weight()).sum::<i32>();
            *weights.entry(edge_wt).or_insert(0) += 1;
            graph[edge_ix] = edge_wt;
        }
        if let Some((offending_weight, _)) = weights.into_iter().find(|&(_, c)| c == 1) {
            let (offending, normal): (Vec<_>, Vec<_>) =
                graph.edges(src_ix).partition(|ex| *ex.weight() == offending_weight);
            req_weight = graph[normal[0].id()] - graph[offending[0].id()] + graph[offending[0].target()];
            break;
        }
    }
    println!("Required weight: {}", req_weight);
}
