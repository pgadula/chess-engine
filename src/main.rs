mod base_types;
mod file_rank;
mod game;
mod magic_gen;
mod moves_gen;
mod precalculated;
mod utility;

use std::sync::Arc;

use base_types::{Color, FileRank};
use game::{BitBoard, FenParser};
use magic_gen::MagicQuery;
use utility::print_as_board;

fn main() {
    let fen = "1nbqkbnr/pppppppp/R7/8/4P3/3R4/PPPP1PPP/RNBQKBNR w KQkq e3 0 1";

    let game = BitBoard::deserialize(fen);
    game.print();
    let mq = Arc::new( MagicQuery::init_sliding());
    
    let (mut rooks, mut bishops) = if game.turn == Color::White { (game.w_rook, game.w_bishop)}  else  {(game.b_rook, game.b_bishop)};

    // while rooks > 0 {
    //     let i = BitBoard::pop_lsb(&mut rooks) as u8;
    //     let file_rank = FileRank::get_file_rank(i).unwrap();

    //     let mut r_move: u64 = mq.clone().get_rook_attack(file_rank, game);

    //     let mut attack_board: u64 = 0;

    //     println!("\n start:{:?}", file_rank);

    //     while r_move > 0 {

    //         let i: u8 = BitBoard::pop_lsb(&mut r_move) as u8;
    //         let attack_file_rank = FileRank::get_file_rank(i).unwrap();
    //         println!("  attack:{:?}", attack_file_rank);
    //         BitBoard::set_bit(&mut attack_board, attack_file_rank)
    //     }
    //     print_as_board(attack_board)
    // }
    while bishops > 0 {
        let i = BitBoard::pop_lsb(&mut bishops) as u8;
        let file_rank = FileRank::get_file_rank(i).unwrap();

        let mut b_move: u64 = mq.clone().get_bishop_attack(file_rank, game);

        let mut attack_board: u64 = 0;

        println!("\n start:{:?}", file_rank);

        while b_move > 0 {

            let i: u8 = BitBoard::pop_lsb(&mut b_move) as u8;
            let attack_file_rank = FileRank::get_file_rank(i).unwrap();
            println!("  attack:{:?}", attack_file_rank);
            BitBoard::set_bit(&mut attack_board, attack_file_rank)
        }
        print_as_board(attack_board)
    }

}
