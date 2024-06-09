mod types;
mod game;
pub mod utility;

use game::Game;
use types::{Color, Moves, Piece, RANK_8, RANK_1};
use utility::{get_pawn_moves, print_chessboard_from_u64};


fn main() {

   let mut game = Game::new_game();
   print_chessboard_from_u64((game.w_pawn >> 8 ) & game.empty_square());
   game.set_piece(&(Piece::Pawn, Color::Black), types::FileRank::A3);
    println!();
   let moves:Moves =  Moves(get_pawn_moves(&game));

   println!("{}", moves)
    
}

