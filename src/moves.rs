use crate::squares::Square;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Move {
    Pass,
    Drop(Square),
    Jump(Square, Square),
}

impl Move {
    #[must_use]
    pub fn from_string(movestr: &str) -> Move {
        if movestr == "0000" {
            return Move::Pass;
        }

        if movestr.len() == 2 {
            let to = Square::from_str(&movestr[0..2]);
            Move::Drop(to)
        } else if movestr.len() == 4 {
            let from = Square::from_str(&movestr[0..2]);
            let to = Square::from_str(&movestr[2..4]);
            Move::Jump(from, to)
        } else {
            panic!("Uh oh");
        }
    }

    #[must_use]
    pub const fn is_pass(&self) -> bool {
        matches!(self, Move::Pass)
    }

    #[must_use]
    pub const fn is_single(&self) -> bool {
        matches!(self, Move::Drop(_))
    }

    #[must_use]
    pub const fn is_double(&self) -> bool {
        matches!(self, Move::Jump(_, _))
    }

    #[must_use]
    pub const fn from(&self) -> Option<Square> {
        match self {
            Move::Pass => None,
            Move::Drop(_) => None,
            Move::Jump(fr, _) => Some(*fr),
        }
    }

    #[must_use]
    pub const fn to(&self) -> Option<Square> {
        match self {
            Move::Pass => None,
            Move::Drop(sq) => Some(*sq),
            Move::Jump(_, to) => Some(*to),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Pass => write!(f, "0000"),
            Move::Drop(sq) => write!(f, "{}", sq),
            Move::Jump(fr, to) => write!(f, "{}{}", fr, to),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        moves::Move,
        squares::{Square, SquareIdx},
    };

    #[test]
    fn from() {
        assert_eq!(Move::Pass.from(), None);
        assert_eq!(Move::Drop(Square::from_index(SquareIdx::A1)).from(), None);
        assert_eq!(
            Move::Jump(
                Square::from_index(SquareIdx::A1),
                Square::from_index(SquareIdx::A3)
            )
            .from(),
            Some(Square::from_index(SquareIdx::A1))
        );
    }

    #[test]
    fn to() {
        assert_eq!(Move::Pass.to(), None);
        assert_eq!(
            Move::Drop(Square::from_index(SquareIdx::A1)).to(),
            Some(Square::from_index(SquareIdx::A1))
        );
        assert_eq!(
            Move::Jump(
                Square::from_index(SquareIdx::A1),
                Square::from_index(SquareIdx::A3)
            )
            .to(),
            Some(Square::from_index(SquareIdx::A3))
        );
    }

    #[test]
    fn is_single() {
        assert!(Move::Pass.is_pass());
        assert!(!Move::Pass.is_single());
        assert!(!Move::Pass.is_double());

        assert!(!Move::Drop(Square::from_index(SquareIdx::A1)).is_pass());
        assert!(Move::Drop(Square::from_index(SquareIdx::A1)).is_single());
        assert!(!Move::Drop(Square::from_index(SquareIdx::A1)).is_double());

        assert!(!Move::Jump(
            Square::from_index(SquareIdx::A1),
            Square::from_index(SquareIdx::A3)
        )
        .is_pass());
        assert!(!Move::Jump(
            Square::from_index(SquareIdx::A1),
            Square::from_index(SquareIdx::A3)
        )
        .is_single());
        assert!(Move::Jump(
            Square::from_index(SquareIdx::A1),
            Square::from_index(SquareIdx::A3)
        )
        .is_double());
    }

    #[test]
    fn string_parsing() {
        let tests = ["0000", "a1", "g7", "a1c3", "g7e5"];

        for movestr in tests {
            let mv = Move::from_string(movestr);
            assert_eq!(mv.to_string(), movestr);
        }
    }
}
