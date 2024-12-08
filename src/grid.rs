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
