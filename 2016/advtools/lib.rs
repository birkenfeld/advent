extern crate itertools;
extern crate regex;
extern crate fnv;

use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufRead, Read};
use std::marker::PhantomData;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;

pub mod prelude {
    pub use std::collections::{HashMap, HashSet};
    pub use std::collections::hash_map::Entry;

    pub use itertools::Itertools;
    pub use regex::{Regex, Captures};
    pub use fnv::FnvHashMap;

    pub use super::IterExt;
    pub use super::iter_input;
    pub use super::input_file;
    pub use super::input_string;
    pub use super::parse_fields;
    pub use super::{to_u8, to_u32, to_usize, to_i32};
    pub use super::from_utf8;
}

pub type TokIter<'t> = std::str::SplitWhitespace<'t>;

pub trait Input where Self: Sized {
    fn read(line: String) -> Self {
        Self::read_token(&mut line.split_whitespace())
    }
    fn read_token(tok: &mut TokIter) -> Self;
}

impl Input for String {
    fn read(line: String) -> String {
        line
    }
    fn read_token(tok: &mut TokIter) -> String {
        tok.next().unwrap().to_owned()
    }
}

impl Input for Vec<String> {
    fn read(line: String) -> Vec<String> {
        line.split_whitespace().map(String::from).collect()
    }
    fn read_token(tok: &mut TokIter) -> Vec<String> {
        tok.next().unwrap().split_whitespace().map(String::from).collect()
    }
}

macro_rules! simple_impl {
    ($ty:ty) => {
        impl Input for $ty {
            fn read_token(tok: &mut TokIter) -> $ty {
                tok.next().unwrap().parse().unwrap()
            }
        }
    }
}

simple_impl!(u8);
simple_impl!(u16);
simple_impl!(u32);
simple_impl!(u64);
simple_impl!(i8);
simple_impl!(i16);
simple_impl!(i32);
simple_impl!(i64);

macro_rules! array_impl {
    ($n:expr, $tok1:ident, $($tok:ident),+) => {
        impl<T: Input> Input for [T; $n] {
            fn read_token($tok1: &mut TokIter) -> [T; $n] {
                [$(T::read_token($tok)),+]
            }
        }
    }
}

array_impl!(1, tok, tok);
array_impl!(2, tok, tok, tok);
array_impl!(3, tok, tok, tok, tok);
array_impl!(4, tok, tok, tok, tok, tok);
array_impl!(5, tok, tok, tok, tok, tok, tok);
array_impl!(6, tok, tok, tok, tok, tok, tok, tok);
array_impl!(7, tok, tok, tok, tok, tok, tok, tok, tok);
array_impl!(8, tok, tok, tok, tok, tok, tok, tok, tok, tok);

impl Input for char {
    fn read_token(tok: &mut TokIter) -> char {
        tok.next().unwrap().chars().next().unwrap()
    }
}

impl Input for () {
    fn read_token(tok: &mut TokIter) -> () {
        tok.next().unwrap();
        ()
    }
}

impl<T: Input> Input for (T,) {
    fn read_token(tok: &mut TokIter) -> (T,) {
        (T::read_token(tok),)
    }
}

macro_rules! tuple_impl {
    ($($tys:ident),+) => {
        impl<$($tys: Input),+> Input for ($($tys),+) {
            fn read_token(tok: &mut TokIter) -> ($($tys),+) {
                ($($tys::read_token(tok)),+)
            }
        }
    }
}

tuple_impl!(T, U);
tuple_impl!(T, U, V);
tuple_impl!(T, U, V, W);
tuple_impl!(T, U, V, W, Y);
tuple_impl!(T, U, V, W, Y, Z);
tuple_impl!(T, U, V, W, Y, Z, T1);
tuple_impl!(T, U, V, W, Y, Z, T1, T2);
tuple_impl!(T, U, V, W, Y, Z, T1, T2, T3);
tuple_impl!(T, U, V, W, Y, Z, T1, T2, T3, T4);
tuple_impl!(T, U, V, W, Y, Z, T1, T2, T3, T4, T5);
tuple_impl!(T, U, V, W, Y, Z, T1, T2, T3, T4, T5, T6);


pub struct InputIterator<I: Input> {
    rdr: BufReader<File>,
    marker: PhantomData<I>,
}

impl<I: Input> Iterator for InputIterator<I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        let mut line = String::new();
        while line.is_empty() {
            if self.rdr.read_line(&mut line).unwrap() == 0 {
                return None;
            }
            while line.trim_right() != line {
                line.pop();
            }
        }
        Some(I::read(line))
    }
}


