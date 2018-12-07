use advtools::prelude::Itertools;
use advtools::input::iter_input;

fn possible(s: (u32, u32, u32)) -> u32 {
    (s.0 + s.1 > s.2 && s.1 + s.2 > s.0 && s.2 + s.0 > s.1) as u32
}

fn main() {
    let mut num_row = 0;
    let mut num_col = 0;
    for (a, b, c) in iter_input::<(u32, u32, u32)>().tuples() {
        num_row += possible(a) + possible(b) + possible(c);
        num_col += possible((a.0, b.0, c.0)) +
            possible((a.1, b.1, c.1)) +
            possible((a.2, b.2, c.2));
    }
    println!("Possible triangles (by row): {}", num_row);
    println!("Possible triangles (by column): {}", num_col);
}
