mod base_types;
mod constants;
mod game;
mod moves_gen;
mod precalculated;
mod utility;
mod magic_gen;

use base_types::FileRank;
use game::Game;
use magic_gen::{get_random_number, magic_index};
use precalculated::{BISHOP_ATTACK_MASK, ROOK_ATTACK_MASK};


fn main() {
    use std::time::Instant;
    let now = Instant::now();

    generate_magics(ROOK_ATTACK_MASK);
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
        let subsets = generate_attack_subsets(attack_mask, count);

        let mut magic_number: u64;
        let relevant_bit = 64 - bit_count;
        magic_offsets[file_rank_index] = relevant_bit;

        let mut found_magic: bool = false;

        let mut attacks: Vec<u64> = vec![1; count];
        while !found_magic {
            attacks.fill(1);
            magic_number = get_random_number();
            found_magic = true;

            for &subset in &subsets {
                let magic_index = magic_index(subset, magic_number, relevant_bit);

                if attacks[magic_index] == 1 {
                    attacks[magic_index] = subset;
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
    println!("magic numbers {:?}", magic_numbers);
    println!("magic offset {:?}", magic_offsets);
}



fn generate_attack_subsets(mask:u64, count: usize) -> Vec<u64> {
    let mut subsets: Vec<u64> = Vec::with_capacity(count);

    for index in 0..count {
        let result = calculate_occupancy(index, mask);
        subsets.push(result)
    }
    subsets
}

fn calculate_occupancy(index: usize, attack_mask: u64) -> u64 {
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


