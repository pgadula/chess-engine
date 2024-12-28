use chess_core::bitboard::GameState;
use chess_uci::search_engine::SearchEngine;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn search_engine_benchmark(c: &mut Criterion) {
    let max_depth = 9;
    let game = GameState::new_game();

    let mut criterion = Criterion::default().sample_size(10);
    criterion.bench_with_input(
        BenchmarkId::new("search_engine bench", max_depth),
        &max_depth,
        |b, &_depth| {
            b.iter_batched(
                /* setup closure: returns a fresh `SearchEngine` each time */
                || {
                    let mut engine = SearchEngine::new();
                    engine.transposition_table.clear_lookup_table();
                    engine.max_depth = max_depth;
                    engine
                },
                /* measurement closure: uses the owned `engine` */
                |mut engine| {
                    engine.rayon_search(&game);
                },
                criterion::BatchSize::SmallInput,
            );
        },
    );
}

pub fn search_engine_benchmark_depth_10(c: &mut Criterion) {
    let max_depth = 10;
    let game = GameState::new_game();

    let mut criterion = Criterion::default().sample_size(10);
    criterion.bench_with_input(
        BenchmarkId::new("search_engine bench", max_depth),
        &max_depth,
        |b, &_depth| {
            b.iter_batched(
                /* setup closure: returns a fresh `SearchEngine` each time */
                || {
                    let mut engine = SearchEngine::new();
                    engine.transposition_table.clear_lookup_table();
                    engine.max_depth = 10;
                    engine
                },
                /* measurement closure: uses the owned `engine` */
                |mut engine| {
                    engine.search(&game);
                },
                criterion::BatchSize::SmallInput,
            );
        },
    );
}

// Define the Criterion benchmark group and main function
criterion_group!(benches, search_engine_benchmark);
criterion_main!(benches);