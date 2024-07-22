mod base_types;
mod file_rank;
mod game;
mod magic_gen;
mod moves_gen;
mod precalculated;
mod utility;

use std::{array, sync::Arc};

use base_types::{Color, FileRank};
use game::{BitBoard, FenParser};
use magic_gen::MagicQuery;
use moves_gen::{get_king_attacks, get_knight_attacks, get_pawn_attacks, get_pawn_moves};
use utility::{bits::set_bit, print_as_board};

fn main() {
    let fen = "1nbqkbnr/pppppppp/R7/8/4P3/2rR4/PPPP1PPP/RNBQKBNR w KQkq e3 0 1";
    let db = Arc::new(MagicQuery::init_sliding());

    let game = BitBoard::deserialize(fen);
    game.print();
    let mut moves = game.generate_moves(db);


    let i = FileRank::B1.index();
    let mut board: u64 = 0u64;

    for (i, position) in moves.into_iter().enumerate() {
        let fr = FileRank::get_file_rank(i as u8);
        let number_of_pos = position.len();
        if number_of_pos > 0{
            println!("\nFrom {:?}", fr.unwrap());
            print!("To: ");
        }
        for attack in position {
            let fr_attack = FileRank::get_file_rank(attack);
            set_bit(&mut board, fr_attack.unwrap());
            print!("{:?}, ", fr_attack.unwrap());
        }
        if number_of_pos > 0{
            println!()
        }
    }
    print_as_board(board);
}

