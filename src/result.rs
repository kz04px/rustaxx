use crate::position::Position;

#[derive(Debug, PartialEq)]
pub enum GameResult {
    BlackWin,
    WhiteWin,
    Draw,
}

impl Position {
    #[must_use]
    pub fn get_result(&self) -> Option<GameResult> {
        let num_black = self.get_black().count();
        let num_white = self.get_white().count();
        let reachable = self.get_both().reach() & self.get_empty();

        if num_black > 0 && num_white == 0 {
            Some(GameResult::BlackWin)
        } else if num_black == 0 && num_white > 0 {
            Some(GameResult::WhiteWin)
        } else if num_black == 0 && num_white == 0 {
            Some(GameResult::Draw)
        } else if self.halfmoves >= 100 {
            Some(GameResult::Draw)
        } else if reachable.is_occupied() {
            None
        } else if num_black > num_white {
            Some(GameResult::BlackWin)
        } else if num_black < num_white {
            Some(GameResult::WhiteWin)
        } else {
            Some(GameResult::Draw)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{position::Position, result::GameResult};

    #[test]
    fn game_results() {
        let tests = [
            // Game not over
            ("startpos", None),
            ("x5o/7/7/7/7/7/o5x x 0 1", None),
            ("x5o/7/7/7/7/7/o5x o 0 1", None),
            ("x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1", None),
            ("x5o/7/2-1-2/7/2-1-2/7/o5x o 0 1", None),
            ("x5o/7/2-1-2/7/2-1-2/7/o5x x 20 40", None),
            ("x5o/7/2-1-2/7/2-1-2/7/o5x o 20 40", None),
            ("7/7/7/7/ooooooo/ooooooo/xxxxxxx x 0 1", None),
            ("7/7/7/7/ooooooo/ooooooo/xxxxxxx o 0 1", None),
            ("7/7/7/7/xxxxxxx/xxxxxxx/ooooooo x 0 1", None),
            ("7/7/7/7/xxxxxxx/xxxxxxx/ooooooo o 0 1", None),
            // No material left
            ("x6/7/7/7/7/7/7 x 0 1", Some(GameResult::BlackWin)),
            ("x6/7/7/7/7/7/7 o 0 1", Some(GameResult::BlackWin)),
            ("o6/7/7/7/7/7/7 x 0 1", Some(GameResult::WhiteWin)),
            ("o6/7/7/7/7/7/7 o 0 1", Some(GameResult::WhiteWin)),
            // No material at all
            ("7/7/7/7/7/7/7 x 0 1", Some(GameResult::Draw)),
            ("7/7/7/7/7/7/7 o 0 1", Some(GameResult::Draw)),
            // Halfmove counter
            ("x5o/7/7/7/7/7/o5x x 99 1", None),
            ("x5o/7/7/7/7/7/o5x o 99 1", None),
            ("x5o/7/7/7/7/7/o5x x 100 1", Some(GameResult::Draw)),
            ("x5o/7/7/7/7/7/o5x o 100 1", Some(GameResult::Draw)),
            ("x5o/7/7/7/7/7/o5x x 200 1", Some(GameResult::Draw)),
            ("x5o/7/7/7/7/7/o5x o 200 1", Some(GameResult::Draw)),
            ("xx4o/7/7/7/7/7/o5x x 100 1", Some(GameResult::Draw)),
            ("xx4o/7/7/7/7/7/o5x o 100 1", Some(GameResult::Draw)),
            ("x4oo/7/7/7/7/7/o5x x 100 1", Some(GameResult::Draw)),
            ("x4oo/7/7/7/7/7/o5x o 100 1", Some(GameResult::Draw)),
            // No material priority over halfmove counter
            ("x6/7/7/7/7/7/7 x 100 1", Some(GameResult::BlackWin)),
            ("x6/7/7/7/7/7/7 o 100 1", Some(GameResult::BlackWin)),
            ("6o/7/7/7/7/7/7 x 100 1", Some(GameResult::WhiteWin)),
            ("6o/7/7/7/7/7/7 o 100 1", Some(GameResult::WhiteWin)),
            // No moves left
            ("xo--3/----3/----3/7/7/7/7 x 0 1", Some(GameResult::Draw)),
            ("xo--3/----3/----3/7/7/7/7 o 0 1", Some(GameResult::Draw)),
            (
                "xxo--2/-----2/-----2/7/7/7/7 x 0 1",
                Some(GameResult::BlackWin),
            ),
            (
                "xxo--2/-----2/-----2/7/7/7/7 o 0 1",
                Some(GameResult::BlackWin),
            ),
            (
                "xoo--2/-----2/-----2/7/7/7/7 x 0 1",
                Some(GameResult::WhiteWin),
            ),
            (
                "xoo--2/-----2/-----2/7/7/7/7 o 0 1",
                Some(GameResult::WhiteWin),
            ),
            // Board full
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxxooo/ooooooo/ooooooo/ooooooo x 0 1",
                Some(GameResult::BlackWin),
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxxooo/ooooooo/ooooooo/ooooooo o 0 1",
                Some(GameResult::BlackWin),
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxoooo/ooooooo/ooooooo/ooooooo x 0 1",
                Some(GameResult::WhiteWin),
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxoooo/ooooooo/ooooooo/ooooooo o 0 1",
                Some(GameResult::WhiteWin),
            ),
            (
                "-xxxxxx/xxxxxxx/xxxxxxx/xxxxooo/ooooooo/ooooooo/oooooo- x 0 1",
                Some(GameResult::BlackWin),
            ),
            (
                "-xxxxxx/xxxxxxx/xxxxxxx/xxxxooo/ooooooo/ooooooo/oooooo- o 0 1",
                Some(GameResult::BlackWin),
            ),
        ];

        for (fen, result) in tests {
            let pos = Position::from_fen(fen);
            assert_eq!(pos.get_result(), result);
        }
    }
}
