use advtools::input;
use advtools::grid::{Grid, Pos, Dir};
use advtools::petgraph::visit::EdgeRef;
use advtools::petgraph::graph::{DiGraph, NodeIndex};
use advtools::prelude::{HashSet, HashMap};


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Forest,
    Path,
    Slope(Dir),
}

/// Run a DFS through the maze to find all nodes (tiles with more than two
/// neighbors) and paths between them.
fn convert_inner(grid: &Grid<Tile>, nodepos: Pos, curpos: Pos, len: u32, fwd: bool, back: bool,
                 nodes: &mut HashMap<Pos, NodeIndex>, graph: &mut DiGraph<(), u32>,
                 visited: &mut HashSet<Pos>, consider_slopes: bool) {
    for nbpos in grid.neighbors(curpos) {
        match grid[nbpos] {
            Tile::Forest => continue,
            Tile::Path => {
                if nodes.contains_key(&nbpos) || grid.neighbors(nbpos).filter(|&p| !matches!(grid[p], Tile::Forest)).count() > 2 {
                    if nbpos == nodepos {
                        continue;
                    }
                    // found a node!
                    if !nodes.contains_key(&nbpos) {
                        let node = graph.add_node(());
                        nodes.insert(nbpos, node);
                    }
                    if fwd {
                        graph.add_edge(nodes[&nodepos], nodes[&nbpos], len + 1);
                    }
                    if back {
                        graph.add_edge(nodes[&nbpos], nodes[&nodepos], len + 1);
                    }
                    convert_inner(grid, nbpos, nbpos, 0, true, true, nodes, graph, visited, consider_slopes)
                } else {
                    if !visited.insert(nbpos) {
                        continue;
                    }
                    convert_inner(grid, nodepos, nbpos, len + 1, fwd, back, nodes, graph, visited, consider_slopes);
                }
            }
            Tile::Slope(slopedir) => {
                if !visited.insert(nbpos) {
                    continue;
                }
                let fwd = !consider_slopes || (fwd && slopedir == nbpos.dir_from(curpos));
                let back = !consider_slopes || (back && slopedir == curpos.dir_from(nbpos));
                convert_inner(grid, nodepos, nbpos, len + 1, fwd, back, nodes, graph, visited, consider_slopes);
            }
        }
    }
}

fn convert(grid: &Grid<Tile>, start: Pos, target: Pos, consider_slopes: bool) -> (DiGraph<(), u32>, NodeIndex, NodeIndex) {
    let mut graph = DiGraph::new();
    let mut nodes = HashMap::new();
    let mut visited = HashSet::new();
    visited.insert(start);
    visited.insert(target);
    nodes.insert(start, graph.add_node(()));
    nodes.insert(target, graph.add_node(()));
    convert_inner(grid, start, start, 0, true, true, &mut nodes, &mut graph, &mut visited, consider_slopes);
    (graph, nodes[&start], nodes[&target])
}

/// Run a DFS through the graph to find the longest path.
fn longest_path(graph: &DiGraph<(), u32>, pos: NodeIndex, target: NodeIndex, steps: u32,
                longest: &mut u32, seen: &mut HashSet<NodeIndex>) {
    seen.insert(pos);
    for edge in graph.edges(pos) {
        let nbpos = edge.target();
        if seen.contains(&nbpos) {
            continue;
        }
        if nbpos == target {
            *longest = (*longest).max(steps + edge.weight());
        } else {
            longest_path(graph, nbpos, target, steps + edge.weight(), longest, seen);
        }
    }
    seen.remove(&pos);
}

fn main() {
    let grid = Grid::new(input::lines().map(|line| line.chars().map(|ch| match ch {
        '#' => Tile::Forest,
        '.' => Tile::Path,
        sl  => Tile::Slope(Dir::from_char(sl)),
    })));
    let start = Pos(1, 0);
    let target = Pos(grid.width() as i32 - 2, grid.height() as i32 - 1);

    let (graph, from, to) = convert(&grid, start, target, true);
    let mut longest = 0;
    longest_path(&graph, from, to, 0, &mut longest, &mut HashSet::new());
    advtools::verify("Longest hike", longest, 2402);

    let (graph, from, to) = convert(&grid, start, target, false);
    longest_path(&graph, from, to, 0, &mut longest, &mut HashSet::new());
    advtools::verify("Longest hike without slopes", longest, 6450);
}
