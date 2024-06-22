use rand::Rng;

pub fn get_random_number() -> u64 {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let n: u64 = rng.gen();
    let n1: u64 = rng.gen();
    let n2: u64 = rng.gen();

    n & n1 & n2
}

pub fn magic_index(subset: u64, magic_number: u64, relevant_bit: usize) -> usize {
    ((subset.wrapping_mul(magic_number)) >> (relevant_bit)) as usize
}