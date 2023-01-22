use std::fmt;
use std::iter::{once, repeat};
use std::ops::{Index, IndexMut};
use num::{Integer, Signed, FromPrimitive, ToPrimitive};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos<N = i32> {
    pub y: N,
    pub x: N,
}

#[allow(non_snake_case)]
pub const fn Pos<N>(x: N, y: N) -> Pos<N> {
    Pos { x, y }
}

impl<N: Integer + Copy> Pos<N> {
    pub fn up(self) -> Self {
        Pos(self.x, self.y - N::one())
    }

    pub fn down(self) -> Self {
        Pos(self.x, self.y + N::one())
    }

    pub fn left(self) -> Self {
        Pos(self.x - N::one(), self.y)
    }

    pub fn right(self) -> Self {
        Pos(self.x + N::one(), self.y)
    }

    pub fn to(self, dir: Dir) -> Self {
        match dir {
            U => self.up(),
            D => self.down(),
            L => self.left(),
            R => self.right()
        }
    }

    pub fn step_left(&mut self) {
        *self = self.left();
    }

    pub fn step_right(&mut self) {
        *self = self.right();
    }

    pub fn step_up(&mut self) {
        *self = self.up();
    }

    pub fn step_down(&mut self) {
        *self = self.down();
    }

    pub fn step(&mut self, dir: Dir) -> &mut Self {
        *self = self.to(dir);
        self
    }

    pub fn maybe_step(&self, dir: Dir, w: N, h: N) -> Option<Self> {
        match dir {
            U => if self.y > N::zero()  { Some(self.up()) } else { None },
            D => if self.y < h-N::one() { Some(self.down()) } else { None },
            L => if self.x > N::zero()  { Some(self.left()) } else { None },
            R => if self.x < w-N::one() { Some(self.right()) } else { None },
        }
    }

    pub fn neighbors(&self) -> impl Iterator<Item=Self> {
        once(self.up()).chain(once(self.down()))
                       .chain(once(self.left()))
                       .chain(once(self.right()))
    }
}

impl<N: Integer + Signed> Pos<N> {
    pub fn manhattan(&self) -> N {
        self.x.abs() + self.y.abs()
    }
}

impl<N: fmt::Display> fmt::Display for Pos<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl<N: Integer> std::ops::Add for Pos<N> {
    type Output = Self;
    fn add(self, other: Pos<N>) -> Pos<N> {
        Pos(self.x + other.x, self.y + other.y)
    }
}

