use ataxx::position::Position;
use std::time::Instant;

fn main() {
    let pos = Position::from_fen("startpos");
    let depth = 6;
    let moves = pos.legal_moves();
    let start = Instant::now();
    let mut total = 0;

    print!("{}", pos);

    for mv in &moves {
        let npos = pos.after_move(&mv);
        let nodes = npos.perft(depth);
        total += nodes;
        println!("{} {}", &mv, nodes);
    }

    if moves.is_empty() {
        let npos = pos.after_pass();
        let nodes = npos.perft(depth);
        total += nodes;
        println!("0000 {}", nodes);
    }

    let duration = start.elapsed();
    let nps = total as f64 / duration.as_secs_f64();

    println!(
        "moves {} nodes {} time {:?} nps {}",
        moves.len(),
        total,
        duration.as_millis(),
        nps as u64
    );
}
