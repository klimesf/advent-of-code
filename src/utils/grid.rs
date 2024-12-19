use std::ops::{Add, AddAssign, Index, IndexMut, Mul};

pub const UP: P = P::new(-1, 0);
pub const RIGHT: P = P::new(0, 1);
pub const DOWN: P = P::new(1, 0);
pub const LEFT: P = P::new(0, -1);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct P {
    pub x: i32,
    pub y: i32,
}

impl P {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        P { x, y }
    }

    #[inline]
    pub fn manhattan_distance(&self, other: &P) -> i32 {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }
}

impl Add for P {
    type Output = P;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        P::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for P {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for P {
    type Output = P;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        P::new(self.x * rhs, self.y * rhs)
    }
}

#[cfg(test)]
mod point_tests {
    use crate::utils::grid::{P};

    #[test]
    fn add_works() {
        assert_eq!(P::new(2, 3), P::new(1, 1) + P::new(1, 2));
        assert_eq!(P::new(1, 1), P::new(-1, -1) + P::new(2, 2));
    }

    #[test]
    fn add_assign_works() {
        let mut p = P::new(1, 1);
        p += P::new(1, 2);
        assert_eq!(P::new(2, 3), p);
    }

    #[test]
    fn mul_works() {
        assert_eq!(P::new(2, 4), P::new(1, 2) * 2);
        assert_eq!(P::new(-2, -4), P::new(1, 2) * -2);
    }
}


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Grid<T> {
    pub x_len: i32,
    pub y_len: i32,
    pub items: Vec<T>,
}

impl Grid<usize> {
    #[inline]
    pub fn new_usize(x_len: i32, y_len: i32, val: usize) -> Self {
        Grid { x_len, y_len, items: vec![val; (x_len * y_len) as usize] }
    }
}

impl Grid<char> {
    #[inline]
    pub fn parse(input: &str) -> Self {
        let items = input.lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .flatten().collect();
        let x_len = input.lines().count() as i32;
        let y_len = if x_len > 0 { input.len() as i32 / x_len } else { 0 };
        Grid { x_len, y_len, items }
    }

    #[inline]
    pub fn new(x_len: i32, y_len: i32, c: char) -> Self {
        Grid { x_len, y_len, items: vec![c; (x_len * y_len) as usize] }
    }

    #[inline]
    pub fn find_first(&self, c: char) -> Option<P> {
        if let Some(pos) = self.items.iter().position(|item| item == &c) {
            let x = pos as i32 / self.y_len;
            let y = pos as i32 % self.y_len;
            Some(P::new(x, y))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn render(&self) {
        for x in 0..self.x_len {
            for y in 0..self.y_len {
                print!("{}", self[(x, y)]);
            }
            println!();
        }
        println!();
    }
}

impl <T> Grid<T> {
    #[inline]
    pub fn contains(&self, p: &P) -> bool {
        p.x >= 0 && p.x < self.x_len && p.y >= 0 && p.y < self.y_len
    }
}

impl<T> Index<P> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, coord: P) -> &Self::Output {
        &self.items[(coord.x * self.y_len + coord.y) as usize]
    }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, coord: (i32, i32)) -> &Self::Output {
        &self.items[(coord.0 * self.y_len + coord.1) as usize]
    }
}

impl<T> IndexMut<P> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, coord: P) -> &mut Self::Output {
        &mut self.items[(coord.x * self.y_len + coord.y) as usize]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, coord: (i32, i32)) -> &mut Self::Output {
        &mut self.items[(coord.0 * self.y_len + coord.1) as usize]
    }
}

impl IntoIterator for Grid<char> {
    type Item = (i32, i32, char);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter().enumerate().map(|(i, c)| (
            i as i32 / self.y_len,
            i as i32 % self.y_len,
            c
        )).collect::<Vec<Self::Item>>().into_iter()
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::utils::grid::{P, Grid};

    #[test]
    fn parse_works() {
        let empty_map = Grid::parse("");
        assert_eq!(empty_map, Grid { x_len: 0, y_len: 0, items: vec![] });

        let basic_map = Grid::parse("#####\n#   #\n#   #\n#####");
        assert_eq!(4, basic_map.x_len);
        assert_eq!(5, basic_map.y_len);
        assert_eq!(20, basic_map.items.len());
    }

    #[test]
    fn index_works() {
        let map = Grid::parse("#####\n# x #\n#   #\n#####");
        assert_eq!('#', map[P::new(0, 0)]);
        assert_eq!('#', map[P::new(0, 4)]);
        assert_eq!('#', map[P::new(1, 0)]);
        assert_eq!(' ', map[P::new(1, 1)]);
        assert_eq!('x', map[P::new(1, 2)]);
        assert_eq!('#', map[P::new(3, 4)]);
        assert_eq!('#', map[(0, 0)]);
        assert_eq!('#', map[(0, 4)]);
        assert_eq!('#', map[(1, 0)]);
        assert_eq!(' ', map[(1, 1)]);
        assert_eq!('x', map[(1, 2)]);
        assert_eq!('#', map[(3, 4)]);
    }

    #[test]
    fn find_first_works() {
        let map = Grid::parse("#####\n# x #\n# y #\n#####");
        assert_eq!(Some(P::new(1, 2)), map.find_first('x'));
        assert_eq!(Some(P::new(2, 2)), map.find_first('y'));
        assert_eq!(None, map.find_first('X'));
    }
}
