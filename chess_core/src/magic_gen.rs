use rand::Rng;

use crate::{
    base_types::FileRank,
    moves_gen::{_gen_bishop_attacks_on_the_fly, _gen_rook_move_fly},
    precalculated::{BISHOP_ATTACK_MASK, BISHOP_MAGIC_NUMBERS, BISHOP_SHIFTS, ROOK_ATTACK_MASK, ROOK_MAGIC_NUMBERS, ROOK_SHIFTS},
    utility::bits::{bit_count, get_lsb_index, pop_bit}, BitBoard,
};

#[derive(Clone, Debug, PartialEq)]
pub struct MoveLookupTable {
    pub rook_attacks: [Vec<u64>; 64],
    pub bishop_attacks: [Vec<u64>; 64],
}

impl MoveLookupTable {
    pub fn init() -> MoveLookupTable {
        let mut rook: [Vec<u64>; 64] = std::array::from_fn(|_| Vec::new());
        let mut bishop: [Vec<u64>; 64] = std::array::from_fn(|_| Vec::new());

        for fr in FileRank::iter() {
            let fr_index = fr.index();

            let r_attack_mask = ROOK_ATTACK_MASK[fr_index];
            let r_magic_number = ROOK_MAGIC_NUMBERS[fr_index];
            let r_shift = ROOK_SHIFTS[fr_index] as usize;

            let b_attack_mask = BISHOP_ATTACK_MASK[fr_index];
            let b_magic_number = BISHOP_MAGIC_NUMBERS[fr_index];
            let b_shift = BISHOP_SHIFTS[fr_index] as usize;

            let r_num_bits: usize = 64 - r_shift;
            let b_num_bits: usize = 64 - b_shift;

            let r_lookup_size = 1u64 << r_num_bits;
            let b_lookup_size = 1u64 << b_num_bits;

            let mut r_table: Vec<u64> = vec![0; r_lookup_size as usize];
            let mut b_table: Vec<u64> = vec![0; b_lookup_size as usize];

            let r_subsets: Vec<u64> = MagicHelper::generate_attack_subsets(r_attack_mask);
            let b_subsets: Vec<u64> = MagicHelper::generate_attack_subsets(b_attack_mask);

            for subset in r_subsets {
                let magic_index = MagicHelper::get_magic_index(subset, r_magic_number, r_shift);
                let attack = _gen_rook_move_fly(*fr, subset);
                r_table[magic_index] = attack;
            }
            
            for subset in b_subsets {
                let magic_index = MagicHelper::get_magic_index(subset, b_magic_number, b_shift);
                let attack = _gen_bishop_attacks_on_the_fly(*fr, subset);
                b_table[magic_index] = attack;
            }
            bishop[fr_index] = b_table;
            rook[fr_index] = r_table;

        }

        MoveLookupTable {
            bishop_attacks: bishop,
            rook_attacks: rook,
        }
    }

    pub fn get_rook_attack(&self, file_rank: FileRank, bit_board: &BitBoard) -> u64 {
        let fr_index = file_rank.index();

        let attack_mask = ROOK_ATTACK_MASK[fr_index];
        let magic_number = ROOK_MAGIC_NUMBERS[fr_index];
        let shift = ROOK_SHIFTS[fr_index] as usize;
        let blockers: u64 = attack_mask & bit_board.get_all_pieces();
        let magic_index = MagicHelper::get_magic_index(blockers, magic_number, shift);
        let attacks = self.rook_attacks[fr_index][magic_index];
        attacks
    }

    pub fn get_bishop_attack(&self, file_rank: FileRank, bit_board: &BitBoard) -> u64 {
        let fr_index = file_rank.index();

        let attack_mask = BISHOP_ATTACK_MASK[fr_index];
        let magic_number = BISHOP_MAGIC_NUMBERS[fr_index];
        let shift = BISHOP_SHIFTS[fr_index] as usize;
        let blockers: u64 = attack_mask & bit_board.get_all_pieces();
        let magic_index = MagicHelper::get_magic_index(blockers, magic_number, shift);
        let attacks = self.bishop_attacks[fr_index][magic_index];
        attacks
    }
}

pub struct MagicHelper {
    magic_numbers: [u64; 64],
    shifts: [usize; 64],
}

impl MagicHelper {
    pub fn get_random_number() -> u64 {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        let n: u64 = rng.gen();
        let n1: u64 = rng.gen();
        let n2: u64 = rng.gen();

        n & n1 & n2
    }

    fn generate_magics(mask_attacks: [u64; 64]) -> MagicHelper {
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
                //TODO its weird to fill with 1
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

    fn calculate_occupancy(index: usize, attack_mask: u64) -> u64 {
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

    fn get_magic_index(blockers: u64, magic_number: u64, shift: usize) -> usize {
        ((blockers.wrapping_mul(magic_number)) >> (shift)) as usize
    }
}
