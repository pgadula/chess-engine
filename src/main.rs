mod base_types;
mod constants;
mod game;
mod moves_gen;
mod precalculated;
mod utility;
mod magic_gen;

use base_types::FileRank;
use constants::{EDGES, FILE_A, FILE_H, RANK_1, RANK_8};
use game::{BitBoard, FenParser};
use magic_gen::MagicQuery;

use utility::print_as_board;
use std::time::Instant;


fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";

    let game = BitBoard::deserialize(fen);
    let file_rank = FileRank::F3;
    game.print();
   let mq =  MagicQuery::init_rook();
   let bmove =  mq.clone().get_bishop_attack(file_rank, game);

   let rmove =  mq.get_rook_attack(file_rank, game);
   let rocks_moves = (game.get_black_pieces() & EDGES) ^ rmove;
   let bishop_moves = (game.get_black_pieces() & EDGES) ^ bmove;
}







