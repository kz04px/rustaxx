use crate::{bitboard::Bitboard, lut::LUT_DOUBLES, moves::Move, piece::Piece, side::Side};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position {
    pub pieces: [Bitboard; 3],
    pub turn: Side,
    pub halfmoves: u32,
    pub fullmoves: u32,
}

impl Position {
    #[must_use]
    pub fn after_move(&self, mv: &Move) -> Self {
        let mut npos = *self;
        npos.makemove(mv);
        npos
    }

    #[must_use]
    pub fn after_pass(&self) -> Self {
        let mut npos = *self;
        npos.makemove(&Move::Pass);
        npos
    }

    #[must_use]
    pub fn get_black(&self) -> Bitboard {
        self.pieces[Piece::Black]
    }

    #[must_use]
    pub fn get_white(&self) -> Bitboard {
        self.pieces[Piece::White]
    }

    #[must_use]
    pub fn get_walls(&self) -> Bitboard {
        self.pieces[Piece::Wall]
    }

    #[must_use]
    pub fn get_both(&self) -> Bitboard {
        self.get_black() | self.get_white()
    }

    #[must_use]
    pub fn get_empty(&self) -> Bitboard {
        !(self.get_black() | self.get_white() | self.get_walls())
    }

    #[must_use]
    pub fn get_us(&self) -> Bitboard {
        match self.turn {
            Side::Black => self.get_black(),
            Side::White => self.get_white(),
        }
    }

    #[must_use]
    pub fn get_them(&self) -> Bitboard {
        match self.turn {
            Side::Black => self.get_white(),
            Side::White => self.get_black(),
        }
    }

    #[must_use]
    pub fn is_legal_move(&self, mv: &Move) -> bool {
        match mv {
            Move::Pass => self.can_pass(),
            _ => self.pseudolegal_moves().contains(mv),
        }
    }

    #[must_use]
    pub fn count_pseudomoves(&self) -> u64 {
        let mut nodes = 0;

        // Single moves
        nodes += (self.get_us().singles() & self.get_empty()).count();

        // Double moves
        for from in self.get_us() {
            nodes += (LUT_DOUBLES[from.0 as usize] & self.get_empty()).count();
        }

        // Pass
        if nodes == 0 {
            nodes = 1;
        }

        nodes as u64
    }

    #[must_use]
    pub fn count_moves(&self) -> u64 {
        if self.is_gameover() {
            return 0;
        }

        self.count_pseudomoves()
    }

    #[must_use]
    pub fn can_pass(&self) -> bool {
        if self.is_gameover() {
            return false;
        }

        self.get_us().reach() & self.get_empty() == Bitboard(0)
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        if self.get_black() & self.get_white() != Bitboard(0) {
            return false;
        }

        if self.get_black() & self.get_walls() != Bitboard(0) {
            return false;
        }

        if self.get_white() & self.get_walls() != Bitboard(0) {
            return false;
        }

        true
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut height = 6;
        let mut width = 6;

        loop {
            let mut yeet = true;
            for x in 0..7 {
                let idx = 7 * height + x;
                if !self.get_walls().is_set(idx) {
                    yeet = false;
                    break;
                }
            }
            if !yeet {
                break;
            }
            height -= 1;
        }

        loop {
            let mut yeet = true;
            for y in 0..7 {
                let idx = 7 * y + width;
                if !self.get_walls().is_set(idx) {
                    yeet = false;
                    break;
                }
            }
            if !yeet {
                break;
            }
            width -= 1;
        }

        for y in (0..=height).rev() {
            for x in 0..=width {
                let idx = 7 * y + x;
                if self.get_black().is_set(idx) {
                    write!(f, "x")?;
                } else if self.get_white().is_set(idx) {
                    write!(f, "o")?;
                } else if self.get_walls().is_set(idx) {
                    write!(f, " ")?;
                } else {
                    write!(f, "-")?;
                }
            }
            writeln!(f)?;
        }

        match self.turn {
            Side::Black => writeln!(f, "Turn: x")?,
            Side::White => writeln!(f, "Turn: o")?,
        }

        Ok(())
    }
}
