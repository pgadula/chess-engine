use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

use crate::{
    bitboard::GameState, types::{Color, PIECES_ARRAY}, utility::{get_file_ranks, pop_lsb}
};

#[derive(Debug)]
pub struct ZobristHashing {
    pub pieces: [u64; 12 * 64],
    pub castling_rights: [u64; 4],
    pub en_passant: u64,
    pub side: u64
}

impl ZobristHashing {
    pub fn init() -> ZobristHashing {
        let seed = 29426028;
        let mut rng = Pcg32::seed_from_u64(seed);

        let zobrist_hashing = ZobristHashing {
            pieces: core::array::from_fn(|i| rng.gen()),
            castling_rights: core::array::from_fn(|i| rng.gen()),
            en_passant: rng.gen(),
            side:rng.gen()
        };
        return zobrist_hashing;
    }

    pub fn slow_hash(&self, game: &GameState) -> u64 {
        let mut hash = 0;
        for (index, squares) in game.bitboard.iter().enumerate() {
            for file_rank in get_file_ranks(*squares) {
                let piece_index = PIECES_ARRAY[index].bitboard_index();
                let r_index = file_rank.index() * piece_index;
                hash ^= self.pieces[r_index];
            }
        }
        hash^= game.castling.get_king_side(&crate::types::Color::White) as u64  * self.castling_rights[0];
        hash^= game.castling.get_king_side(&crate::types::Color::Black) as u64  * self.castling_rights[1];
        hash^= game.castling.get_queen_side(&crate::types::Color::White) as u64 *   self.castling_rights[2];
        hash^= game.castling.get_queen_side(&crate::types::Color::Black) as u64 *   self.castling_rights[3];

        hash ^= game.en_passant.map_or(0, |fr|fr.mask());    
        hash ^= (game.move_turn as u64) * self.side;
       return hash;
    }
}
