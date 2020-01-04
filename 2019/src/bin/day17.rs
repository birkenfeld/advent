use advtools::prelude::{Itertools, HashSet};
use advtools::input::input_string;
use advent19::{Machine, Dir};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Instr { Fwd(usize), L, R, A, B, C }

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<bool>,
}

impl Grid {
    fn neighbor(&self, xy: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
        dir.maybe_step(xy, self.width, self.height)
           .filter(|(x, y)| self.grid[y*self.width + x])
    }
}

fn seq_len(seq: &[Instr]) -> usize {
    seq.iter().map(|instr| match instr {
        Instr::Fwd(n) if *n >= 10 => 2,
        _                         => 1,
    }).sum::<usize>() + seq.len() - 1  // add the commas
}

fn seq_fmt(seq: &[Instr]) -> String {
    seq.iter().map(|i| match i {
        Instr::Fwd(n) => n.to_string(),
        x => format!("{:?}", x)
    }).join(",") + "\n"
}


fn main() {
    let code = Machine::parse(&input_string());
    let mut width = usize::max_value();
    let mut robot_pos = (0, 0);
    let mut robot_dir = Dir::U;
    let grid = Machine::new(&code).enumerate().filter_map(|(i, ch)| match ch as u8 {
        b'#'  => Some(true),
        b'.'  => Some(false),
        b'\n' => { width = width.min(i); None }
        b'^'  => { robot_pos = (i % (width+1), i / (width+1)); robot_dir = Dir::U; Some(true) }
        b'v'  => { robot_pos = (i % (width+1), i / (width+1)); robot_dir = Dir::D; Some(true) }
        b'<'  => { robot_pos = (i % (width+1), i / (width+1)); robot_dir = Dir::L; Some(true) }
        b'>'  => { robot_pos = (i % (width+1), i / (width+1)); robot_dir = Dir::R; Some(true) }
        _ => panic!("invalid char in machine output")
    }).collect_vec();
    let grid = Grid { width, height: grid.len()/width, grid };

    // Part 1: determine the sum of "alignment parameters" which are intersections
    // in the scaffold, i.e. all neighbors are present.
    let alignment = (0..grid.width).cartesian_product(0..grid.height).filter(|&xy| {
        Dir::iter().all(|d| grid.neighbor(xy, d).is_some())
    }).map(|(x, y)| x * y).sum::<usize>();
    advtools::verify("Alignment param sum", alignment, 5940);

    let mut machine = Machine::new(&code);
    machine.set_mem(0, 2);

    // Determine the initial series of instructions without functions.
    let mut instrs = Vec::new();
    loop {
        if let Some(new) = grid.neighbor(robot_pos, robot_dir) {
            match instrs.last_mut() {
                Some(Instr::Fwd(n)) => *n += 1,
                _ => instrs.push(Instr::Fwd(1)),
            }
            robot_pos = new;
        } else if grid.neighbor(robot_pos, robot_dir.left()).is_some() {
            instrs.push(Instr::L);
            robot_dir = robot_dir.left();
        } else if grid.neighbor(robot_pos, robot_dir.right()).is_some() {
            instrs.push(Instr::R);
            robot_dir = robot_dir.right();
        } else {
            break;
        }
    }

    // Determine possible substrings with useful lengths (max is 10).
    let mut substr = HashSet::new();
    for &n in &[6, 8, 10] {
        for i in (0..instrs.len()-n).step_by(2) {
            if seq_len(&instrs[i..i+n]) <= 20 {
                substr.insert(&instrs[i..i+n]);
            }
        }
    }

    // Try every combination of substrings as A, B, C and check if any of them
    // leads to the main becoming at most 10 items long.
    let n = instrs.len();
    let mut main = Vec::new();  // Always work in the same allocation.
    'outer: for ((a, b), c) in substr.iter().cartesian_product(&substr).cartesian_product(&substr) {
        main.clear();
        let mut i = 0;
        while i < n {
            if instrs.get(i..i + a.len()) == Some(*a) {
                main.push(Instr::A);
                i += a.len();
            } else if instrs.get(i..i + b.len()) == Some(*b) {
                main.push(Instr::B);
                i += b.len();
            } else if instrs.get(i..i + c.len()) == Some(*c) {
                main.push(Instr::C);
                i += c.len();
            } else {
                main.push(instrs[i]);
                i += 1;
            }
            if main.len() > 10 {
                continue 'outer;
            }
        }
        // We found something with max. 10 elements, still need to check the
        // overall length when formatted.
        if seq_len(&main) <= 20 {
            for seq in &[&*main, a, b, c] {
                machine = machine.with_input_str(&seq_fmt(seq));
            }
            advtools::verify("Dust collected",
                             machine.with_input_str("n\n").last().unwrap(), 923795);
            return;
        }
    }
}
