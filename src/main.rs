mod types;
mod game;
pub mod utility;
mod moves;
mod precalculated;

use game::Game;


use types::{Color, FileRank::{self, *}, Piece};
use utility::print_as_board;



fn main() {

    let mut board:u64 = 0;

    for r in 0..8{
        for f in 0..8{
            if f != 7{
                let index:u8 = r * 8 + f;
                if(f > 1){
                    Game::set_bit_by_index(&mut board, index)
                }
            }
        }
    }
    print_as_board(board);
    print!("{board}");
}

