use crate::{bitboard::Bitboard, moves::Move, position::Position, side::Side};

impl Position {
    pub fn makemove(&mut self, mv: &Move) {
        match mv {
            Move::Pass => {
                self.halfmoves += 1;
                self.fullmoves += (self.turn == Side::White) as u32;
                self.turn = !self.turn;
            }
            Move::Drop(sq) => {
                let to_bb = Bitboard::from_square(*sq);
                let captured = to_bb.singles() & self.get_them();

                self.pieces[self.turn as usize] ^= to_bb | captured;
                self.pieces[!self.turn as usize] ^= captured;

                self.halfmoves = 0;
                self.fullmoves += (self.turn == Side::White) as u32;
                self.turn = !self.turn;
            }
            Move::Jump(fr, to) => {
                let to_bb = Bitboard::from_square(*to);
                let from_bb = Bitboard::from_square(*fr);
                let captured = to_bb.singles() & self.get_them();

                self.pieces[self.turn as usize] ^= to_bb | from_bb | captured;
                self.pieces[!self.turn as usize] ^= captured;

                self.halfmoves += 1;
                self.fullmoves += (self.turn == Side::White) as u32;
                self.turn = !self.turn;
            }
        }
    }
}
