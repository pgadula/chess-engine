mod base_types;
mod constants;
mod game;
mod moves_gen;
mod precalculated;
mod utility;

use base_types::FileRank;
use game::{FenParser, Game};
use precalculated::{BISHOP_ATTACK_MASK, KING_ATTACK_MASK, KNIGHT_ATTACK_MASK, ROOK_ATTACK_MASK};
use rand::Rng;
use utility::print_as_board;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    generate_magics(KING_ATTACK_MASK);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn generate_magics(mask_attacks: [u64; 64]) {
    let mut magic_numbers: [u64; 64] = [0; 64];
    let mut magic_offsets = [0; 64];

    for file_rank in FileRank::iter() {
        let file_rank_index = file_rank.index() as usize;
        let attack_mask = mask_attacks[file_rank_index];

        let bit_count = Game::bit_count(attack_mask);
        let count: usize = 1 << bit_count;
        let subsets = get_subsets(count, attack_mask);


        let mut is_valid_magic_number: bool = false;

        let mut magic_number: u64;
        let relevant_bit = 64 - bit_count;

        magic_offsets[file_rank_index] = relevant_bit;

        while !is_valid_magic_number {
            is_valid_magic_number = true;
            let mut attacks: Vec<u64> = vec![1; count]; //
            magic_number = get_random_number();

            for subset in subsets.clone().into_iter() {
                let magic_index = ((subset.wrapping_mul(magic_number)) >> (relevant_bit)) as usize;

                if attacks[magic_index] == 1 {
                    attacks[magic_index] = subset;
                    // println!("fit, {} {}",attacks[magic_index], subset  )
                } else {
                    is_valid_magic_number = false;
                    break;
                }
            }

            if is_valid_magic_number {
                is_valid_magic_number = true;
                magic_numbers[file_rank_index] = magic_number;
            }
        }
    }
    println!("magic numbers {:?}", magic_numbers);
    println!("magic offset {:?}", magic_offsets);
}

fn get_subsets(count: usize, mask: u64) -> Vec<u64> {
    let mut subsets: Vec<u64> = Vec::with_capacity(count);

    for index in 0..count {
        let result = get_occupancy(index, mask);
        subsets.push(result)
    }
    subsets
}

fn get_occupancy(index: usize, attack_mask: u64) -> u64 {
    let mut mask = attack_mask;
    let mut occupancy = 0u64;
    let bit_count = Game::bit_count(attack_mask);
    for count in 0..bit_count {
        let square = Game::get_lsb_index(mask);
        let c: u64 = count as u64;
        let i = index as u64;
        Game::pop_bit(&mut mask, square as u8);
        if i & (1u64 << c) > 0 {
            occupancy |= 1u64 << square;
        }
    }
    occupancy
}

pub fn get_rook_moves(square: FileRank, blockers: u64) {
    let index = square.index() as usize;
    let and_mask = ROOK_ATTACK_MASK[index];
    let and_result = and_mask & blockers;
    let bit_count = Game::bit_count(and_mask);
    print_as_board(and_mask);
    println!();
    print_as_board(and_result);
    print!("{bit_count}");
}

pub fn get_random_number() -> u64 {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let n: u64 = rng.gen();
    let n1: u64 = rng.gen();
    let n2: u64 = rng.gen();

    n & n1 & n2
}
