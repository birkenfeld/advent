pub extern crate itertools;
pub extern crate regex;
pub extern crate odds;

use std::borrow::Cow;
use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufRead, Read};
use std::marker::PhantomData;
use std::ops::Add;
use std::path::Path;
use itertools::Itertools;

pub mod prelude {
    pub use std::collections::{HashMap, HashSet, VecDeque};
    pub use std::collections::hash_map::Entry;
    pub use std::iter::FromIterator;

    pub use itertools;
    pub use itertools::Itertools;
    pub use regex;
    pub use regex::{Regex, Captures};
    pub use odds;
    pub use odds::slice::rotate_left;

    pub use super::{input_file, input_string};
    pub use super::{parse_parts, parse_parts_trim};
    pub use super::{iter_input, iter_input_trim, iter_input_parts, iter_input_parts_trim};
    pub use super::IterExt;
    pub use super::{to_u8, to_u32, to_usize, to_i32};
    pub use super::from_utf8;
    pub use super::rotate_right;
}

pub fn input_file() -> File {
    let mut infile = Path::new("input").join(
        Path::new(&env::args_os().item()).file_name().unwrap());
    infile.set_extension("txt");
    File::open(&infile)
        .unwrap_or_else(|_| panic!("input file \"{}\" not found", infile.display()))
}

pub fn input_string() -> String {
    let mut contents = String::new();
    input_file().read_to_string(&mut contents).unwrap();
    contents
}

pub trait Indices {
    fn list(&self) -> Cow<'static, [usize]>;
}

impl Indices for usize {
    fn list(&self) -> Cow<'static, [usize]> { vec![*self].into() }
}

impl Indices for &'static [usize] {
    fn list(&self) -> Cow<'static, [usize]> { (*self).into() }
}

impl Indices for (usize, usize) {
    fn list(&self) -> Cow<'static, [usize]> { vec![self.0, self.1].into() }
}

impl Indices for (usize, usize, usize) {
    fn list(&self) -> Cow<'static, [usize]> { vec![self.0, self.1, self.2].into() }
}

impl Indices for (usize, usize, usize, usize) {
    fn list(&self) -> Cow<'static, [usize]> { vec![self.0, self.1, self.2, self.3].into() }
}

impl Indices for (usize, usize, usize, usize, usize) {
    fn list(&self) -> Cow<'static, [usize]> { vec![self.0, self.1, self.2, self.3, self.4].into() }
}

pub type TokIter<'t> = Iterator<Item = &'t str> + 't;

pub trait ParseResult where Self: Sized {
    fn read(line: Cow<str>, trim: &[char], mut indices: &[usize]) -> Self {
        let mut part_iter = line.split_whitespace().map(|v| v.trim_matches(trim));
        if !indices.is_empty() {
            let filter_iter = &mut part_iter.enumerate().batching(|it| loop {
                if indices.is_empty() { return None; }
                let (ix, item) = it.next().unwrap();
                if ix == indices[0] {
                    indices = &indices[1..];
                    return Some(item);
                }
            }) as &mut TokIter;
            Self::read_token(filter_iter).unwrap()
        } else {
            Self::read_token(&mut part_iter).unwrap()
        }
    }
    fn read_token(tok: &mut TokIter) -> Option<Self>;
}

impl ParseResult for String {
    fn read(line: Cow<str>, trim: &[char], _: &[usize]) -> String {
        if trim.is_empty() {
            line.into_owned()
        } else {
            line.trim_matches(trim).to_owned()
        }
    }
    fn read_token(tok: &mut TokIter) -> Option<String> {
        tok.next().map(ToOwned::to_owned)
    }
}

impl<T> ParseResult for Vec<T> where T: ParseResult {
    fn read_token(tok: &mut TokIter) -> Option<Vec<T>> {
        let mut result = Vec::new();
        while let Some(item) = T::read_token(tok) {
            result.push(item)
        }
        Some(result)
    }
}

