use crate::types::FileRank;

pub fn print_as_board(number: u64) {
    // Convert the number to a 64-bit binary string, padded with zeros if necessary
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

pub fn set_bit(bit_board: &mut u64, file_rank: &FileRank) {
    let file_rank_num = (*file_rank) as u8;
    let mask = 1u64 << file_rank_num;
    *bit_board |= mask;
}
pub fn set_bit_by_index(bit_board: &mut u64, index: u8) {
    let mask = 1u64 << index;
    *bit_board |= mask;
}

pub fn clear_bit(bit_board: &mut u64, file_rank: &FileRank) {
    let file_rank_num = (*file_rank) as u8;
    let mask = 1u64 << file_rank_num;
    *bit_board &= !(mask);
}

pub fn pop_bit(bit_board: &mut u64, index: u8) {
    let mask = 1u64 << index;
    *bit_board ^= mask;
}

pub fn pop_lsb(b: &mut u64) -> u32 {
    let i = b.trailing_zeros();
    *b &= (*b) - 1;
    return i;
}

pub fn get_file_ranks(bitboard: u64)->Vec<FileRank>{
    let mut file_ranks:Vec<FileRank> = Vec::with_capacity(64);
    let mut copy = bitboard;
    while copy > 0 {
        let index = pop_lsb(&mut copy);
        if let Some(fr) = FileRank::get_file_rank(index as u8) {
            file_ranks.push(fr);
        }
    }
    file_ranks
}

//Kernighan’s algorithm
pub fn bit_count(bit_board: u64) -> usize {
    let mut b = bit_board;
    let mut count = 0;
    while b != 0 {
        b &= b - 1; // Clears the lowest set bit
        count += 1;
    }
    count
}

pub fn get_lsb_index(bit_board: u64) -> u32 {
    bit_board.trailing_zeros()
}
