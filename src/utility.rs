use crate::{game::Game, types::{Move, RANK_2, RANK_3}};

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

pub fn get_pawn_moves(game: &Game)->Vec<Move>{
    let mut moves:Vec<Move> = Vec::new();
    let mut pawns = game.w_pawn;
    let blockers = game.empty_square();
    
    while pawns > 0{
        let index = pawns.trailing_zeros() as u8;
        let isolated_pawn:u64 = 1 << index as u64;   
        let single_push:u64 = (isolated_pawn >> 8) & blockers;
        let double_push = (single_push & RANK_3)  >> 8 & blockers;
        if single_push > 0{
            let mv = Move{
                from: index,
                to: single_push.trailing_zeros() as u8
            };
            println!("{}", mv);
            moves.push( mv );
        }
        if double_push > 0 {
            let mv = Move{
                from: index,
                to: double_push.trailing_zeros() as u8
            };
            println!("{}", mv);

            moves.push( mv );

        }
        println!();

        Game::clear_bit_by_index(&mut pawns, index)
    }

    return moves;
}