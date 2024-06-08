mod types;
mod game;
pub mod utility;

use game::{ Game};
use types::{Color, FileRank, Moves, Piece};
use utility::{get_pawn_moves, print_chessboard_from_u64};


fn main() {

   let mut game = Game::new_game();
   print_chessboard_from_u64((game.w_pawn >> 8 ) & game.empty_square());

   let moves:Moves =  Moves(get_pawn_moves(&game));

   println!("{}", moves)
    
}

