use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufRead, Read};
use std::marker::PhantomData;
use std::ops::Add;

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


pub fn iter_input<I: Input>() -> InputIterator<I> {
    let fp = File::open("input.txt").expect("input file \"input.txt\" not found in cwd");
    let rdr = BufReader::new(fp);
    InputIterator { rdr: rdr, marker: PhantomData }
}


pub fn input_string() -> String {
    let mut fp = File::open("input.txt").expect("input file \"input.txt\" not found in cwd");
    let mut contents = String::new();
    fp.read_to_string(&mut contents).unwrap();
    contents
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


pub fn sorted<T: Ord, I: Iterator<Item=T>>(it: I) -> Vec<T> {
    let mut v: Vec<T> = it.collect();
    v.sort();
    v
}
