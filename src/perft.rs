use crate::position::*;

impl Position {
    #[must_use]
    pub fn perft(&self, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        } else if self.is_gameover() {
            return 0;
        } else if depth == 1 {
            return self.count_pseudomoves();
        }

        let moves = self.pseudolegal_moves();
        let mut nodes: u64 = 0;

        for mv in &moves {
            let npos = self.after_move(mv);
            nodes += npos.perft(depth - 1);
        }

        if moves.is_empty() {
            let npos: Position = self.after_pass();
            return npos.perft(depth - 1);
        }

        nodes
    }
}

#[cfg(test)]
mod tests {
    use crate::position::Position;

    #[test]
    fn perft_suite() {
        let tests: [(&str, Vec<u64>); 20] = [
            ("7/7/7/7/7/7/7 x 0 1", vec![1, 0, 0, 0, 0, 0]),
            ("7/7/7/7/7/7/7 o 0 1", vec![1, 0, 0, 0, 0, 0]),
            (
                "x5o/7/7/7/7/7/o5x x 0 1",
                vec![1, 16, 256, 6460, 155888, 4752668],
            ),
            (
                "x5o/7/7/7/7/7/o5x o 0 1",
                vec![1, 16, 256, 6460, 155888, 4752668],
            ),
            (
                "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
                vec![1, 14, 196, 4184, 86528, 2266352],
            ),
            (
                "x5o/7/2-1-2/7/2-1-2/7/o5x o 0 1",
                vec![1, 14, 196, 4184, 86528, 2266352],
            ),
            (
                "x5o/7/2-1-2/3-3/2-1-2/7/o5x x 0 1",
                vec![1, 14, 196, 4100, 83104, 2114588],
            ),
            (
                "x5o/7/2-1-2/3-3/2-1-2/7/o5x o 0 1",
                vec![1, 14, 196, 4100, 83104, 2114588],
            ),
            (
                "x5o/7/3-3/2-1-2/3-3/7/o5x x 0 1",
                vec![1, 16, 256, 5948, 133264, 3639856],
            ),
            (
                "x5o/7/3-3/2-1-2/3-3/7/o5x o 0 1",
                vec![1, 16, 256, 5948, 133264, 3639856],
            ),
            (
                "7/7/7/7/ooooooo/ooooooo/xxxxxxx x 0 1",
                vec![1, 1, 75, 249, 14270, 452980],
            ),
            (
                "7/7/7/7/ooooooo/ooooooo/xxxxxxx o 0 1",
                vec![1, 75, 249, 14270, 452980],
            ),
            (
                "7/7/7/7/xxxxxxx/xxxxxxx/ooooooo x 0 1",
                vec![1, 75, 249, 14270, 452980],
            ),
            (
                "7/7/7/7/xxxxxxx/xxxxxxx/ooooooo o 0 1",
                vec![1, 1, 75, 249, 14270, 452980],
            ),
            (
                "7/7/7/2x1o2/7/7/7 x 0 1",
                vec![1, 23, 419, 7887, 168317, 4266992],
            ),
            (
                "7/7/7/2x1o2/7/7/7 o 0 1",
                vec![1, 23, 419, 7887, 168317, 4266992],
            ),
            ("x5o/7/7/7/7/7/o5x x 100 1", vec![1, 0, 0, 0, 0, 0]),
            ("x5o/7/7/7/7/7/o5x o 100 1", vec![1, 0, 0, 0, 0, 0]),
            (
                "7/7/7/7/-------/-------/x5o x 0 1",
                vec![1, 2, 4, 13, 30, 73, 174],
            ),
            (
                "7/7/7/7/-------/-------/x5o o 0 1",
                vec![1, 2, 4, 13, 30, 73, 174],
            ),
        ];

        for (fen, nodes) in tests {
            let pos = Position::from_fen(fen);
            for (depth, expected) in nodes.iter().enumerate() {
                let nodes = pos.perft(depth as u8);
                assert_eq!(&nodes, expected);
            }
        }
    }
}
