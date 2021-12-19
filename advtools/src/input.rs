use std::{any, env, path};
use regex::Regex;

// Main input API

pub fn set(s: &str) {
    crate::INPUT.with(|k| *k.borrow_mut() = Some(Box::leak(s.into())));
}

pub fn raw_string() -> &'static str {
    let mut infile = path::Path::new("input").join(
        path::Path::new(&env::args_os().next().expect("no executable name")
        ).file_name().expect("no file name?"));
    infile.set_extension("txt");
    crate::INPUT.with(|k| k.borrow().clone().unwrap_or_else(|| {
        Box::leak(
            std::fs::read_to_string(&infile).unwrap_or_else(
                |e| panic!("could not read input file: {}", e)).into()
        )
    }))
}

pub fn string() -> &'static str {
    raw_string().trim_end()
}

pub fn parse<T: InputItem>() -> T {
    T::read_part(&mut string().split_whitespace())
        .unwrap_or_else(|| panic!("input {:?} failed to convert to {}",
                                  string(), any::type_name::<T>()))
}

pub fn rx_parse<T: InputItem>(regex: &str) -> T {
    let rx = Regex::new(regex).expect("given regex is invalid");
    let caps = rx.captures(string()).unwrap_or_else(
        || panic!("input {:?} did not match the regex {:?}", string(), rx.as_str()));
    let mut part_iter = (1..rx.captures_len()).map(|i| {
        caps.get(i).map(|c| c.as_str()).unwrap_or("")
    });
    T::read_part(&mut part_iter)
        .unwrap_or_else(|| panic!("input {:?} failed to convert to {}",
                                  string(), any::type_name::<T>()))
}

pub fn lines() -> impl Iterator<Item=&'static str> {
    string().lines().map(|l| l.trim_end()).filter(|l| !l.is_empty())
}

pub fn chars() -> impl Iterator<Item=char> {
    string().chars()
}

pub fn parse_vec<T: InputItem>() -> Vec<T> {
    parse_lines().collect()
}

pub fn parse_lines<T: InputItem>() -> impl Iterator<Item=T> {
    lines().map(|line| {
        T::read_part(&mut line.split_whitespace())
            .unwrap_or_else(|| panic!("line {:?} failed to convert to {}",
                                      line, any::type_name::<T>()))
    })
}

pub fn rx_lines<T: InputItem>(regex: &str) -> impl Iterator<Item=T> {
    let rx = Regex::new(regex).expect("given regex is invalid");
    let mut loc = rx.capture_locations();
    lines().map(move |line| {
        let _ = rx.captures_read(&mut loc, line).unwrap_or_else(
            || panic!("line {:?} did not match the regex {:?}", line, rx.as_str()));
        let mut part_iter = (1..rx.captures_len()).map(|i| {
            loc.get(i).map(|(s, e)| &line[s..e]).unwrap_or("")
        });
        T::read_part(&mut part_iter)
            .unwrap_or_else(|| panic!("line {:?} failed to convert to {}",
                                      line, any::type_name::<T>()))
    })
}


// InputItem trait

/// Trait implemented for all types that can be parsed from an input line.
pub trait InputItem where Self: Sized {
    /// Take parts from the iterator and try to parse them into `Self`.
    fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self>;
}

// &str: just delivers a single token
impl<'a> InputItem for &'a str {
    fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
        tok.next()
    }
}

// char: takes the first character of a token
impl InputItem for char {
    fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
        tok.next()?.chars().next()
    }
}

// unit: discards the value but still consumes a token
impl InputItem for () {
    fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
        tok.next().map(drop)
    }
}

// simple impls for primitive types
macro_rules! simple_impl {
    ($($ty:ty)+) => { $(
        impl InputItem for $ty {
            fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
                tok.next()?.trim_matches(&[',', ':'][..]).parse().ok()
            }
        }
        )+
    }
}

simple_impl!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 bool);

// Container impls

// Option: allows the sub-type to fail parsing.  This is very useful with
// regexes parsing inputs that have two or more alternative line types.
impl<T: InputItem> InputItem for Option<T> {
    fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
        Some(T::read_part(tok))
    }
}

// Vec: takes as many sub-items as possible.
impl<T> InputItem for Vec<T> where T: InputItem {
    fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
        let mut result = Vec::new();
        while let Some(item) = T::read_part(tok) {
            result.push(item)
        }
        Some(result)
    }
}

// Tuple and array: takes the exact number of sub-itesm.
macro_rules! tuple_impl {
    ($($tys:ident),+) => {
        impl<$($tys: InputItem),+> InputItem for ($($tys),+ ,) {
            #[allow(non_snake_case)]
            fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
                // Consume all parts for subitems, regardless of if they parse or not.
                let ( $($tys),+, ) = (
                    $( $tys::read_part(tok) ),+ ,
                );
                // Afterwards apply `?` if one of the subitems failed.
                Some(( $( $tys? ),+ , ))
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
        impl<$ty: InputItem> InputItem for [$ty; $n] {
            fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
                Some([
                    $( $ty::read_part(tok) $qm ),+
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

// Parsing comma-separated values

pub struct Csv<T>{
    pub vec: Vec<T>
}

impl<T: InputItem> InputItem for Csv<T> {
    fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
        let mut vec = vec![];
        while let Some(item) = tok.next() {
            let mut parts = item.split(',').map(|t| t.trim()).filter(|c| !c.is_empty());
            if let Some(res) = <Vec<T>>::read_part(&mut parts) {
                vec.extend(res);
            }
        }
        Some(Csv { vec })
    }
}

// Assorted helper functions

macro_rules! impl_to {
    ($fname:ident, $ty:ty) => {
        pub fn $fname<T: AsRef<str>>(s: T) -> $ty {
            s.as_ref().parse().unwrap_or_else(
                |_| panic!("could not parse {:?} into a {}",
                           s.as_ref(), stringify!($ty)))
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
