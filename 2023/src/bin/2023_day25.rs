use advtools::input;
use advtools::prelude::HashSet;

fn main() {
    input::set("jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr");

    for line in input::rx_lines(r"(\w+): (.+)") {
        let (name, deps): (&str, &str) = line;
        let deps = deps.split(' ').collect::<HashSet<_>>();
        println!("{}: {:?}", name, deps);
    }
}
