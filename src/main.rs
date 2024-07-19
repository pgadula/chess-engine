mod base_types;
mod file_rank;
mod game;
mod magic_gen;
mod moves_gen;
mod precalculated;
mod utility;

use base_types::FileRank;
use game::{BitBoard, FenParser};
use magic_gen::MagicQuery;
use utility::print_as_board;

fn main() {
    let fen = "1nbqkbnr/pppppppp/R7/8/4P3/3R4/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";

    let game = BitBoard::deserialize(fen);
    game.print();
    let mq = MagicQuery::init_leaper();
    //    let bmove =  mq.clone().get_bishop_attack(file_rank, game);
    let mut w_rooks = game.w_rook;
    while w_rooks > 0 {
        let i = BitBoard::pop_lsb(&mut w_rooks) as u8;
        let file_rank = FileRank::get_file_rank(i).unwrap();

        let mut rmove: u64 = mq.clone().get_rook_attack(file_rank, game);

        let mut attack_board: u64 = 0;

        println!("\n start:{:?}", file_rank);

        while rmove > 0 {

            let i: u8 = BitBoard::pop_lsb(&mut rmove) as u8;
            let attack_file_rank = FileRank::get_file_rank(i).unwrap();
            println!("  attack:{:?}", attack_file_rank);
            BitBoard::set_bit(&mut attack_board, attack_file_rank)
        }
        print_as_board(attack_board)
    }

}
