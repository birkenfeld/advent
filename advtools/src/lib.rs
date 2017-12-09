extern crate itertools;
extern crate regex;

use std::borrow::Cow;
use std::env;
use std::fmt::Debug;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufRead, Read};
use std::marker::PhantomData;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;
use itertools::Itertools;

pub mod prelude {
    pub use std::collections::{HashMap, HashSet};
    pub use std::collections::hash_map::Entry;

    pub use itertools::Itertools;
    pub use regex::{Regex, Captures};

    pub use super::IterExt;
    pub use super::parse;
    pub use super::iter_input;
    pub use super::iter_input_trim;
    pub use super::input_file;
    pub use super::input_string;
    pub use super::{to_u8, to_u32, to_usize, to_i32};
    pub use super::from_utf8;
}

pub type TokIter<'t> = Iterator<Item = &'t str> + 't;

pub trait ParseResult where Self: Sized {
    fn read(line: Cow<str>, trim: &[char]) -> Self {
        let iter = &mut line.split_whitespace().map(|v| v.trim_matches(trim)) as &mut TokIter;
        Self::read_token(iter)
    }
    fn read_token(tok: &mut TokIter) -> Self;
}

impl ParseResult for String {
    fn read(line: Cow<str>, trim: &[char]) -> String {
        if trim.is_empty() {
            line.into_owned()
        } else {
            line.trim_matches(trim).to_owned()
        }
    }
    fn read_token(mut tok: &mut TokIter) -> String {
        tok.item().to_owned()
    }
}

impl<T> ParseResult for Vec<T> where T: FromStr, T::Err: Debug {
    fn read(line: Cow<str>, trim: &[char]) -> Vec<T> {
        line.split_whitespace().map(|p| p.trim_matches(trim).parse().unwrap()).collect()
    }
    fn read_token(tok: &mut TokIter) -> Vec<T> {
        tok.map(|p| p.parse().unwrap()).collect()
    }
}

macro_rules! simple_impl {
    ($ty:ty) => {
        impl ParseResult for $ty {
            fn read_token(mut tok: &mut TokIter) -> $ty {
                tok.item().parse().unwrap()
            }
        }
    }
}

simple_impl!(u8);
simple_impl!(u16);
simple_impl!(u32);
simple_impl!(u64);
simple_impl!(usize);
simple_impl!(i8);
simple_impl!(i16);
simple_impl!(i32);
simple_impl!(i64);
simple_impl!(isize);

macro_rules! array_impl {
    ($n:expr, $tok1:ident, $($tok:ident),+) => {
        impl<T: ParseResult> ParseResult for [T; $n] {
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

impl ParseResult for char {
    fn read_token(mut tok: &mut TokIter) -> char {
        tok.item().chars().item()
    }
}

impl ParseResult for () {
    fn read_token(mut tok: &mut TokIter) -> () {
        tok.item();
        ()
    }
}

impl<T: ParseResult> ParseResult for (T,) {
    fn read_token(tok: &mut TokIter) -> (T,) {
        (T::read_token(tok),)
    }
}

macro_rules! tuple_impl {
    ($($tys:ident),+) => {
        impl<$($tys: ParseResult),+> ParseResult for ($($tys),+) {
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


pub struct InputIterator<I: ParseResult> {
    rdr: BufReader<File>,
    trim: Vec<char>,
    marker: PhantomData<I>,
}

impl<I: ParseResult> Iterator for InputIterator<I> {
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
        Some(I::read(Cow::from(line), &self.trim))
    }
}


pub fn input_file() -> File {
    let mut infile = Path::new("input").join(
        Path::new(&env::args_os().item()).file_name().unwrap());
    infile.set_extension("txt");
    File::open(&infile)
        .unwrap_or_else(|_| panic!("input file \"{}\" not found", infile.display()))
}

pub fn iter_input<I: ParseResult>() -> InputIterator<I> {
    let rdr = BufReader::new(input_file());
    InputIterator { rdr: rdr, trim: vec![], marker: PhantomData }
}

pub fn iter_input_trim<I: ParseResult>(trim: &str) -> InputIterator<I> {
    let rdr = BufReader::new(input_file());
    InputIterator { rdr: rdr, trim: trim.chars().collect(), marker: PhantomData }
}

pub fn input_string() -> String {
    let mut contents = String::new();
    input_file().read_to_string(&mut contents).unwrap();
    contents
}

pub trait Indices {
    fn list(&self) -> Cow<[usize]>;
}

impl Indices for usize {
    fn list(&self) -> Cow<[usize]> { vec![*self].into() }
}

impl<'a> Indices for &'a [usize] {
    fn list(&self) -> Cow<[usize]> { (*self).into() }
}

impl Indices for (usize, usize) {
    fn list(&self) -> Cow<[usize]> { vec![self.0, self.1].into() }
}

impl Indices for (usize, usize, usize) {
    fn list(&self) -> Cow<[usize]> { vec![self.0, self.1, self.2].into() }
}

impl Indices for (usize, usize, usize, usize) {
    fn list(&self) -> Cow<[usize]> { vec![self.0, self.1, self.2, self.3].into() }
}

impl Indices for (usize, usize, usize, usize, usize) {
    fn list(&self) -> Cow<[usize]> { vec![self.0, self.1, self.2, self.3, self.4].into() }
}

fn parse_inner<'a, I, It, Ix>(in_iter: It, ix: Ix) -> I
    where I: ParseResult, It: Iterator<Item = &'a str>, Ix: Indices
{
    let mut indices = &ix.list()[..];
    let iter = &mut in_iter.enumerate().batching(|it| loop {
        if indices.is_empty() { return None; }
        let (ix, item) = it.next().unwrap();
        if ix == indices[0] {
            indices = &indices[1..];
            return Some(item);
        }
    }) as &mut TokIter;
    I::read_token(iter)
}

pub fn parse<I: ParseResult, Ix: Indices>(line: &str, ix: Ix) -> I {
    parse_inner(line.split_whitespace(), ix)
}

pub fn parse_trim<I: ParseResult, Ix: Indices>(line: &str, ix: Ix, trim: &str) -> I {
    let trim: Vec<_> = trim.chars().collect();
    parse_inner(line.split_whitespace().map(|v| v.trim_matches(&trim[..])), ix)
}

pub trait IterExt: Iterator {
    fn sum_from(self, start: Self::Item) -> Self::Item where
        <Self as Iterator>::Item: Add<Self::Item, Output=Self::Item>, Self: Sized
    {
        self.fold(start, |s, e| s + e)
    }

    fn item(&mut self) -> Self::Item {
        self.next().unwrap()
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