pub fn input_file() -> File {
    let mut infile = Path::new("input").join(
        Path::new(&env::args_os().next().unwrap()).file_name().unwrap());
    infile.set_extension("txt");
    File::open(&infile)
        .unwrap_or_else(|_| panic!("input file \"{}\" not found", infile.display()))
}


pub fn iter_input<I: Input>() -> InputIterator<I> {
    let rdr = BufReader::new(input_file());
    InputIterator { rdr: rdr, marker: PhantomData }
}


pub fn input_string() -> String {
    let mut contents = String::new();
    input_file().read_to_string(&mut contents).unwrap();
    contents
}


pub trait ParseParts {
    type Indices;
    fn parse(indices: Self::Indices, line: &str) -> Self;
}

macro_rules! simple_parseparts_impl {
    ($ty:ty, $($tt:tt)*) => {
        impl ParseParts for $ty {
            type Indices = usize;
            fn parse(index: usize, line: &str) -> Self {
                line.split_whitespace().nth(index).unwrap().parse().unwrap_or_else(
                    |_| panic!("Couldn't parse line {:?}", line))
            }
        }
        simple_parseparts_impl!($($tt)*);
    };
    () => {};
}

simple_parseparts_impl!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, char,);

impl<T: FromStr, U: FromStr> ParseParts for (T, U) {
    type Indices = (usize, usize);
    fn parse((i1, i2): Self::Indices, line: &str) -> Self {
        let mut ws = line.split_whitespace();
        (ws.nth(i1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
         ws.nth(i2 - i1 - 1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)))
    }
}

impl<T: FromStr, U: FromStr, V: FromStr> ParseParts for (T, U, V) {
    type Indices = (usize, usize, usize);
    fn parse((i1, i2, i3): Self::Indices, line: &str) -> Self {
        let mut ws = line.split_whitespace();
        (ws.nth(i1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
         ws.nth(i2 - i1 - 1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
         ws.nth(i3 - i2 - 1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
        )
    }
}

impl<T: FromStr, U: FromStr, V: FromStr, W: FromStr> ParseParts for (T, U, V, W) {
    type Indices = (usize, usize, usize, usize);
    fn parse((i1, i2, i3, i4): Self::Indices, line: &str) -> Self {
        let mut ws = line.split_whitespace();
        (ws.nth(i1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
         ws.nth(i2 - i1 - 1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
         ws.nth(i3 - i2 - 1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
         ws.nth(i4 - i3 - 1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
        )
    }
}

impl<T: FromStr, U: FromStr, V: FromStr, W: FromStr, X: FromStr> ParseParts for (T, U, V, W, X) {
    type Indices = (usize, usize, usize, usize, usize);
    fn parse((i1, i2, i3, i4, i5): Self::Indices, line: &str) -> Self {
        let mut ws = line.split_whitespace();
        (ws.nth(i1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
         ws.nth(i2 - i1 - 1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
         ws.nth(i3 - i2 - 1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
         ws.nth(i4 - i3 - 1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
         ws.nth(i5 - i4 - 1).and_then(|v| v.parse().ok()).unwrap_or_else(
            || panic!("Couldn't parse line {:?}", line)),
        )
    }
}

pub fn parse_fields<P: ParseParts>(line: &str, indices: P::Indices) -> P {
    P::parse(indices, line)
}


pub trait IterExt: Iterator {
    fn sum_from(self, start: Self::Item) -> Self::Item where
        <Self as Iterator>::Item: Add<Self::Item, Output=Self::Item>, Self: Sized
    {
        self.fold(start, |s, e| s + e)
    }
}

impl<I: Iterator> IterExt for I { }


pub struct Uids<T> {
    map: HashMap<T, usize>
}

impl<T: Hash + Eq> Uids<T> {
    pub fn new() -> Uids<T> {
        Uids { map: HashMap::new() }
    }

    pub fn get_id(&mut self, k: T) -> usize {
        let n = self.map.len();
        *self.map.entry(k).or_insert(n)
    }
}


macro_rules! impl_to {
    ($fname:ident, $ty:ty) => {
        pub fn $fname<T: AsRef<str>>(s: T) -> $ty {
            s.as_ref().parse().unwrap()
        }
    };
}

impl_to!(to_u8, u8);
impl_to!(to_u32, u32);
impl_to!(to_usize, usize);
impl_to!(to_i32, i32);

pub fn from_utf8<T: AsRef<[u8]>>(s: T) -> String {
    std::str::from_utf8(s.as_ref()).unwrap().to_owned()
}
