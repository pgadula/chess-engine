use crate::{bitboard::BitBoard, types::FileRank};

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

pub fn set_bit(bit_board: &mut u64, file_rank: FileRank) {
    let file_rank_num = file_rank as u8;
    let mask = 1u64 << file_rank_num;
    *bit_board |= mask;
}
pub fn set_bit_by_index(bit_board: &mut u64, index: u8) {
    let mask = 1u64 << index;
    *bit_board |= mask;
}

pub fn clear_bit(bit_board: &mut u64, file_rank: FileRank) {
    let file_rank_num = file_rank as u8;
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

pub fn get_heatmap(bitboard: &BitBoard) -> [f32; 64] {
    const MAX_PER_SQUARE:f32 = 4.00;
    let mut attacks_per_square: [f32; 64] = [0.0; 64];
    FileRank::iter().for_each(|fr| {
        let index = fr.index();
        let sum:f32 = (bitboard.black_attacked_squares[index].len()
        + bitboard.white_attacked_squares[index].len()) as f32;
        attacks_per_square[index] = 
           sum / MAX_PER_SQUARE;
    });
    attacks_per_square
}

pub fn print_heatmap(bitboard: &BitBoard) {
    let heatmap = get_heatmap(bitboard);
    println!("Heatmap:");
    println!("  a     b     c     d     e     f     g     h");
    println!(" +-----+-----+-----+-----+-----+-----+-----+-----+");

    for rank in (0..8).rev() {
        print!("{}|", (rank + 1).to_string());
        for file in 0..8 {
            let index = rank * 8 + file;
            let value = heatmap[index];
            let colored_value = if value > 0.75 {
                format!("\x1b[31m{:5.2}\x1b[0m", value)  // Red
            } else if value > 0.5 {
                format!("\x1b[33m{:5.2}\x1b[0m", value)  // Yellow
            } else if value > 0.25 {
                format!("\x1b[32m{:5.2}\x1b[0m", value)  // Green
            } else {
                format!("{:5.2}", value)  // Default color
            };
            print!("{}|", colored_value);
        }
        println!(" {}", (rank + 1).to_string());
        println!(" +-----+-----+-----+-----+-----+-----+-----+-----+");
    }

    println!("  a     b     c     d     e     f     g     h");
}