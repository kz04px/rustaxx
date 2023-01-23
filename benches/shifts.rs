#![feature(test)]

extern crate test;

use ataxx::bitboard::Bitboard;
use std::process::Termination;
use test::Bencher;

#[bench]
fn run_singles(b: &mut Bencher) -> impl Termination {
    b.iter(|| {
        for sq in 0..49 {
            std::hint::black_box(Bitboard::from_index(sq).singles());
        }
    });
}

#[bench]
fn run_doubles(b: &mut Bencher) -> impl Termination {
    b.iter(|| {
        for sq in 0..49 {
            std::hint::black_box(Bitboard::from_index(sq).doubles());
        }
    });
}
