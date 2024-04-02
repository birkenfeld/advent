use advtools::input;
use advtools::grid::{Grid, Pos};
use advtools::prelude::{Itertools, HashSet};

fn main() {
    input::set("\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
");
    let mut start = Pos(0, 0);
    let blocks = input::rx_lines(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").map(
        |(x1, y1, z1, x2, y2, z2): (i32, i32, i32, i32, i32, i32)| {
            if x1 != x2 {
                
            } else if y1 != y2 {
                
            } else {
                
            }
        }
    ).collect_vec();

    // advtools::verify("Reachable positions", visit(&grid, start, 64), 3762);
}
