use crate::position::Position;

impl Position {
    #[must_use]
    pub fn is_gameover(&self) -> bool {
        self.get_black().is_empty()
            || self.get_white().is_empty()
            || self.halfmoves >= 100
            || (self.get_both().reach() & self.get_empty()).is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::position::Position;

    #[test]
    fn gameover_true() {
        let tests: [&str; 12] = [
            "7/7/7/7/7/7/7 x 0 1",
            "7/7/7/7/7/7/7 o 0 1",
            "7/7/7/7/7/7/x6 x 0 1",
            "7/7/7/7/7/7/x6 o 0 1",
            "7/7/7/7/7/7/o6 x 0 1",
            "7/7/7/7/7/7/o6 o 0 1",
            "x5o/7/7/7/7/7/o5x x 100 1",
            "x5o/7/7/7/7/7/o5x o 100 1",
            "7/7/7/7/-------/-------/ooooxxx x 0 1",
            "7/7/7/7/-------/-------/ooooxxx o 0 1",
            "xo--3/----3/----3/7/7/7/7 x 0 1",
            "xo--3/----3/----3/7/7/7/7 o 0 1",
        ];

        for fen in tests {
            let pos = Position::from_fen(fen);
            assert!(pos.is_gameover());
        }
    }

    #[test]
    fn gameover_false() {
        let tests: [&str; 10] = [
            "x5o/7/7/7/7/7/o5x x 0 1",
            "x5o/7/7/7/7/7/o5x o 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x o 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 20 40",
            "x5o/7/2-1-2/7/2-1-2/7/o5x o 20 40",
            "7/7/7/7/ooooooo/ooooooo/xxxxxxx x 0 1",
            "7/7/7/7/ooooooo/ooooooo/xxxxxxx o 0 1",
            "7/7/7/7/xxxxxxx/xxxxxxx/ooooooo x 0 1",
            "7/7/7/7/xxxxxxx/xxxxxxx/ooooooo o 0 1",
        ];

        for fen in tests {
            let pos = Position::from_fen(fen);
            assert!(!pos.is_gameover());
        }
    }
}
