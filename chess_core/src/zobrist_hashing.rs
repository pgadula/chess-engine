use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

use crate::{
    bitboard::GameState,
    types::PIECES_ARRAY,
    utility::get_file_ranks,
};

#[derive(Debug)]
pub struct ZobristHashing {
    pub pieces: [u64; 12 * 64],
    pub castling_rights: [u64; 16],
    pub en_passant: [u64; 64],
    pub side: u64,
}

const SEED:u64 = 29426028;
impl ZobristHashing {
    pub fn init() -> ZobristHashing {
        let mut rng = Pcg32::seed_from_u64(SEED);

        let zobrist_hashing = ZobristHashing {
            pieces: core::array::from_fn(|_| rng.gen()),
            castling_rights: core::array::from_fn(|_| rng.gen()),
            en_passant: core::array::from_fn(|_| rng.gen()),
            side: rng.gen(),
        };
        return zobrist_hashing;
    }

    pub fn get_hash(&self, game: &GameState) -> u64 {
        let mut hash = 0;
        for (index, squares) in game.bitboard.iter().enumerate() {
            for file_rank in get_file_ranks(*squares) {
                let piece_index = PIECES_ARRAY[index].bitboard_index();
                let hash_index = piece_index * 64 + file_rank.index();
                hash ^= self.pieces[hash_index];
            }
        }

        hash ^= self.castling_rights[game.castling.mask as usize % 16];
        if let Some(file_rank) = game.en_passant {
            hash ^= self.en_passant[file_rank.index()];
        }

        hash ^= (game.move_turn as u64) * self.side;
        return hash;
    }
}
