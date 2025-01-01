use rand::Rng;

use crate::{moves_gen::get_pawn_pattern_attacks, precalculated::{PAWN_ATTACK_MASK, PAWN_MAGIC_NUMBERS, PAWN_SHIFTS}, types::Color};

use super::{
    moves_gen::{_gen_bishop_attacks_on_the_fly, _gen_rook_move_fly},
    precalculated::{
        BISHOP_ATTACK_MASK, BISHOP_MAGIC_NUMBERS, BISHOP_SHIFTS, ROOK_ATTACK_MASK,
        ROOK_MAGIC_NUMBERS, ROOK_SHIFTS,
    },
    types::FileRank,
    utility::{bit_count, get_lsb_index, pop_bit},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MoveLookupTable {
    pub rook_attacks: [Vec<u64>; 64],
    pub bishop_attacks: [Vec<u64>; 64],
    pub pawn_attacks: [Vec<u64>; 128],

}

impl MoveLookupTable {
    pub fn init() -> MoveLookupTable {
        let mut rook: [Vec<u64>; 64] = std::array::from_fn(|_| Vec::new());
        let mut bishop: [Vec<u64>; 64] = std::array::from_fn(|_| Vec::new());
        let mut pawn: [Vec<u64>; 128] = std::array::from_fn(|_| Vec::new());

        for fr in FileRank::iter() {
            let fr_index = fr.index();
            bishop[fr_index] = MoveLookupTable::calc_lookup_table(
                BISHOP_ATTACK_MASK[fr_index],
                BISHOP_SHIFTS[fr_index],
                BISHOP_MAGIC_NUMBERS[fr_index],
                fr,
                _gen_bishop_attacks_on_the_fly,
            );
            rook[fr_index] = MoveLookupTable::calc_lookup_table(
                ROOK_ATTACK_MASK[fr_index],
                ROOK_SHIFTS[fr_index],
                ROOK_MAGIC_NUMBERS[fr_index],
                fr,
                _gen_rook_move_fly,
            );

            let white_move_fly = MoveLookupTable::factor_pawn_move(Color::White);

            pawn[fr_index] = MoveLookupTable::calc_lookup_table(
                PAWN_ATTACK_MASK[fr_index],
                PAWN_SHIFTS[fr_index],
                PAWN_MAGIC_NUMBERS[fr_index],
                fr,
                white_move_fly
            );

            let black_move_fly = MoveLookupTable::factor_pawn_move(Color::Black);


            let b_index = fr_index + 64;
            pawn[b_index] = MoveLookupTable::calc_lookup_table(
                PAWN_ATTACK_MASK[b_index],
                PAWN_SHIFTS[b_index],
                PAWN_MAGIC_NUMBERS[b_index],
                fr,
                black_move_fly
            );
        }

        MoveLookupTable {
            bishop_attacks: bishop,
            rook_attacks: rook,
            pawn_attacks: pawn
        }
    }

    fn factor_pawn_move(color: Color) -> impl Fn(FileRank, u64) -> u64 {
        // Use `move` if the closure needs to capture `color`.
        return move |fr: FileRank, _board: u64| {
            get_pawn_pattern_attacks(color, &fr)
        }
    }

    fn calc_lookup_table<F>(
        attacks_mask: u64,
        shift: usize,
        magic_number: u64,
        file_rank: &FileRank,
        mut move_generator: F,
    ) -> Vec<u64>
    where
        F: FnMut(FileRank, u64) -> u64,
    {
        let num_bits: usize = 64 - shift; 
        let lookup_size = 1u64 << num_bits;
        let mut table: Vec<u64> = vec![0; lookup_size as usize];
        let subsets: Vec<u64> = MagicHelper::generate_attack_subsets(attacks_mask);
        for subset in subsets {
            let magic_index = MagicHelper::get_magic_index(subset, magic_number, shift);
            let attack = move_generator(*file_rank, subset);
            table[magic_index] = attack;
        }
        return table;
    }

    #[inline(always)]
    pub fn get_rook_attack(&self, file_rank: FileRank, all_pieces: u64) -> u64 {
        let fr_index = file_rank.index();

        let attack_mask = ROOK_ATTACK_MASK[fr_index];
        let magic_number = ROOK_MAGIC_NUMBERS[fr_index];
        let shift = ROOK_SHIFTS[fr_index] as usize;
        let blockers: u64 = attack_mask & all_pieces;
        let magic_index = MagicHelper::get_magic_index(blockers, magic_number, shift);
        let attacks = self.rook_attacks[fr_index][magic_index];
        attacks
    }

    #[inline(always)]
    pub fn get_pawn_attack(&self, file_rank: FileRank, all_pieces: u64, color: Color) -> u64 {

        let offset = match color {
            Color::White => 0,
            Color::Black => 64,
        };
        let fr_index = file_rank.index() + offset;

        let attack_mask = PAWN_ATTACK_MASK[fr_index];
        let magic_number = PAWN_MAGIC_NUMBERS[fr_index];
        let shift = PAWN_SHIFTS[fr_index] as usize;
        let blockers: u64 = attack_mask & all_pieces;
        let magic_index = MagicHelper::get_magic_index(blockers, magic_number, shift);
        let attacks = self.pawn_attacks[fr_index][magic_index];
        attacks
    }

    #[inline(always)]
    pub fn get_bishop_attack(&self, file_rank: FileRank, all_pieces: u64) -> u64 {
        let fr_index = file_rank.index();

        let attack_mask = BISHOP_ATTACK_MASK[fr_index];
        let magic_number = BISHOP_MAGIC_NUMBERS[fr_index];
        let shift = BISHOP_SHIFTS[fr_index] as usize;
        let blockers: u64 = attack_mask & all_pieces;
        let magic_index = MagicHelper::get_magic_index(blockers, magic_number, shift);
        let attacks = self.bishop_attacks[fr_index][magic_index];
        attacks
    }
}

pub struct MagicHelper {
    #[allow(dead_code)]
    pub magic_numbers: [u64; 64],

    #[allow(dead_code)]
    pub shifts: [usize; 64],
}

impl MagicHelper {
    pub fn get_random_number() -> u64 {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        let n: u64 = rng.gen();
        let n1: u64 = rng.gen();
        let n2: u64 = rng.gen();

        n & n1 & n2
    }

    pub fn generate_magics(mask_attacks: [u64; 64]) -> MagicHelper {
        let mut magic_numbers: [u64; 64] = [0; 64];
        let mut shifts = [0; 64];

        for file_rank in FileRank::iter() {
            let file_rank_index = file_rank.index();
            let attack_pattern_mask = mask_attacks[file_rank_index];

            let bit_count = bit_count(attack_pattern_mask);
            let count: usize = 1 << bit_count;
            let subsets = Self::generate_attack_subsets(attack_pattern_mask);

            let mut magic_number: u64;
            let relevant_bit = 64 - bit_count;
            shifts[file_rank_index] = relevant_bit;

            let mut found_magic: bool = false;

            let mut attacks: Vec<bool> = vec![false; count];
            while !found_magic {
                attacks.fill(false);
                magic_number = MagicHelper::get_random_number();
                found_magic = true;

                for &subset in &subsets {
                    let magic_index =
                        MagicHelper::get_magic_index(subset, magic_number, relevant_bit);

                    if attacks[magic_index] == false {
                        attacks[magic_index] = true;
                    } else {
                        found_magic = false;
                        break;
                    }
                }

                if found_magic {
                    found_magic = true;
                    magic_numbers[file_rank_index] = magic_number;
                }
            }
        }

        MagicHelper {
            magic_numbers,
            shifts,
        }
    }

    pub fn generate_attack_subsets(attack_mask: u64) -> Vec<u64> {
        let bit_count = bit_count(attack_mask);
        let count: usize = 1 << bit_count;
        let mut subsets: Vec<u64> = Vec::with_capacity(count);

        for index in 0..count {
            let result = Self::calculate_occupancy(index, attack_mask);
            subsets.push(result)
        }
        subsets
    }

    pub fn calculate_occupancy(index: usize, attack_mask: u64) -> u64 {
        let mut mask = attack_mask;
        let mut occupancy = 0u64;
        let bit_count = bit_count(attack_mask);
        for count in 0..bit_count {
            let square = get_lsb_index(mask);
            let c: u64 = count as u64;
            let i = index as u64;
            pop_bit(&mut mask, square as u8);
            if i & (1u64 << c) > 0 {
                occupancy |= 1u64 << square;
            }
        }
        occupancy
    }

    #[inline(always)]
    pub fn get_magic_index(blockers: u64, magic_number: u64, shift: usize) -> usize {
        ((blockers.wrapping_mul(magic_number)) >> (shift)) as usize
    }
}
