use std::cell::RefCell;
use std::path::Path;
use std::fmt::Display;
use std::sync::atomic::{AtomicI32, Ordering};

pub use rayon;
pub use itertools;

pub mod input;
pub mod grid;

pub mod prelude {
    pub use std::collections::VecDeque;
    pub use std::collections::hash_map::Entry;
    pub use std::iter::once;

    pub use hashbrown::{HashMap, HashSet};
    pub use itertools::{Itertools, iproduct};
    pub use regex::{Regex, Captures};
    pub use odds::slice::rotate_left;
    pub use arrayvec::ArrayVec;

    pub fn rotate_right<T>(t: &mut [T], n: usize) {
        let m = t.len() - n;
        odds::slice::rotate_left(t, m);
    }

    pub struct Uids<T> {
        map: hashbrown::HashMap<T, usize>
    }

    impl<T: std::hash::Hash + Eq> Uids<T> {
        pub fn new() -> Uids<T> {
            Uids { map: Default::default() }
        }

        pub fn get_id(&mut self, k: T) -> usize {
            let n = self.map.len();
            *self.map.entry(k).or_insert(n)
        }
    }

    impl<T, Q> std::ops::Index<&Q> for Uids<T>
    where T: std::hash::Hash + Eq + std::borrow::Borrow<Q>, Q: std::hash::Hash + Eq + ?Sized
    {
        type Output = usize;
        fn index(&self, q: &Q) -> &usize {
            &self.map[&q]
        }
    }

    /// Perform a binary search
    pub fn binary_search<I, F>(mut low: I, mut high: I, mut test: F) -> I
    where I: num::Integer + Copy + From<u8>, F: FnMut(I) -> bool
    {
        loop {
            if low + I::one() == high {
                return high;
            }
            let guess = (low + high) / I::from(2);
            if test(guess) {
                high = guess;
            } else {
                low = guess;
            }
        }
    }
}

thread_local! {
    static INPUT: RefCell<Option<&'static str>> = Default::default();
}

static OUT_CONTROL: AtomicI32 = AtomicI32::new(1);

pub fn bench_mode(path: impl AsRef<Path>) {
    OUT_CONTROL.store(0, Ordering::SeqCst);
    INPUT.with(|k| *k.borrow_mut() = Some(
        Box::leak(
            std::fs::read_to_string(path.as_ref()).unwrap_or_else(
                |e| panic!("could not read input file: {}", e)).into()
        )
    ));
}

pub fn print(part: &str, value: impl Display) {
    if OUT_CONTROL.load(Ordering::SeqCst) > 0 {
        let n = OUT_CONTROL.fetch_add(1, Ordering::SeqCst);
        println!("{}. {}: {}", n, part, value);
    }
}

pub fn verify(part: &str, value: impl Display, check: impl Display) {
    let value_str = format!("{}", value);
    let check_str = format!("{}", check);
    assert_eq!(value_str, check_str);
    if OUT_CONTROL.load(Ordering::SeqCst) > 0 {
        let n = OUT_CONTROL.fetch_add(1, Ordering::SeqCst);
        println!("{}. {}: {}", n, part, value_str);
    }
}
