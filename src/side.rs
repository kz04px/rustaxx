use std::ops::{Index, IndexMut, Not};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Side {
    Black,
    White,
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::Black => Side::White,
            Side::White => Side::Black,
        }
    }
}

impl<T> Index<Side> for [T; 2] {
    type Output = T;

    fn index(&self, index: Side) -> &Self::Output {
        match index {
            Side::Black => &self[0],
            Side::White => &self[1],
        }
    }
}

impl<T> IndexMut<Side> for [T; 2] {
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        match index {
            Side::Black => &mut self[0],
            Side::White => &mut self[1],
        }
    }
}
