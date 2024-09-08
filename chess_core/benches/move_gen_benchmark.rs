

use chess_core::bitboard::{GameState, FenParser};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let fen = "1nbqkbnr/pppppppp/R7/8/4P3/2rR4/PPPP1PPP/RNBQKBNR w KQkq e3 0 1";
    let mut game = GameState::deserialize(fen);


    c.bench_function("move generation", |b| b.iter(|| {
        game.calculate_pseudolegal_moves();
        game.get_valid_moves();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);