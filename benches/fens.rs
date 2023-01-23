#![feature(test)]

use ataxx::{bitboard::Bitboard, position::Position, side::Side};
use std::process::Termination;
use test::Bencher;

extern crate test;

const FENS: [&str; 20] = [
    "x5o/7/7/7/7/7/o5x x 0 1",
    "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
    "x5o/7/3-3/2-1-2/3-3/7/o5x x 0 1",
    "x2-2o/3-3/2---2/7/2---2/3-3/o2-2x x 0 1",
    "x2-2o/3-3/7/--3--/7/3-3/o2-2x x 0 1",
    "x1-1-1o/2-1-2/2-1-2/7/2-1-2/2-1-2/o1-1-1x x 0 1",
    "x5o/7/2-1-2/3-3/2-1-2/7/o5x x 0 1",
    "x5o/7/3-3/2---2/3-3/7/o5x x 0 1",
    "x5o/2-1-2/1-3-1/7/1-3-1/2-1-2/o5x x 0 1",
    "x5o/1-3-1/2-1-2/7/2-1-2/1-3-1/o5x x 0 1",
    "x-1-1-o/-1-1-1-/1-1-1-1/-1-1-1-/1-1-1-1/-1-1-1-/o-1-1-x x 0 1",
    "x-1-1-o/1-1-1-1/1-1-1-1/1-1-1-1/1-1-1-1/1-1-1-1/o-1-1-x x 0 1",
    "x1-1-1o/2-1-2/-------/2-1-2/-------/2-1-2/o1-1-1x x 0 1",
    "x5o/1-----1/1-3-1/1-1-1-1/1-3-1/1-----1/o5x x 0 1",
    "x-1-1-o/1-1-1-1/-1-1-1-/-1-1-1-/-1-1-1-/1-1-1-1/o-1-1-x/ x 0 1",
    "x5o/1--1--1/1--1--1/7/1--1--1/1--1--1/o5x x 0 1",
    "x-3-o/1-1-1-1/1-1-1-1/3-3/1-1-1-1/1-1-1-1/o-3-x x 0 1",
    "x2-2o/3-3/3-3/-------/3-3/3-3/o2-2x x 0 1",
    "x2-2o/2-1-2/1-3-1/-2-2-/1-3-1/2-1-2/o2-2x x 0 1",
    "x5o/6-/1-4-/-3--1/2-4/7/o-3-x x 0 1",
];

const POSITIONS: [Position; 20] = [
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x0),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x140050000),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x82820000),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x2041C0070408),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x20400C600408),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x50A140050A14),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x141050000),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x83820000),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0xA220088A00),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x11140051100),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0xAAAAAAAAAAAA),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0xA952A54A952A),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x50A7F29FCA14),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x1F225489F00),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0xA9555AB5552A),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x1B3600D9B00),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x8952A10A9522),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x20408FE20408),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x20A229288A08),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
    Position {
        pieces: [
            Bitboard(0x40000000040),
            Bitboard(0x1000000000001),
            Bitboard(0x20426210022),
        ],
        turn: Side::Black,
        halfmoves: 0,
        fullmoves: 1,
    },
];

#[bench]
fn set_fen(b: &mut Bencher) -> impl Termination {
    b.iter(|| {
        for fen in FENS {
            std::hint::black_box(Position::from_fen(fen));
        }
    });
}

#[bench]
fn get_fen(b: &mut Bencher) -> impl Termination {
    b.iter(|| {
        for pos in POSITIONS {
            std::hint::black_box(pos.get_fen());
        }
    });
}
