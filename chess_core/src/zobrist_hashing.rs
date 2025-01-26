use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

use crate::{
    bitboard::GameState,
    types::{FileRank, MoveType, PieceMove, PIECES_ARRAY},
    utility::get_file_ranks_from_mask,
};

#[derive(Debug)]
pub struct ZobristHashing {
    pub pieces: [u64; 12 * 64],
    pub castling_rights: [u64; 16],
    pub en_passant: [u64; 64],
    pub side: u64,
}

const SEED: u64 = 2305843009213693951; // prime near 2^64
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
    pub fn piece_index(index: usize, file_rank:&FileRank)->usize{
        let piece_index = PIECES_ARRAY[index].bitboard_index();
        piece_index * 64 + file_rank.index()
    }

    pub fn get_hash_from_scratch(&self, game: &GameState) -> u64 {
        let mut hash = 0;
        for (index, squares) in game.bitboard.iter().enumerate() {
            for file_rank in get_file_ranks_from_mask(*squares) {
                let hash_index = Self::piece_index(index, &file_rank);
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

    pub fn fast_recalculate_hash(&self, game: &GameState, mv: &PieceMove) -> u64 {
        let mut hash = game.hash;
        hash ^= (game.move_turn.flip() as u64) * self.side;
        hash ^= (game.move_turn as u64) * self.side;

        println!("{:?}", mv);
        match mv.move_type {
            MoveType::Quiet | MoveType::DoublePush(_) => {
                let hash_index_from = Self::piece_index(mv.piece.bitboard_index(), &mv.from);
                let hash_index_target = Self::piece_index(mv.piece.bitboard_index(), &mv.target);
                //revert
                hash ^= self.pieces[hash_index_from];

                //applying
                hash ^= self.pieces[hash_index_target];
            },

            MoveType::CastleKingSide => {
            },
            MoveType::CastleQueenSide => {}
            MoveType::Promotion(piece_type) => {}
            MoveType::Capture => {
                // let target_piece =  game.get_piece_at(&mv.target).unwrap();
                // let hash_index_targeted = Self::piece_index(target_piece.bitboard_index(), &mv.target);
                // let hash_index_from = Self::piece_index(mv.piece.bitboard_index(), &mv.from);
                // let hash_index_target = Self::piece_index(mv.piece.bitboard_index(), &mv.target);

                //reverting
                // hash ^= self.pieces[hash_index_targeted];
                // hash ^= self.pieces[hash_index_from];

                // //applying
                // hash ^= self.pieces[hash_index_target];
            }
            MoveType::CaptureWithPromotion(_) => {}
            MoveType::EnPassantCapture => todo!(),
        }

        return hash;
    }
}
