mod types;
mod game;
pub mod utility;
mod moves;
mod precalculated;

use game::Game;
use moves::{_gen_rook_mask, get_pawn_moves};
use precalculated::ROOK_ATTACK_MASK;
use types::{Color, FileRank::{self, *}, Moves, Piece, FILE_RANK, RANK_1, RANK_8};
use utility::{ print_chessboard_from_u64};


fn main() {

   let mut game = Game::new_game();
   game.set_piece(&(Piece::Pawn, Color::White), E4);
   // Game::clear_bit(&mut game.w_pawn, E7);

   game.w_turn = false;
    println!();
    game.print();


    FileRank::iterator().for_each(|fr|{
      println!("for rank {:?}", fr);
      let board = _gen_rook_mask(*fr);

      print_chessboard_from_u64(board);
      println!();
      println!("mask {}", board);
      let pre = ROOK_ATTACK_MASK[(*fr) as usize];
      assert_eq!(pre, board)

    });

}

