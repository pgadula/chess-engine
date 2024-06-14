mod game;
mod base_types;
mod moves;
mod constants;
mod utility;
mod precalculated;

use base_types::FileRank;
use constants::{NOT_A, NOT_B, NOT_G, NOT_H, NOT_RANK_1_2, NOT_RANK_7_8};

use game::Game;
use moves::{_gen_bishop_attacks_mask, _get_king_attacks};
use precalculated::{BISHOP_ATTACK_MASK, ROOK_ATTACK_MASK};
use utility::print_as_board;
 
fn main() {
    FileRank::iter().for_each(|fr|{
        let tr: u8 = fr.rank();
        let tf: u8 = fr.file();
        let start:usize =  (tr * 8 + tf).into();
        let board1 = ROOK_ATTACK_MASK[start];
        let board2 = BISHOP_ATTACK_MASK[start];
        let queen_attacks = board1 | board2;
        println!();
        print_as_board(queen_attacks);
    });


}  

