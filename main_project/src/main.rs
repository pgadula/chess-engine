mod test_cases;

use chess_core::{
    algebraic_notation::AlgebraicNotation,
    bitboard::{self, BitBoard, FenParser},
    file_rank::{BLACK_KING_CASTLE_MASK, BLACK_QUEEN_CASTLE_MASK, RANK_3, RANK_8, WHITE_KING_CASTLE_MASK, WHITE_QUEEN_CASTLE_MASK},
    types::{AlgebraicNotationToken, BoardSide, Color, FileRank, MoveType, Piece, PieceIndex, PieceMove, PieceType},
    utility::{bit_count, print_as_board},
};
use test_cases::TEST_CASES;

fn main() {
    let fen = "rnb1kbnr/pp1pq1pp/4p3/5p2/2PpP1P1/2NQB3/PP3P1P/R3KBNR w KQkq - 1 8";
    let mut game = BitBoard::deserialize(fen);
    // let king = game.bitboard[PieceIndex::k.idx()];
    // handle_castling(&game);
    // let mask = (game.get_black_pieces() & RANK_8) ^  ;
    // print_as_board(mask);

    // for test_position in TEST_CASES.iter() {
    //     debug_move_generator(test_position);
    // }
    // debug_move_generator(&TEST_CASES[5]);

    game.calculate_pseudolegal_moves();
    game.print();

    let valid_attacks: Vec<&PieceMove> = game.get_valid_moves();
    for attack in valid_attacks {
        println!();
        println!("{:?}", attack);
        let mut c_game = game.clone();
        c_game.print();
        c_game.apply(attack);
        c_game.print();


        println!();
    }
}



fn debug_move_generator(test_position: &test_cases::TestPosition) {
    let mut game: BitBoard = BitBoard::deserialize(&test_position.fen);
    game.calculate_pseudolegal_moves();
    // print_as_board(game.b_attacks_mask);
    game.print();

    let valid_attacks: Vec<&PieceMove> = game.get_valid_moves();
    let count = valid_attacks.len();
    println!(
        "fen: {}\nexpected nodes: {} received: {}\n",
        test_position.fen, test_position.nodes, count,
    );

    for attack in valid_attacks {
        println!();
        println!("{:?}", attack);
        let mut c_game = game.clone();
        c_game.print();
        c_game.apply(attack);
        c_game.print();


        println!();
    }
}


