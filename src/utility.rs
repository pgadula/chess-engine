use crate::{game::Game, types::{Move, RANK_2, RANK_3, RANK_6}};

pub fn print_chessboard_from_u64(number: u64) {
    // Convert the number to a 64-bit binary string, padded with zeros if necessary
    let binary_string = format!("{:064b}", number);

    for row in (0..8).rev() {
        for col in (0..8).rev() {
            // Calculate the index in the binary string
            let index = row * 8 + col;
            print!("{} ", &binary_string[index..index + 1]);
        }
        println!();
    }
}

