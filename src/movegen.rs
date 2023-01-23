use crate::{lut::LUT_DOUBLES, moves::Move, position::Position};

impl Position {
    #[must_use]
    pub fn legal_moves(&self) -> Vec<Move> {
        if self.is_gameover() {
            return Vec::new();
        }

        self.pseudolegal_moves()
    }

    #[must_use]
    pub fn pseudolegal_moves(&self) -> Vec<Move> {
        let mut vec = Vec::with_capacity(200);

        // Single moves
        for sq in self.get_us().singles() & self.get_empty() {
            vec.push(Move::Drop(sq));
        }

        // Double moves
        for from in self.get_us() {
            for to in LUT_DOUBLES[from.0 as usize] & self.get_empty() {
                vec.push(Move::Jump(from, to));
            }
        }

        vec
    }
}
