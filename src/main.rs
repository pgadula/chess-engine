mod base_types;
mod file_rank;
mod game;
mod magic_gen;
mod moves_gen;
mod precalculated;
mod utility;

use std::{
    array,
    sync::Arc,
    time::{Duration, Instant},
};

use base_types::{get_piece_from_char, Color, FileRank};
use game::{BitBoard, FenParser};
use magic_gen::{MagicHelper, DB};
use moves_gen::{get_king_attacks, get_knight_attacks, get_pawn_attacks, get_pawn_moves};
use rand::Rng;
use utility::{bits::set_bit, print_as_board};

fn main() {
    let fen = "1nbqkbnr/pppppppp/R7/8/4P3/2rR4/PPPP1PPP/RNBQKBNR w KQkq e3 0 1";
    let db = Arc::new(DB::sliding_pieces());

    let game = BitBoard::deserialize(fen);
    game.print();
    benchmark("generate moves", 100_000, || {
        game.generate_moves(db.clone());
    });

}

fn benchmark<F>(label: &str, number_of_iteration: usize, func: F) -> Duration
where
    F: Fn() -> (),
{
    let now = Instant::now();
    for _ in 0..number_of_iteration {
        func();
    }
    let elapsed = now.elapsed();
    println!("Elapsed for {}: {:.2?}", label, elapsed);
    elapsed
}
