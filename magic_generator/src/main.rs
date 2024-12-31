use std::array;

use chess_core::{magic_gen::{self, MagicHelper}, moves_gen::{get_king_attacks, get_pawn_moves, get_pawn_pattern_attacks}, types::{Color, FileRank}};

fn main() {
    let mut mask:[u64; 64] = array::from_fn(|_|{0});
    for (i, file_rank) in FileRank::iter().enumerate() {
        mask[i] = get_pawn_pattern_attacks(Color::Black, &file_rank)
    }

    let result = MagicHelper::generate_magics(mask);

    println!("MAGIC_NUMBERS: {:?}", result.magic_numbers);
    println!("SHIFTS: {:?}", result.shifts);

}

