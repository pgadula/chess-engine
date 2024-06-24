use rand::Rng;

use crate::{base_types::FileRank, game::BitBoard, generate_attack_subsets, moves_gen::_gen_rook_move_fly, utility::print_as_board};
pub struct Magic{}


impl Magic {
    pub fn get_random_number() -> u64 {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    
        let n: u64 = rng.gen();
        let n1: u64 = rng.gen();
        let n2: u64 = rng.gen();
    
        n & n1 & n2
    }
    
    pub fn get_magic_index(mask: u64, magic_number: u64, shift: usize) -> usize {
        ((mask.wrapping_mul(magic_number)) >> (shift)) as usize
    }
    
    
    
    pub fn create_table(file_rank:FileRank, attack_mask:u64, shift: usize, magic_number: u64)->Vec<u64>{
        let num_bits: usize = 64 - shift;
        let lookup_size = 1u64 << num_bits;
        let mut table: Vec<u64> = vec![0; lookup_size as usize];
    
        let subsets: Vec<u64> = generate_attack_subsets(attack_mask);
    
        for subset in subsets {
            let magic_index = Magic::get_magic_index(subset, magic_number, shift);
            let attack = _gen_rook_move_fly(file_rank, subset);
    
            table[magic_index] = attack;
        }
    
        table
    
    }
}
