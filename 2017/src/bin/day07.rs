extern crate advtools;

use std::collections::HashMap;
use std::mem;

struct Tree<'a> {
    childmap: HashMap<&'a str, Vec<&'a str>>,
    weights: HashMap<&'a str, i32>,
    totals: HashMap<&'a str, i32>,
}

impl<'a> Tree<'a> {
    fn child_weight(&self, n: &str) -> i32 {
        self.childmap[n].iter().map(|v| self.weight(v)).sum::<i32>()
    }

    fn weight(&self, n: &str) -> i32 {
        self.weights[n] + self.child_weight(n)
    }

    fn find_root(&'a self, node: &'a str) -> &'a str {
        for (parent, children) in &self.childmap {
            if children.iter().any(|v| *v == node) {
                return self.find_root(parent);
            }
        }
        node
    }

    fn check_children(&self, node: &str) -> Option<i32> {
        if self.childmap[node].is_empty() { return None; }
        let mut odd = None;
        let mut last = self.totals[self.childmap[node][0]];
        for (child, i) in self.childmap[node][1..].iter().zip(1..) {
            let el = self.totals[child];
            if el != last {
                if odd.is_none() {
                    odd = Some(((el, i), (last, i-1)));
                }
            } else if let Some((ref mut odd, ref mut reg)) = odd {
                if odd.0 != el {
                    mem::swap(odd, reg);
                }
            }
            last = el;
        }
        odd.map(|((_, odd_idx), (normal, _))| normal - self.child_weight(self.childmap[node][odd_idx]))
    }

    fn find_required_weight(&self, root: &str) -> Option<i32> {
        self.childmap[root].iter()
                           .flat_map(|ch| self.find_required_weight(ch))
                           .next()
                           .or_else(|| self.check_children(root))
    }
}

fn main() {
    let input = advtools::iter_input::<String>().collect::<Vec<_>>();
    let mut childmap = HashMap::new();
    let mut weights = HashMap::new();
    for line in &input {
        let mut split = line.split_whitespace();
        let name = split.next().unwrap();
        let weight = split.next().unwrap().trim_matches(&['(', ')'][..]).parse::<i32>().unwrap();
        let children = split.skip(1).map(|v| v.trim_matches(',')).collect::<Vec<_>>();
        weights.insert(name, weight);
        childmap.insert(name, children);
    }
    let mut tree = Tree { childmap, weights, totals: HashMap::new() };
    let mut totals = HashMap::new();
    for &node in tree.childmap.keys() {
        totals.insert(node, tree.weight(node));
    }
    tree.totals = totals;
    let root = tree.find_root(tree.weights.keys().next().unwrap());
    println!("Bottom program: {}", root);
    println!("Required weight: {}", tree.find_required_weight(root).unwrap());
}
