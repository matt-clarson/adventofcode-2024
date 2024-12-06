use std::{fmt::Debug, hash::Hash};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2<I>(pub I, pub I);

impl Vec2<usize> {
    pub fn try_add(&self, d: Vec2<isize>, max: Vec2<usize>) -> Option<Vec2<usize>> {
        if self.0 == 0 && d.0 == -1
            || self.0 == max.0 && d.0 == 1
            || self.1 == 0 && d.1 == -1
            || self.1 == max.1 && d.1 == 1
        {
            None
        } else {
            Some(Vec2(
                (self.0 as isize + d.0) as usize,
                (self.1 as isize + d.1) as usize,
            ))
        }
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
