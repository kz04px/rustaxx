use std::ops::{Index, IndexMut};

pub enum Piece {
    Black,
    White,
    Wall,
}

impl<T> Index<Piece> for [T; 3] {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        match index {
            Piece::Black => &self[0],
            Piece::White => &self[1],
            Piece::Wall => &self[2],
        }
    }
}

impl<T> IndexMut<Piece> for [T; 3] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        match index {
            Piece::Black => &mut self[0],
            Piece::White => &mut self[1],
            Piece::Wall => &mut self[2],
        }
    }
}