macro_rules! simple_impl {
    ($ty:ty) => {
        impl ParseResult for $ty {
            fn read_token(tok: &mut TokIter) -> Option<$ty> {
                Some(tok.next()?.parse().unwrap())
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

// macro_rules! array_impl {
//     ($n:expr, $tok1:ident, $($tok:ident),+) => {
//         impl<T: ParseResult> ParseResult for [T; $n] {
//             fn read_token($tok1: &mut TokIter) -> [T; $n] {
//                 [$(T::read_token($tok)),+]
//             }
//         }
//     }
// }

// array_impl!(1, tok, tok);
// array_impl!(2, tok, tok, tok);
// array_impl!(3, tok, tok, tok, tok);
// array_impl!(4, tok, tok, tok, tok, tok);
// array_impl!(5, tok, tok, tok, tok, tok, tok);
// array_impl!(6, tok, tok, tok, tok, tok, tok, tok);
// array_impl!(7, tok, tok, tok, tok, tok, tok, tok, tok);
// array_impl!(8, tok, tok, tok, tok, tok, tok, tok, tok, tok);

impl ParseResult for char {
    fn read_token(tok: &mut TokIter) -> Option<char> {
        tok.next()?.chars().next()
    }
}

impl ParseResult for () {
    fn read_token(tok: &mut TokIter) -> Option<()> {
        tok.next().map(|_| ())
    }
}

impl<T: ParseResult> ParseResult for (T,) {
    fn read_token(tok: &mut TokIter) -> Option<(T,)> {
        T::read_token(tok).map(|v| (v,))
    }
}

macro_rules! tuple_impl {
    ($($tys:ident),+) => {
        impl<$($tys: ParseResult),+> ParseResult for ($($tys),+) {
            #[allow(non_snake_case)]
            fn read_token(tok: &mut TokIter) -> Option<($($tys),+)> {
                $(
                    let $tys = $tys::read_token(tok)?;
                )+
                Some(($($tys),+))
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


pub struct InputIterator<T: ParseResult> {
    rdr: BufReader<File>,
    trim: Vec<char>,
    indices: Cow<'static, [usize]>,
    marker: PhantomData<T>,
}

impl<T: ParseResult> Iterator for InputIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut line = String::new();
        while line.is_empty() {
            if self.rdr.read_line(&mut line).unwrap() == 0 {
                return None;
            }
            while line.trim_right() != line {
                line.pop();
            }
        }
            Some(T::read(Cow::from(line), &self.trim, &self.indices))
    }
}


pub fn iter_input<T: ParseResult>() -> InputIterator<T> {
    let rdr = BufReader::new(input_file());
    InputIterator { rdr: rdr, trim: vec![], indices: vec![].into(), marker: PhantomData }
}

pub fn iter_input_trim<T: ParseResult>(trim: &str) -> InputIterator<T> {
    let rdr = BufReader::new(input_file());
    InputIterator { rdr: rdr, trim: trim.chars().collect(), indices: vec![].into(), marker: PhantomData }
}

pub fn iter_input_parts<T: ParseResult, Ix: Indices>(ix: Ix) -> InputIterator<T> {
    let rdr = BufReader::new(input_file());
    InputIterator { rdr: rdr, trim: vec![], indices: ix.list(), marker: PhantomData }
}

pub fn iter_input_parts_trim<T: ParseResult, Ix: Indices>(ix: Ix, trim: &str) -> InputIterator<T> {
    let rdr = BufReader::new(input_file());
    InputIterator { rdr: rdr, trim: trim.chars().collect(), indices: ix.list(), marker: PhantomData }
}

pub fn parse_parts<T: ParseResult, Ix: Indices>(line: &str, ix: Ix) -> T {
    T::read(line.into(), &[], &ix.list()[..])
}

pub fn parse_parts_trim<T: ParseResult, Ix: Indices>(line: &str, ix: Ix, trim: &str) -> T {
    let trim: Vec<_> = trim.chars().collect();
    T::read(line.into(), &trim, &ix.list()[..])
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

pub fn rotate_right<T>(t: &mut [T], n: usize) {
    let m = t.len() - n;
    odds::slice::rotate_left(t, m);
}
