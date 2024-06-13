mod game;
mod base_types;
mod moves;
mod constants;
mod utility;
mod precalculated;

use constants::{NOT_A, NOT_B, NOT_G, NOT_H};
use game::Game;
use moves::_get_knight_attacks;
use utility::print_as_board;
use base_types::{FileRank};
 
fn main() {
    let board = _get_knight_attacks(FileRank::E4);
    print_as_board(board);

}

