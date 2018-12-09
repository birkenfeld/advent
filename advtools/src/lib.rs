pub extern crate itertools;
pub extern crate arrayvec;
pub extern crate rayon;
pub extern crate regex;
pub extern crate odds;
pub extern crate hashbrown;

use std::hash::Hash;

pub mod prelude {
    pub use std::collections::VecDeque;
    pub use std::collections::hash_map::Entry;
    pub use std::iter::FromIterator;

    pub use hashbrown::{HashMap, HashSet};
    pub use itertools;
    pub use itertools::Itertools;
    pub use regex;
    pub use regex::{Regex, Captures};
    pub use odds;
    pub use odds::slice::rotate_left;
    pub use arrayvec;
    pub use arrayvec::ArrayVec;
    pub use rayon;
}

use std::cell::RefCell;
use std::path::Path;
use std::fmt::Display;

thread_local! {
    static INPUT: RefCell<Option<String>> = Default::default();
    static BENCH_MODE: RefCell<Option<u32>> = RefCell::new(Some(0));
}

pub fn bench_mode(path: impl AsRef<Path>) {
    BENCH_MODE.with(|k| *k.borrow_mut() = None);
    INPUT.with(|k| *k.borrow_mut() = Some(
        std::fs::read_to_string(path.as_ref()).unwrap_or_else(
            |e| panic!("could not read input file: {}", e))
    ));
}

pub fn print(part: &str, value: impl Display) {
    BENCH_MODE.with(|k| if let Some(ref mut n) = *k.borrow_mut() {
        *n += 1;
        println!("{}. {}: {}", n, part, value);
    });
}

pub mod input {
    use std::borrow::Cow;
    use std::env;
    use std::io::{BufRead, Cursor};
    use std::marker::PhantomData;
    use std::path::Path;
    use regex::{Regex, CaptureLocations};
    use itertools::Itertools;

    pub fn input_string() -> String {
        ::INPUT.with(|k| k.borrow().clone().unwrap_or_else(|| {
            let mut infile = Path::new("input").join(
                Path::new(&env::args_os().next().expect("no executable name")
                ).file_name().expect("no file name?"));
            infile.set_extension("txt");
            std::fs::read_to_string(&infile).unwrap_or_else(
                |e| panic!("could not read input file: {}", e))
        }))
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
        fn list(&self) -> Cow<'static, [usize]> {
            vec![self.0, self.1, self.2, self.3, self.4].into()
        }
    }

    pub type TokIter<'t> = Iterator<Item = &'t str> + 't;

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

    pub struct InputIterator<T, R> {
        rdr: R,
        trim: Vec<char>,
        indices: Cow<'static, [usize]>,
        marker: PhantomData<T>,
    }

    impl<T: ParseResult, R: BufRead> Iterator for InputIterator<T, R> {
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
            Some(T::read_line(Cow::from(line), &self.trim, &self.indices))
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
                while line.trim_right() != line {
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


    pub fn input_file() -> impl BufRead {
        Cursor::new(input_string())
    }

    pub fn iter_input<T: ParseResult>() -> InputIterator<T, impl BufRead> {
        InputIterator { rdr: input_file(), trim: vec![],
                        indices: vec![].into(), marker: PhantomData }
    }

    pub fn iter_input_trim<T: ParseResult>(trim: &str) -> InputIterator<T, impl BufRead> {
        InputIterator { rdr: input_file(), trim: trim.chars().collect(),
                        indices: vec![].into(), marker: PhantomData }
    }

    pub fn iter_input_parts<T: ParseResult, Ix: Indices>(ix: Ix) -> InputIterator<T, impl BufRead> {
        InputIterator { rdr: input_file(), trim: vec![],
                        indices: ix.list(), marker: PhantomData }
    }

    pub fn iter_input_parts_trim<T: ParseResult, Ix: Indices>(ix: Ix, trim: &str) -> InputIterator<T, impl BufRead> {
        InputIterator { rdr: input_file(), trim: trim.chars().collect(),
                        indices: ix.list(), marker: PhantomData }
    }

    pub fn iter_input_regex<T: ParseResult>(regex: &str) -> RegexInputIterator<T, impl BufRead> {
        let rx = Regex::new(regex).expect("given regex is invalid");
        let loc = rx.capture_locations();
        RegexInputIterator { rx, loc, rdr: input_file(), marker: PhantomData }
    }

    pub fn parse_str<T: ParseResult>(part: &str) -> T {
        T::read_token(&mut [part].into_iter().map(|&v| v)).unwrap()
    }

    pub fn parse_parts<T: ParseResult, Ix: Indices>(line: &str, ix: Ix) -> T {
        T::read_line(line.into(), &[], &ix.list()[..])
    }

    pub fn parse_parts_trim<T: ParseResult, Ix: Indices>(line: &str, ix: Ix, trim: &str) -> T {
        let trim: Vec<_> = trim.chars().collect();
        T::read_line(line.into(), &trim, &ix.list()[..])
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
}

pub fn rotate_right<T>(t: &mut [T], n: usize) {
    let m = t.len() - n;
    odds::slice::rotate_left(t, m);
}

pub struct Uids<T> {
    map: hashbrown::HashMap<T, usize>
}

impl<T: Hash + Eq> Uids<T> {
    pub fn new() -> Uids<T> {
        Uids { map: Default::default() }
    }

    pub fn get_id(&mut self, k: T) -> usize {
        let n = self.map.len();
        *self.map.entry(k).or_insert(n)
    }
}
