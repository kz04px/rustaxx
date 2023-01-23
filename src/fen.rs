use crate::{bitboard::Bitboard, position::Position, side::Side};

impl Position {
    #[must_use]
    pub fn from_fen(fen: &str) -> Self {
        if fen == "startpos" {
            return Position::from_fen("x5o/7/7/7/7/7/o5x x 0 1");
        }

        let mut pos = Position {
            pieces: [Bitboard(0), Bitboard(0), Bitboard(0)],
            turn: Side::Black,
            halfmoves: 0,
            fullmoves: 1,
        };
        let parts = fen.split(' ').collect::<Vec<&str>>();

        if parts.len() != 4 {
            panic!("FEN wrong format");
        }

        let mut x: i32 = 0;
        let mut y: i32 = 6;
        for c in parts[0].chars() {
            match c {
                'x' | 'X' => {
                    pos.pieces[Side::Black as usize] ^= Bitboard::from_index((7 * y + x) as u8);
                    x += 1;
                }
                'o' | 'O' => {
                    pos.pieces[Side::White as usize] ^= Bitboard::from_index((7 * y + x) as u8);
                    x += 1;
                }
                '-' => {
                    pos.pieces[2] ^= Bitboard::from_index((7 * y + x) as u8);
                    x += 1;
                }
                '1'..='7' => x += (c as u8 - b'0') as i32,
                '/' => {
                    x = 0;
                    y -= 1;
                }
                _ => panic!("Unrecognised FEN token"),
            }
        }

        match parts[1] {
            "x" | "X" => pos.turn = Side::Black,
            "o" | "O" => pos.turn = Side::White,
            _ => panic!("Unrecognised FEN token"),
        }

        match parts[2].parse::<u32>() {
            Ok(n) => pos.halfmoves = n,
            Err(_e) => panic!("Unrecognised FEN token"),
        }

        match parts[3].parse::<u32>() {
            Ok(n) => pos.fullmoves = n,
            Err(_e) => panic!("Unrecognised FEN token"),
        }

        pos
    }

    #[must_use]
    pub fn get_fen(&self) -> String {
        let mut fen = String::from("");

        for y in (0..=6).rev() {
            let mut spaces = 0;

            for x in 0..=6 {
                let idx = 7 * y + x;
                let bb = Bitboard::from_index(idx);

                if !(self.pieces[0] & bb).is_empty() {
                    if spaces > 0 {
                        fen += &spaces.to_string();
                        spaces = 0;
                    }
                    fen += "x";
                } else if !(self.pieces[1] & bb).is_empty() {
                    if spaces > 0 {
                        fen += &spaces.to_string();
                        spaces = 0;
                    }
                    fen += "o";
                } else if !(self.pieces[2] & bb).is_empty() {
                    if spaces > 0 {
                        fen += &spaces.to_string();
                        spaces = 0;
                    }
                    fen += "-";
                } else {
                    spaces += 1;
                }
            }

            if spaces > 0 {
                fen += &spaces.to_string();
            }

            if y > 0 {
                fen += "/";
            }
        }

        if self.turn == Side::Black {
            fen += " x";
        } else {
            fen += " o";
        }

        fen += " ";
        fen += &self.halfmoves.to_string();

        fen += " ";
        fen += &self.fullmoves.to_string();

        fen
    }
}

#[cfg(test)]
mod tests {
    use crate::position::Position;

    #[test]
    fn fen() {
        let tests: [&str; 8] = [
            "7/7/7/7/7/7/7 x 0 1",
            "7/7/7/7/7/7/7 o 0 1",
            "x5o/7/7/7/7/7/o5x x 0 1",
            "x5o/7/7/7/7/7/o5x o 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x o 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 20 40",
            "x5o/7/2-1-2/7/2-1-2/7/o5x o 20 40",
        ];

        for fen in tests {
            let pos = Position::from_fen(fen);
            assert_eq!(pos.get_fen(), fen);
        }
    }
}
