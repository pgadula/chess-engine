use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

use crate::{
    bitboard::GameState, types::PIECES_ARRAY, utility::{get_file_ranks, pop_lsb}
};

#[derive(Debug)]
pub struct ZobristHashing {
    pub pieces: [u64; (12 * 64)],
    pub castling_rights: [u64; 4],
    pub en_passant: u64,
    pub side: u64
}

impl ZobristHashing {
    pub fn new() -> ZobristHashing {
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
    pub fn hash(&self, game: &GameState) -> u64 {
        let mut hash = 0;
        for (index, squares) in game.bitboard.iter().enumerate() {
            for file_rank in get_file_ranks(*squares) {
                let piece_index = PIECES_ARRAY[index].bitboard_index();
                let r_index = file_rank.index() * piece_index;
                hash ^= (self.pieces[r_index] * PIECES_ARRAY[index].bitboard_index() as u64)
            }
        }
        hash
    }
}
