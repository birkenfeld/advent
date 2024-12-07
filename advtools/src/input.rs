use std::{any, path::Path};
use regex::Regex;

// Main input API

pub fn set(s: &str) {
    crate::Timer::start();
    *crate::INPUT.lock().unwrap() = Some(Box::leak(s.into()));
}

pub fn raw_string() -> &'static str {
    crate::INPUT.lock().unwrap().get_or_insert_with(|| {
        let mut args = std::env::args_os();
        let exe = args.next().expect("no executable name");
        let exe = Path::new(&exe).file_name().expect("no file name?");
        let exe = exe.to_str().expect("not utf-8");
        // Try directly here
        let mut infile = Path::new("input").join(&exe);
        // Try in yearly subdirectory
        if !infile.is_file() {
            infile = Path::new(&exe[..4]).join("input").join(&exe);
        }
        // Allow giving explicit input file name on the command line
        if let Some(arg) = args.next() {
            infile = Path::new(&arg).into();
        }
        infile.set_extension("txt");
        crate::Timer::start();
        Box::leak(
            std::fs::read_to_string(&infile).unwrap_or_else(
                |e| panic!("could not read input file: {}", e)).into()
        )
    })
}

pub fn string() -> &'static str {
    raw_string().trim_end()
}

pub fn parse<T: InputItem<'static>>() -> T {
    parse_str(string())
    // T::read_part(&mut string().split_whitespace())
    //     .unwrap_or_else(|| panic!("input {:?} failed to convert to {}",
    //                               string(), any::type_name::<T>()))
}

pub fn rx_parse<T: InputItem<'static>>(regex: &str) -> T {
    rx_parse_str(regex, string())
}

pub fn lines() -> impl Iterator<Item=&'static str> {
    string().lines().map(|l| l.trim_end()).filter(|l| !l.is_empty())
}

pub fn chars() -> impl Iterator<Item=char> {
    string().chars()
}

pub fn parse_vec<T: InputItem<'static>>() -> Vec<T> {
    parse_lines().collect()
}

pub fn parse_lines<T: InputItem<'static>>() -> impl Iterator<Item=T> {
    lines().map(|line| {
        T::read_part(&mut line.split_whitespace())
            .unwrap_or_else(|| panic!("line {:?} failed to convert to {}",
                                      line, any::type_name::<T>()))
    })
}

pub fn rx_lines<T: InputItem<'static>>(regex: &str) -> impl Iterator<Item=T> {
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


// Parsing arbitrary data, not input

pub fn parse_str<'a, T: InputItem<'a>>(s: &'a str) -> T {
    T::read_part(&mut s.split_whitespace())
        .unwrap_or_else(|| panic!("string {:?} failed to convert to {}",
                                  s, any::type_name::<T>()))
}

pub fn rx_parse_str<'a, T: InputItem<'a>>(regex: &str, s: &'a str) -> T {
    let rx = Regex::new(regex).expect("given regex is invalid");
    let caps = rx.captures(s).unwrap_or_else(
        || panic!("input {:?} did not match the regex {:?}", string(), rx.as_str()));
    let mut part_iter = (1..rx.captures_len()).map(|i| {
        caps.get(i).map(|c| c.as_str()).unwrap_or("")
    });
    T::read_part(&mut part_iter)
        .unwrap_or_else(|| panic!("input {:?} failed to convert to {}",
                                  string(), any::type_name::<T>()))
}

// InputItem trait

/// Trait implemented for all types that can be parsed from an input line.
pub trait InputItem<'a> where Self: Sized {
    /// Take parts from the iterator and try to parse them into `Self`.
    fn read_part(tok: &mut impl Iterator<Item=&'a str>) -> Option<Self>;
}

// &str: just delivers a single token
impl<'a> InputItem<'a> for &'a str {
    fn read_part(tok: &mut impl Iterator<Item=&'a str>) -> Option<Self> {
        tok.next()
    }
}

// char: takes the first character of a token
impl<'a> InputItem<'a> for char {
    fn read_part(tok: &mut impl Iterator<Item=&'a str>) -> Option<Self> {
        tok.next()?.chars().next()
    }
}

// unit: discards the value but still consumes a token
impl<'a> InputItem<'a> for () {
    fn read_part(tok: &mut impl Iterator<Item=&'a str>) -> Option<Self> {
        tok.next().map(drop)
    }
}

// simple impls for primitive types
macro_rules! simple_impl {
    ($($ty:ty)+) => { $(
        impl<'a> InputItem<'a> for $ty {
            fn read_part(tok: &mut impl Iterator<Item=&'a str>) -> Option<Self> {
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
impl<'a, T: InputItem<'a>> InputItem<'a> for Option<T> {
    fn read_part(tok: &mut impl Iterator<Item=&'a str>) -> Option<Self> {
        Some(T::read_part(tok))
    }
}

// Vec: takes as many sub-items as possible.
impl<'a, T> InputItem<'a> for Vec<T> where T: InputItem<'a> {
    fn read_part(tok: &mut impl Iterator<Item=&'a str>) -> Option<Self> {
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
        impl<'a, $($tys: InputItem<'a>),+> InputItem<'a> for ($($tys),+ ,) {
            #[allow(non_snake_case)]
            fn read_part(tok: &mut impl Iterator<Item=&'a str>) -> Option<Self> {
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
        impl<'a, $ty: InputItem<'a>> InputItem<'a> for [$ty; $n] {
            fn read_part(tok: &mut impl Iterator<Item=&'a str>) -> Option<Self> {
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

// Parsing separated values from sub-items

pub struct Sep<T, const SEP: char = ','> {
    pub vec: Vec<T>
}

impl<'a, T: InputItem<'a>, const SEP: char> InputItem<'a> for Sep<T, SEP> {
    fn read_part(tok: &mut impl Iterator<Item=&'a str>) -> Option<Self> {
        let mut vec = vec![];
        for item in tok {
            let mut parts = item.split(SEP).map(|t| t.trim()).filter(|c| !c.is_empty());
            if let Some(res) = <Vec<T>>::read_part(&mut parts) {
                vec.extend(res);
            }
        }
        Some(Sep { vec })
    }
}
