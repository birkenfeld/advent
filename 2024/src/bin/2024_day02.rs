use advtools::input;
use advtools::prelude::Itertools;

fn safe(report: &[i32], skip: Option<usize>) -> bool {
    let (min, max) = report.iter()
                           .enumerate()
                           .filter(|&(i, _)| Some(i) != skip)
                           .tuple_windows()
                           .map(|((_, a), (_, b))| b - a)
                           .minmax()
                           .into_option().unwrap();
    (min >= -3 && max <= -1) || (min >= 1 && max <= 3)
}

fn main() {
    let mut ok = 0;
    let mut dampened = 0;
    for report in input::parse_lines::<Vec<i32>>() {
        if safe(&report, None) {
            ok += 1;
        } else if (0..report.len()).any(|i| safe(&report, Some(i))) {
            dampened += 1;
        }
    }
    advtools::verify("Safe reports", ok, 257);
    advtools::verify("Safe reports with dampener", ok + dampened, 328);
}
