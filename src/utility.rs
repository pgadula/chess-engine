use crate::{game::Game, types::Move};

pub fn print_chessboard_from_u64(number: u64) {
    // Convert the number to a 64-bit binary string, padded with zeros if necessary
    let binary_string = format!("{:064b}", number);

    println!("Chessboard representation:");
    for row in (0..8).rev() {
        for col in 0..8 {
            // Calculate the index in the binary string
            let index = row * 8 + col;
            print!("{} ", &binary_string[index..index + 1]);
        }
        println!();
    }
}

pub fn get_pawn_moves(game: &Game)->Vec<Move>{
    let mut moves:Vec<Move> = Vec::new();
    let mut pawns = game.w_pawn;
    while pawns > 0{
        let index = pawns.trailing_zeros() as u8;
        moves.push( Move{
            from: index,
            to: index
        });
        Game::clear_bit_by_index(&mut pawns, index)
    }

    return moves;
}