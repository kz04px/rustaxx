use ataxx::position::Position;
use std::time::Instant;

fn main() {
    let pos = Position::from_fen("startpos");

    for i in 0..=7 {
        let start = Instant::now();
        let nodes = pos.perft(i);
        let duration = start.elapsed();
        let nps = nodes as f64 / duration.as_secs_f64();
        println!(
            "info depth {} nodes {} time {:?} nps {}",
            i,
            nodes,
            duration.as_millis(),
            nps as u64
        );
    }
}
