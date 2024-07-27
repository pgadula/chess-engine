use chess_core::{
    algebraic_notation::AlgebraicNotation,
    base_types::{Color, Piece},
    magic_gen::MoveLookupTable,
    utility::print_as_board,
    BitBoard, FenParser,
};
use std::{
    array,
    sync::Arc,
    time::{Duration, Instant},
};

fn main() {
    let fen = "1nbqkbnr/pppppppp/R7/8/4P3/2rR4/PPPP1PPP/RNBQKBNR w KQkq e3 0 1";

    let mut game = BitBoard::deserialize(fen);
    let board: u64 = 0xF;

    game.print();

    game.calculate_moves();

    // let notations = "e4 e5 Nxf3 Nc6 Bb5 a6 Ba4 Nf6 O-O Be7 Re1 b5 Bb3 d6 c3 O-O-O h3 Nb8 d4 Nbd7";

    // let mut notation = notations.split_whitespace();
    // while let Some(c) = notation.next() {
    //     let mut tokenizer: AlgebraicNotation = AlgebraicNotation::new(c);
    //     while let Some(token) = tokenizer.next_token() {
    //         println!("token: {:?}",token);
    //     }
    //     println!();
    // }
}
