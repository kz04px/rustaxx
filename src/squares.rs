use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square(pub u8);

#[rustfmt::skip]
pub enum SquareIdx {
    A1, B1, C1, D1, E1, F1, G1,
    A2, B2, C2, D2, E2, F2, G2,
    A3, B3, C3, D3, E3, F3, G3,
    A4, B4, C4, D4, E4, F4, G4,
    A5, B5, C5, D5, E5, F5, G5,
    A6, B6, C6, D6, E6, F6, G6,
    A7, B7, C7, D7, E7, F7, G7,
}

impl Square {
    #[must_use]
    pub fn from_index(idx: SquareIdx) -> Self {
        Self(idx as u8)
    }

    #[must_use]
    pub fn from_str(sqstr: &str) -> Self {
        let x = sqstr.chars().nth(0).expect("Missing file character") as u8 - b'a';
        let y = sqstr.chars().nth(1).expect("Missing rank character") as u8 - b'1';
        let idx = 7 * y + x;
        Self(idx)
    }

    #[must_use]
    pub fn get_x(&self) -> u8 {
        self.0 % 7
    }

    #[must_use]
    pub fn get_y(&self) -> u8 {
        self.0 / 7
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (b'a' + self.get_x()) as char,
            (b'1' + self.get_y()) as char
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::squares::Square;

    #[test]
    fn string_parsing() {
        let tests = ["a1", "a7", "g1", "g7"];

        for sqstr in tests {
            let sq = Square::from_str(sqstr);
            assert_eq!(sq.to_string(), sqstr);
        }
    }
}
