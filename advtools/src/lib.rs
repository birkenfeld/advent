use std::cell::RefCell;
use std::fmt::Display;
use std::sync::{Mutex, atomic::{AtomicI32, Ordering}};
use std::time::Instant;

pub use rayon;
pub use itertools;
pub use petgraph;
pub use memoize;
pub use num;

pub mod input;
pub mod grid;
pub mod vecs;

pub mod prelude {
    pub use std::collections::VecDeque;
    pub use std::collections::hash_map::Entry;
    pub use std::iter::once;

    pub use hashbrown::{HashMap, HashSet};
    pub use itertools::{Itertools, iproduct};
    pub use regex::{Regex, Captures};
    pub use arrayvec::ArrayVec;
    pub use num::integer::lcm;

    #[derive(Default)]
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
            &self.map[q]
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

static INPUT: Mutex<Option<&'static str>> = Mutex::new(None);


// Some magic to automatically print elapsed time on exit.

struct Timer(Instant);

thread_local!(static TIMER: RefCell<Option<Timer>> = RefCell::new(None));

impl Timer {
    fn start() {
        TIMER.with(|k| *k.borrow_mut() = Some(Timer(Instant::now())));
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("   Elapsed: {:?}", self.0.elapsed());
    }
}

static OUT_CONTROL: AtomicI32 = AtomicI32::new(1);

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
