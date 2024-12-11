use std::{fmt::Debug, hash::Hash};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2<I>(pub I, pub I);

impl Vec2<usize> {
    pub fn try_add(&self, d: Vec2<isize>, max: Vec2<usize>) -> Option<Vec2<usize>> {
        let x = if d.0.is_negative() {
            self.0.checked_sub(d.0.wrapping_abs() as usize)
        } else {
            self.0.checked_add(d.0 as usize)
        }?;

        let y = if d.1.is_negative() {
            self.1.checked_sub(d.1.wrapping_abs() as usize)
        } else {
            self.1.checked_add(d.1 as usize)
        }?;

        if x > max.0 || y > max.1 {
            return None;
        }

        Some(Vec2(x, y))
    }

    pub fn try_subtract(&self, d: Vec2<isize>, max: Vec2<usize>) -> Option<Vec2<usize>> {
        let x = if d.0.is_negative() {
            self.0.checked_add(d.0.wrapping_abs() as usize)
        } else {
            self.0.checked_sub(d.0 as usize)
        }?;

        let y = if d.1.is_negative() {
            self.1.checked_add(d.1.wrapping_abs() as usize)
        } else {
            self.1.checked_sub(d.1 as usize)
        }?;

        if x > max.0 || y > max.1 {
            return None;
        }

        Some(Vec2(x, y))
    }

    pub fn subtract(&self, d: Vec2<usize>) -> Vec2<isize> {
        Vec2(
            (self.0 as isize) - (d.0 as isize),
            (self.1 as isize) - (d.1 as isize),
        )
    }
}

impl Vec2<isize> {
    pub fn rotate_clockwise(&self) -> Self {
        Self(-self.1, self.0)
    }
}

impl<I: Debug> Debug for Vec2<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.0, self.1)
    }
}

pub struct Grid2D<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid2D<T> {
    pub fn max(&self) -> Vec2<usize> {
        Vec2(self.width - 1, self.height - 1)
    }

    pub fn get(&self, p: Vec2<usize>) -> Option<&T> {
        if p.0 >= self.width || p.1 >= self.height {
            return None;
        }

        self.data.get(self.idx(p))
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vec2<usize>, &T)> {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| Vec2(x, y)))
            .map(|p| (p, unsafe { self.get_unchecked(p) }))
    }

    unsafe fn get_unchecked(&self, p: Vec2<usize>) -> &T {
        self.data.get_unchecked(self.idx(p))
    }

    fn idx(&self, Vec2(x, y): Vec2<usize>) -> usize {
        y * self.width + x
    }
}

impl<I: Iterator<Item = char>> From<I> for Grid2D<char> {
    fn from(value: I) -> Self {
        let mut width = 0;
        let mut height = 1;
        let mut data = vec![];

        for c in value {
            match c {
                '\n' => {
                    height += 1;
                }
                _ => {
                    if height == 1 {
                        width += 1;
                    }
                    data.push(c);
                }
            }
        }

        Self {
            width,
            height,
            data,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grid_get() {
        let grid = Grid2D::from("abcd\nefgh\nijkl".chars());

        assert_eq!(grid.get(Vec2(0, 0)), Some(&'a'));
        assert_eq!(grid.get(Vec2(3, 1)), Some(&'h'));
        assert_eq!(grid.get(Vec2(1, 2)), Some(&'j'));
        assert_eq!(grid.get(Vec2(1, 4)), None);
        assert_eq!(grid.get(Vec2(4, 1)), None);
    }

    #[test]
    fn grid_iter() {
        let grid = Grid2D::from("abcd\nefgh\nijkl".chars());

        let mut iter = grid.iter();

        assert_eq!(iter.next(), Some((Vec2(0, 0), &'a')));
        assert_eq!(iter.next(), Some((Vec2(1, 0), &'b')));
        assert_eq!(iter.next(), Some((Vec2(2, 0), &'c')));
        assert_eq!(iter.next(), Some((Vec2(3, 0), &'d')));
        assert_eq!(iter.next(), Some((Vec2(0, 1), &'e')));
    }
}
