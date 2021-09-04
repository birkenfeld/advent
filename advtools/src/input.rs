use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use regex::{Regex, CaptureLocations};
use itertools::Itertools;

fn input_file_name() -> PathBuf {
    let mut infile = Path::new("input").join(
        Path::new(&env::args_os().next().expect("no executable name")
        ).file_name().expect("no file name?"));
    infile.set_extension("txt");
    infile
}

pub fn input_file() -> Box<dyn BufRead> {
    crate::INPUT.with(|k| match k.borrow().clone() {
        Some(s) => Box::new(Cursor::new(s)) as Box<dyn BufRead>,
        None => {
            let f = File::open(&input_file_name()).unwrap_or_else(
                |e| panic!("could not read input file: {}", e));
            Box::new(BufReader::new(f))
        }
    })
}

pub fn input_string() -> String {
    crate::INPUT.with(|k| k.borrow().clone().unwrap_or_else(|| {
        std::fs::read_to_string(&input_file_name()).unwrap_or_else(
            |e| panic!("could not read input file: {}", e))
    }))
}

pub type TokIter<'t> = dyn Iterator<Item = &'t str> + 't;

pub trait ParseResult where Self: Sized {
    fn read_line(line: Cow<str>, trim: &[char], mut indices: &[usize]) -> Self {
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
    // Special case: reads the whole line.
    fn read_line(line: Cow<str>, trim: &[char], _: &[usize]) -> String {
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

macro_rules! tuple_impl {
    ($($tys:ident),+) => {
        impl<$($tys: ParseResult),+> ParseResult for ($($tys),+ ,) {
            fn read_token(tok: &mut TokIter) -> Option<($($tys),+ ,)> {
                Some((
                    $( $tys::read_token(tok)? ),+ ,
                ))
            }
        }
    }
}

tuple_impl!(T);
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

macro_rules! array_impl {
    ($ty:ident, $n:expr, $($qm:tt)+) => {
        impl<$ty: ParseResult> ParseResult for [$ty; $n] {
            fn read_token(tok: &mut TokIter) -> Option<Self> {
                Some([
                    $( $ty::read_token(tok) $qm ),+
                ])
            }
        }
    }
}

array_impl!(T, 1, ?);
array_impl!(T, 2, ??);
array_impl!(T, 3, ???);
array_impl!(T, 4, ????);
array_impl!(T, 5, ?????);
array_impl!(T, 6, ??????);
array_impl!(T, 7, ???????);
array_impl!(T, 8, ????????);
array_impl!(T, 9, ?????????);

pub struct InputIterator<T, R, A> {
    rdr: R,
    trim: Vec<char>,
    indices: A,
    marker: PhantomData<T>,
}

impl<T: ParseResult, R: BufRead, const N: usize> Iterator for InputIterator<T, R, [usize; N]> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut line = String::new();
        while line.is_empty() {
            if self.rdr.read_line(&mut line).unwrap() == 0 {
                return None;
            }
            while line.trim_end() != line {
                line.pop();
            }
        }
        Some(T::read_line(Cow::from(line), &self.trim, &self.indices[..]))
    }
}

pub struct RegexInputIterator<T, R> {
    rx: Regex,
    loc: CaptureLocations,
    rdr: R,
    marker: PhantomData<T>,
}

impl<T: ParseResult, R: BufRead> Iterator for RegexInputIterator<T, R> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut line = String::new();
        while line.is_empty() {
            if self.rdr.read_line(&mut line).unwrap() == 0 {
                return None;
            }
            while line.trim_end() != line {
                line.pop();
            }
        }
        let _ = self.rx.captures_read(&mut self.loc, &line).unwrap_or_else(
            || panic!("line {:?} did not match the input regex {:?}",
                      line, self.rx.as_str()));
        let mut tok_iter = (1..self.rx.captures_len()).map(|i| {
            self.loc.get(i).map(|(s, e)| &line[s..e]).unwrap_or("")
        });
        Some(T::read_token(&mut tok_iter).expect("line conversion failed"))
    }
}


pub fn iter_lines() -> InputIterator<String, impl BufRead, [usize; 0]> {
    InputIterator { rdr: input_file(), trim: vec![],
                    indices: [], marker: PhantomData }
}

pub fn iter_input<T: ParseResult>() -> InputIterator<T, impl BufRead, [usize; 0]> {
    InputIterator { rdr: input_file(), trim: vec![],
                    indices: [], marker: PhantomData }
}

pub fn iter_input_trim<T: ParseResult>(trim: &str) -> InputIterator<T, impl BufRead, [usize; 0]> {
    InputIterator { rdr: input_file(), trim: trim.chars().collect(),
                    indices: [], marker: PhantomData }
}

pub fn iter_input_parts<T: ParseResult, Ix, const N: usize>(ix: [Ix; N]) -> InputIterator<T, impl BufRead, [Ix; N]> {
    InputIterator { rdr: input_file(), trim: vec![],
                    indices: ix, marker: PhantomData }
}

pub fn iter_input_parts_trim<T: ParseResult, Ix, const N: usize>(ix: [Ix; N], trim: &str) -> InputIterator<T, impl BufRead, [Ix; N]> {
    InputIterator { rdr: input_file(), trim: trim.chars().collect(),
                    indices: ix, marker: PhantomData }
}

pub fn iter_input_regex<T: ParseResult>(regex: &str) -> RegexInputIterator<T, impl BufRead> {
    let rx = Regex::new(regex).expect("given regex is invalid");
    let loc = rx.capture_locations();
    RegexInputIterator { rx, loc, rdr: input_file(), marker: PhantomData }
}

pub fn parse_str<T: ParseResult>(part: &str) -> T {
    T::read_token(&mut [part].iter().map(|&v| v)).unwrap()
}

pub fn parse_parts<T: ParseResult, const N: usize>(line: &str, ix: [usize; N]) -> T {
    T::read_line(line.into(), &[], &ix[..])
}

pub fn parse_parts_trim<T: ParseResult, const N: usize>(line: &str, ix: [usize; N], trim: &str) -> T {
    let trim: Vec<_> = trim.chars().collect();
    T::read_line(line.into(), &trim, &ix[..])
}

macro_rules! impl_to {
    ($fname:ident, $ty:ty) => {
        pub fn $fname<T: AsRef<str>>(s: T) -> $ty {
            s.as_ref().parse().expect(concat!("expected a ", stringify!($ty)))
        }
    };
}

impl_to!(to_u8, u8);
impl_to!(to_u16, u16);
impl_to!(to_u32, u32);
impl_to!(to_u64, u64);
impl_to!(to_usize, usize);
impl_to!(to_i8, i8);
impl_to!(to_i16, i16);
impl_to!(to_i32, i32);
impl_to!(to_i64, i64);
impl_to!(to_isize, isize);

pub fn from_utf8<T: AsRef<[u8]>>(s: T) -> String {
    std::str::from_utf8(s.as_ref()).expect("input is not valid UTF8").into()
}