impl<N: Integer + Copy> std::ops::AddAssign for Pos<N> {
    fn add_assign(&mut self, other: Pos<N>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<N: Integer + Copy> std::ops::Mul<N> for Pos<N> {
    type Output = Self;
    fn mul(self, other: N) -> Pos<N> {
        Pos(self.x * other, self.y * other)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    w: usize,
    h: usize,
    v: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(it: impl IntoIterator<Item=Vec<T>>) -> Self {
        let mut v = Vec::new();
        let mut it = it.into_iter();
        let mut first = it.next().unwrap();
        let w = first.len();
        v.append(&mut first);
        for mut item in it {
            assert_eq!(item.len(), w);
            v.append(&mut item);
        }
        Self { w, h: v.len() / w, v }
    }

    pub fn from_iter(w: usize, it: impl IntoIterator<Item=T>) -> Self {
        let v = it.into_iter().collect_vec();
        Self { w, h: v.len() / w, v }
    }

    pub fn len(&self) -> usize {
        self.w * self.h
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn center<N>(&self) -> Pos<N>
    where N: Integer + Copy + FromPrimitive + ToPrimitive
    {
        Pos(N::from_usize(self.w / 2).unwrap(), N::from_usize(self.h / 2).unwrap())
    }

    pub fn positions<N>(&self) -> impl Iterator<Item=Pos<N>> + 'static
    where N: Integer + Copy + FromPrimitive + ToPrimitive
    {
        (0..self.h).cartesian_product(0..self.w).map(|(y, x)| {
            Pos(N::from_usize(x).unwrap(), N::from_usize(y).unwrap())
        })
    }

    pub fn find_pos(&self, mut f: impl FnMut(&T) -> bool) -> Option<Pos<usize>> {
        self.positions().find(|&p| f(&self[p]))
    }

    pub fn neighbors<N>(&self, pos: Pos<N>) -> impl Iterator<Item=Pos<N>> + 'static
    where N: Integer + Copy + FromPrimitive + ToPrimitive + 'static
    {
        let (w, h) = (N::from_usize(self.w).expect("invalid width"),
                      N::from_usize(self.h).expect("invalid height"));
        Dir::iter().flat_map(move |d| pos.maybe_step(d, w, h))
    }

    pub fn neighbors_diag<N>(&self, pos: Pos<N>) -> impl Iterator<Item=Pos<N>> + 'static
    where N: Integer + Copy + FromPrimitive + ToPrimitive + 'static
    {
        let (w, h) = (N::from_usize(self.w).expect("invalid width"),
                      N::from_usize(self.h).expect("invalid height"));
        Dir::iter().flat_map(move |d| pos.maybe_step(d, w, h)).chain(
            Dir::iter().flat_map(move |d| pos.maybe_step(d, w, h)
                .and_then(|p| p.maybe_step(d.left(), w, h)))
        )
    }

    pub fn iter(&self) -> impl Iterator<Item=&[T]> {
        self.v.chunks(self.w)
    }

    pub fn get<N: ToPrimitive>(&self, Pos { x, y }: Pos<N>) -> Option<&T> {
        if let Some(y) = y.to_usize() {
            if let Some(x) = x.to_usize() {
                if y < self.h && x < self.w {
                    return self.v.get(y * self.w + x);
                }
            }
        }
        None
    }

    pub fn get_mut<N: ToPrimitive>(&mut self, Pos { x, y }: Pos<N>) -> Option<&mut T> {
        if let Some(y) = y.to_usize() {
            if let Some(x) = x.to_usize() {
                if y < self.h && x < self.w {
                    return self.v.get_mut(y * self.w + x);
                }
            }
        }
        None
    }

    pub fn count(&self, f: impl Fn(&T) -> bool) -> usize {
        self.v.iter().filter(|t| f(*t)).count()
    }

    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Grid<U> {
        Grid {
            w: self.w,
            h: self.h,
            v: self.v.iter().map(f).collect()
        }
    }

    pub fn debug(&self, f: impl Fn(&T) -> char) {
        for row in self.iter() {
            for item in row {
                print!("{}", f(item));
            }
            println!();
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn enlarge(&mut self, n: usize, el: T) {
        let mut new_v = vec![el.clone(); (self.w + 2*n) * n];
        for row in self.iter() {
            new_v.extend(repeat(el.clone()).take(n)
                         .chain(row.iter().cloned())
                         .chain(repeat(el.clone()).take(n)));
        }
        new_v.extend(repeat(el).take((self.w + 2*n) * n));
        self.v = new_v;
        self.w += 2*n;
        self.h += 2*n;
    }
}

impl<T> Grid<Option<T>> {
    pub fn empty(w: usize, h: usize) -> Self {
        Self::from_iter(w, (0..w*h).map(|_| None))
    }
}

impl Grid<bool> {
    pub fn empty(w: usize, h: usize) -> Self {
        Self::from_iter(w, (0..w*h).map(|_| false))
    }
}

impl<T, N: ToPrimitive> Index<Pos<N>> for Grid<T> {
    type Output = T;
    fn index(&self, Pos { x, y }: Pos<N>) -> &T {
        let ix = y.to_usize().expect("invalid Y")*self.w + x.to_usize().expect("invalid X");
        &self.v[ix]
    }
}

impl<T, N: ToPrimitive> IndexMut<Pos<N>> for Grid<T> {
    fn index_mut(&mut self, Pos { x, y }: Pos<N>) -> &mut T {
        let ix = y.to_usize().expect("invalid Y")*self.w + x.to_usize().expect("invalid X");
        &mut self.v[ix]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &T {
        &self[Pos(x, y)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self[Pos(x, y)]
    }
}

impl fmt::Display for Grid<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, val) in self.v.iter().enumerate() {
            write!(f, "{}", if *val { "#" } else { "." })?;
            if i % self.w == self.w-1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

use Dir::*;

impl Dir {
    pub fn left(&self)  -> Self { match self { U => L, R => U, D => R, L => D } }
    pub fn right(&self) -> Self { match self { U => R, R => D, D => L, L => U } }
    pub fn flip(&self)  -> Self { match self { U => D, R => L, D => U, L => R } }
    pub fn ul_dr(&self) -> Self { match self { U => L, R => D, D => R, L => U } }
    pub fn ur_dl(&self) -> Self { match self { U => R, R => U, D => L, L => D } }

    pub fn from_str(s: &str) -> Self {
        match s {
            "U" | "N" | "^" => U,
            "D" | "S" | "v" => D,
            "L" | "W" | "<" => L,
            "R" | "E" | ">" => R,
            _ => unreachable!("invalid direction")
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            'U' | 'N' | '^' => U,
            'D' | 'S' | 'v' => D,
            'L' | 'W' | '<' => L,
            'R' | 'E' | '>' => R,
            _ => unreachable!("invalid direction")
        }
    }

    pub fn as_bytes(&self) -> &'static [u8] {
        match *self {
            U => b"U",
            D => b"D",
            L => b"L",
            R => b"R",
        }
    }

    pub fn iter() -> impl Iterator<Item=Self> {
        [U, D, R, L].into_iter()
    }
}
