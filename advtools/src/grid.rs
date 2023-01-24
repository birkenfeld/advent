use std::fmt;
use std::iter::{once, repeat};
use std::ops::{Index, IndexMut};
use num::{Integer, Signed, FromPrimitive, ToPrimitive};
use itertools::Itertools;

/// An (x,y) position within a grid.
///
/// The `N` type can be selected freely depending on need; overflow must be
/// considered carefully if it is unsigned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos<N = i32> {
    pub y: N,
    pub x: N,
}

/// Helper constructor to allow `Pos(x, y)` although `Pos` is not a tuple
/// struct.
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

    pub fn maybe_to(&self, dir: Dir, w: N, h: N) -> Option<Self> {
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
    /// Construct a new grid from a nested iterator of cells.
    pub fn new<I: IntoIterator<Item=T>>(it: impl IntoIterator<Item=I>) -> Self {
        let mut v = Vec::new();
        let mut it = it.into_iter();
        let first = it.next().unwrap();
        v.extend(first);
        let w = v.len();
        for item in it {
            v.extend(item);
            assert_eq!(v.len() % w, 0);
        }
        Self { w, h: v.len() / w, v }
    }

    /// Return the total number of cells.
    pub fn len(&self) -> usize {
        self.w * self.h
    }

    /// Return the width of the grid.
    pub fn width(&self) -> usize {
        self.w
    }

    /// Return the height of the grid.
    pub fn height(&self) -> usize {
        self.h
    }

    /// Iterator over all rows of the grid.
    pub fn rows(&self) -> impl Iterator<Item=&[T]> {
        self.v.chunks(self.w)
    }

    /// Get a reference to the item at a given position.
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

    /// Get a mutable reference to the item at a given position.
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

    /// Return the position of the center cell of the grid (or, if width/height
    /// are even, the cell just left/above of center).
    pub fn center<N>(&self) -> Pos<N>
    where N: Integer + Copy + FromPrimitive + ToPrimitive
    {
        Pos(N::from_usize(self.w / 2).unwrap(), N::from_usize(self.h / 2).unwrap())
    }

    /// Iterate over all positions in the grid.
    pub fn positions<N>(&self) -> impl Iterator<Item=Pos<N>> + 'static
    where N: Integer + Copy + FromPrimitive + ToPrimitive
    {
        (0..self.h).cartesian_product(0..self.w).map(|(y, x)| {
            Pos(N::from_usize(x).unwrap(), N::from_usize(y).unwrap())
        })
    }

    /// Find the position of the first cell (going by columns, then rows)
    /// satisfying the predicate.
    pub fn find_pos(&self, mut f: impl FnMut(&T) -> bool) -> Option<Pos<usize>> {
        self.positions().find(|&p| f(&self[p]))
    }

    /// Count the number of cells satisfying the predicate.
    pub fn count(&self, mut f: impl FnMut(&T) -> bool) -> usize {
        self.v.iter().filter(|t| f(*t)).count()
    }

    /// Iterate over all orthogonal neighbors of the cell.
    pub fn neighbors<N>(&self, pos: Pos<N>) -> impl Iterator<Item=Pos<N>> + 'static
    where N: Integer + Copy + FromPrimitive + ToPrimitive + 'static
    {
        let (w, h) = (N::from_usize(self.w).expect("invalid width"),
                      N::from_usize(self.h).expect("invalid height"));
        Dir::iter().flat_map(move |d| pos.maybe_to(d, w, h))
    }

    /// Iterate over all orthogonal and diagonal neighbors of the cell.
    pub fn neighbors_diag<N>(&self, pos: Pos<N>) -> impl Iterator<Item=Pos<N>> + 'static
    where N: Integer + Copy + FromPrimitive + ToPrimitive + 'static
    {
        let (w, h) = (N::from_usize(self.w).expect("invalid width"),
                      N::from_usize(self.h).expect("invalid height"));
        Dir::iter().flat_map(move |d| pos.maybe_to(d, w, h)).chain(
            Dir::iter().flat_map(move |d| pos.maybe_to(d, w, h)
                .and_then(|p| p.maybe_to(d.left(), w, h)))
        )
    }

    /// Map the grid by applying a function to every cell.
    pub fn map<U>(&self, f: impl FnMut(&T) -> U) -> Grid<U> {
        Grid {
            w: self.w,
            h: self.h,
            v: self.v.iter().map(f).collect()
        }
    }

    /// Print the grid by formatting each cell with the given function.
    pub fn debug(&self, mut f: impl FnMut(&T) -> char) {
        for row in self.rows() {
            for item in row {
                print!("{}", f(item));
            }
            println!();
        }
    }
}

impl<T: Clone> Grid<T> {
    /// Create a new grid where every cell is the given value.
    pub fn fill(value: T, w: usize, h: usize) -> Self {
        let v = vec![value; w*h];
        Self { w, h, v }
    }

    /// Enlarge the grid by `n` cells on every side.
    pub fn enlarge(&mut self, n: usize, el: T) {
        let mut new_v = vec![el.clone(); (self.w + 2*n) * n];
        for row in self.rows() {
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
