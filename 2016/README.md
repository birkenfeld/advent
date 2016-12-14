## Advent of Code 2016

These are Rust-language solutions for
the [coding-challenge advent calendar](http://adventofcode.com/2016).  You'll
need Rust (stable should be fine) and Cargo to run.

Every day is a separate crate; run `cargo run --release` in the subdirectories.
There is also a toplevel make file; run `make build` or `make`.

I've tried to make the solutions small and somewhat optimized for speed (so far,
no solution takes more than about a second on an up-to-date machine).  Inputs
that are larger than a few lines are included in text file form and parsed.

There are a few external crates commonly used as dependencies, such as regex,
itertools, rayon.  A custom helper library is included in `advtools`, mostly for
easily parsing the input files.
