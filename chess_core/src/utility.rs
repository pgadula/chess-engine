use crate::types::FileRank;

pub fn print_as_board(number: u64) {
    let binary_string = format!("{:064b}", number);
    println!("   a b c d e f g h");
    for row in (0..8).rev() {
        print!("{}| ", row + 1);
        for col in (0..8).rev() {
            let index = row * 8 + col;
            print!("{} ", &binary_string[index..index + 1]);
        }
        println!();
    }
    println!("   a b c d e f g h");
    println!("Bitboard: {}", number);
}

#[inline(always)]
pub fn set_bit(bit_board: &mut u64, file_rank: &FileRank) {
    let file_rank_num = (*file_rank) as u8;
    let mask = 1u64 << file_rank_num;
    *bit_board |= mask;
}

#[inline(always)]
pub fn set_bit_by_index(bit_board: &mut u64, index: u8) {
    let mask = 1u64 << index;
    *bit_board |= mask;
}

#[inline(always)]
pub fn clear_bit(bit_board: &mut u64, file_rank: &FileRank) {
    let file_rank_num = (*file_rank) as u8;
    let mask = 1u64 << file_rank_num;
    *bit_board &= !(mask);
}

#[inline(always)]
pub fn pop_bit(bit_board: &mut u64, index: u8) {
    let mask = 1u64 << index;
    *bit_board ^= mask;
}

#[inline(always)]
pub fn pop_lsb(b: &mut u64) -> u32 {
    let i = b.trailing_zeros();
    *b &= (*b) - 1;
    return i;
}

#[inline(always)]
pub fn get_file_ranks_from_mask(bitboard: u64) -> impl Iterator<Item = FileRank> {
    let mut copy = bitboard;

    std::iter::from_fn(move ||{
        while copy > 0 {
            let index = pop_lsb(&mut copy);
            if let Some(fr) = FileRank::get_file_rank(index as u8) {
                return Some(fr)
            }
        }
        None
    })
}


#[inline(always)]
//Kernighan’s algorithm
pub fn bit_count(bit_board: u64) -> usize {
    return bit_board.count_ones() as usize
}

#[inline(always)]
pub fn get_lsb_index(bit_board: u64) -> u32 {
    bit_board.trailing_zeros()
}
