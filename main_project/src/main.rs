use std::{
    array,
    sync::Arc,
    time::{Duration, Instant},
};
use chess_core::{magic_gen::DB, BitBoard, FenParser};

fn main() {
    let fen = "1nbqkbnr/pppppppp/R7/8/4P3/2rR4/PPPP1PPP/RNBQKBNR w KQkq e3 0 1";

    let game = BitBoard::deserialize(fen);
        game.generate_moves();

}
