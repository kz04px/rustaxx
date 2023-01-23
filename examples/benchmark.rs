use ataxx::position::Position;
use std::time::Instant;

fn main() {
    let fens = [
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
    let depth = 7;
    let mut total_nodes = 0;
    let mut total_time = std::time::Duration::new(0, 0);

    println!("Pos       Nodes       ΣNodes     Time     ΣTime   Mnps  ΣMnps  FEN");
    for (idx, fen) in fens.iter().enumerate() {
        let pos = Position::from_fen(fen);
        let start = Instant::now();
        let nodes = pos.perft(depth);
        let duration = start.elapsed();

        total_nodes += nodes;
        total_time += duration;

        let nps = nodes as f64 / duration.as_secs_f64();
        let total_nps = total_nodes as f64 / total_time.as_secs_f64();

        println!(
            "{:<5}{:>10}{:>13}{:>9.3}{:>10.3}{:>7}{:>7}  {:<}",
            idx + 1,
            nodes,
            total_nodes,
            duration.as_millis() as f64 / 1000.0,
            total_time.as_millis() as f64 / 1000.0,
            (nps / 1_000_000f64) as u64,
            (total_nps / 1_000_000f64) as u64,
            fen
        );
    }
}
