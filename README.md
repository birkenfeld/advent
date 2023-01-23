## 🎄 Advent of Code 2015-2022 🎄

These are Rust-language solutions for the [coding-challenge advent
calendar](http://adventofcode.com/).  You'll need stable Rust 1.65 and Cargo to
run.

The code for the different years is in the respective subdirectories.

I've tried to make the solutions small and somewhat optimized for speed (so far,
no solution takes more than about a second on an up-to-date machine).  Inputs
are included in text file form and parsed.

### External code used

A custom helper library is used, called `advtools`.  It provides utilities for
easily parsing the input files, which I don't want to rewrite each year, and
access to often used external crates like itertools and rayon.

For tasks that require nontrivial datastructures or algorithms, I try to find
and use a third-party crate to show off the ease of using Rust's crates
infrastructures, e.g. `petgraph`.

### Building

All code is contained in a single Cargo workspace, with a different binary
target for each day.  Solutions are printed to stdout.

A simple Makefile is also provided in order to run all years and/or days.  Just
run `make` to run everything, or e.g. `make 2022` to run only one year.

### `advtools`

This subdirectory contains a library I've developed over time to ease
boilerplate and keep utilities for common tasks, such as manipulation of things
on grids.  It is also released under that name to `crates.io`.
