

use chess_core::bitboard::{GameState, FenParser};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn perft_benchmark(c: &mut Criterion) {
    let game = GameState::new_game();
    let depth = 3;

    // Create a custom configuration with 10 samples
    let mut criterion = Criterion::default().sample_size(10);

    // Benchmark the `perft` function with the custom configuration
    criterion.bench_with_input(BenchmarkId::new("perft depth", depth), &depth, |b, &depth| {
        b.iter(|| {
            let (total_nodes, _) = game.clone().perft(black_box(depth));
            black_box(total_nodes); // Prevent optimizations
        });
    });
}

// Define the Criterion benchmark group and main function
criterion_group!(benches, perft_benchmark);
criterion_main!(benches);