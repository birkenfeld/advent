use std::fmt;
use std::iter::once;
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
        *self = match dir {
            U => self.up(),
            D => self.down(),
            L => self.left(),
            R => self.right()
        };
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

#[derive(Clone, PartialEq)]
pub struct Grid<T> {
    w: usize,
    h: usize,
    v: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(it: impl IntoIterator<Item=Vec<T>>) -> Self {
        let mut v = Vec::new();
        let mut w = 0;
        for mut item in it {
            w = item.len();
            v.append(&mut item);
        }
        Self { w, h: v.len() / w, v }
    }

    pub fn from_iter(w: usize, it: impl IntoIterator<Item=T>) -> Self {
        let v = it.into_iter().collect_vec();
        Self { w, h: v.len() / w, v }
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn positions(&self) -> impl Iterator<Item=Pos<usize>> + 'static {
        (0..self.h).cartesian_product(0..self.w).map(|(y, x)| Pos(x, y))
    }

    pub fn find_pos(&self, mut f: impl FnMut(&T) -> bool) -> Option<Pos<usize>> {
        self.positions().find(|&p| f(&self[p]))
    }

    pub fn for_neighbors<N>(&mut self, pos: Pos<N>, mut f: impl FnMut(&mut T))
    where N: Integer + Copy + FromPrimitive + ToPrimitive
    {
        let (w, h) = (N::from_usize(self.w).expect("invalid width"),
                      N::from_usize(self.h).expect("invalid height"));
        Dir::iter().flat_map(|d| pos.maybe_step(d, w, h)).for_each(|p| f(&mut self[p]));
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
        [U, D, R, L].iter().cloned()
    }
}
