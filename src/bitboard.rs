use crate::squares::Square;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (0..7).rev() {
            for x in 0..7 {
                let idx = 7 * y + x;
                if self.is_set(idx) {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0 & 0x1ffffffffffff)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.0 ^= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 | rhs.0)
    }
}

impl Bitboard {
    #[must_use]
    pub const fn new(mask: u64) -> Self {
        Self(mask)
    }

    #[must_use]
    pub const fn from_index(idx: u8) -> Self {
        Self(1u64 << idx)
    }

    #[must_use]
    pub const fn from_square(sq: Square) -> Self {
        Self(1u64 << sq.0)
    }

    #[must_use]
    pub const fn from_full() -> Self {
        Self(0x1ffffffffffff)
    }

    #[must_use]
    pub const fn from_empty() -> Self {
        Self(0x0)
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn is_occupied(&self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.0 == 0x1ffffffffffff
    }

    #[must_use]
    pub const fn is_set(&self, idx: u8) -> bool {
        ((self.0 >> idx) & 1) == 1
    }

    #[must_use]
    pub const fn north(&self) -> Self {
        Self((self.0 << 7) & 0x1ffffffffffff)
    }

    #[must_use]
    pub const fn south(&self) -> Self {
        Self(self.0 >> 7)
    }

    #[must_use]
    pub const fn east(&self) -> Self {
        Self((self.0 << 1) & 0x1fbf7efdfbf7e)
    }

    #[must_use]
    pub const fn west(&self) -> Self {
        Self((self.0 >> 1) & 0xfdfbf7efdfbf)
    }

    #[must_use]
    pub const fn singles(&self) -> Self {
        Self(
            (self.0 << 7 | self.0 >> 7) & 0x1ffffffffffff
                | (self.0 >> 1 | self.0 >> 8 | self.0 << 6) & 0xfdfbf7efdfbf
                | (self.0 << 1 | self.0 >> 6 | self.0 << 8) & 0x1fbf7efdfbf7e,
        )
    }

    #[must_use]
    pub const fn doubles(&self) -> Self {
        Self(
            // right 2
            ((self.0 << 16 | self.0 << 9 | self.0 << 2 | self.0 >> 5 | self.0 >> 12) & 0x1f3e7cf9f3e7c) |
            // right 1
            ((self.0 << 15 | self.0 >> 13) & 0x1fbf7efdfbf7e) |
            // centre
            ((self.0 << 14 | self.0 >> 14) & 0x1ffffffffffff) |
            // left 1
            ((self.0 << 13 | self.0 >> 15) & 0xfdfbf7efdfbf) |
            // left 2
            ((self.0 << 12 | self.0 << 5 | self.0 >> 2 | self.0 >> 9 | self.0 >> 16) & 0x7cf9f3e7cf9f),
        )
    }

    #[must_use]
    pub fn both(&self) -> Self {
        self.singles() | self.doubles()
    }

    #[must_use]
    pub fn reach(&self) -> Self {
        self.singles() | self.doubles()
    }

    #[must_use]
    pub const fn count(&self) -> i32 {
        self.0.count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::bitboard::Bitboard;

    #[test]
    fn count_tests() {
        assert!(Bitboard(0).count() == 0);
        assert!(Bitboard(1).count() == 1);
        assert!(Bitboard(2).count() == 1);
        assert!(Bitboard(3).count() == 2);
    }

    #[test]
    fn empty() {
        assert!(Bitboard(0).is_empty());
        assert!(!Bitboard(1).is_empty());
    }

    #[test]
    fn bitxor() {
        assert!(Bitboard(0) ^ Bitboard(0) == Bitboard(0));
        assert!(Bitboard(1) ^ Bitboard(2) == Bitboard(3));
    }

    #[test]
    fn bitor() {
        assert!(Bitboard(1) | Bitboard(2) == Bitboard(3));
    }

    #[test]
    fn bitand() {
        assert!(Bitboard(1) & Bitboard(2) == Bitboard(0));
    }

    #[test]
    fn bitnot() {
        assert_eq!(!Bitboard(0), Bitboard(0x1ffffffffffff));
        assert_eq!(!Bitboard(0x1ffffffffffff), Bitboard(0));
    }

    #[test]
    fn north() {
        assert_eq!(Bitboard(0x0).north(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).north(), Bitboard(0x80));
        assert_eq!(Bitboard(0x7f).north(), Bitboard(0x3f80));
        assert_eq!(Bitboard(0x3f800000000).north(), Bitboard(0x1fc0000000000));
        assert_eq!(Bitboard(0x1fc0000000000).north(), Bitboard(0x0));
    }

    #[test]
    fn south() {
        assert_eq!(Bitboard(0x0).south(), Bitboard(0x0));
        assert_eq!(Bitboard(0x80).south(), Bitboard(0x1));
        assert_eq!(Bitboard(0x1fc000).south(), Bitboard(0x3f80));
        assert_eq!(Bitboard(0x3f80).south(), Bitboard(0x7f));
        assert_eq!(Bitboard(0x7f).south(), Bitboard(0x0));
    }

    #[test]
    fn east() {
        assert_eq!(Bitboard(0x0).east(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).east(), Bitboard(0x2));
    }

    #[test]
    fn west() {
        assert_eq!(Bitboard(0x0).west(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).west(), Bitboard(0x0));
    }

    #[test]
    fn singles() {
        assert_eq!(Bitboard(0x0).singles(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).singles(), Bitboard(0x182));
        assert_eq!(Bitboard(0x100).singles(), Bitboard(0x1c287));
    }

    #[test]
    fn doubles() {
        assert_eq!(Bitboard(0x0).doubles(), Bitboard(0x0));
        assert_eq!(Bitboard(0x1).doubles(), Bitboard(0x1c204));
        assert_eq!(Bitboard(0x100).doubles(), Bitboard(0x1e20408));
    }
}
