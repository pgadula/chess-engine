mod test_cases;

use chess_core::{
    algebraic_notation::AlgebraicNotation,
    bitboard::{self, BitBoard, FenParser},
    file_rank::RANK_3,
    types::{AlgebraicNotationToken, PieceMove, BoardSide, Color, FileRank, Piece, PieceType},
    utility::{bit_count, print_as_board},
};
use test_cases::TEST_CASES;

fn main() {
    let fen = "1nbqkbnr/pppppppp/R7/8/4P3/2rR4/PPPP1PPP/RNBQKBNR w KQkq e3 0 1";


    // print_as_board(white_queen_castle_mask); 
    // for test_position in TEST_CASES.iter().filter(|e| e.depth == 1) {
    //     debug_move_generator(test_position);
    // }
    debug_move_generator(&TEST_CASES[5]);

}

fn debug_move_generator(test_position: &test_cases::TestPosition) {
    let mut game: BitBoard = BitBoard::deserialize(&test_position.fen);
    game.calculate_pseudolegal_moves();
    print_as_board(game.get_black_pieces());
    // print_as_board(game.b_attacks_mask);
    game.print();
    let attacks = if game.turn == Color::White { &game.flat_white_attacks } else { &game.flat_black_attacks }; 


    let valid_attacks:Vec<&PieceMove> = attacks
        .iter()
        .map(|attack| {
            let mut cloned_game: BitBoard = game.clone();

            handle_normal_attack(&mut cloned_game, attack);
            handle_en_passant(&mut cloned_game, attack);

            cloned_game.calculate_pseudolegal_moves();
            let BoardSide {
                king,
                opposite_attacks,
                ..
            } = cloned_game.get_player_info(&cloned_game.turn);

            let check = cloned_game.detect_check(&king, &opposite_attacks);

            (check, attack)
        })
        .filter(|attack| attack.0 == false)
        .map(|tuple| tuple.1).collect();
    let count = valid_attacks.len();
    println!(
        "fen: {}\nexpected nodes: {} received: {}\n",
        test_position.fen, test_position.nodes, count,
    );

    for attack in valid_attacks {
        println!("{:?}", attack)
    }
}

fn handle_normal_attack(cloned_game: &mut BitBoard, attack: &PieceMove) {
    let target_piece = cloned_game.get_piece_at(&attack.target);
            
    if let Some(target_piece) = target_piece {
        cloned_game.clear_piece(&target_piece, &attack.target);
    }
            
    cloned_game.set_piece(&attack.piece, &attack.target);
    cloned_game.clear_piece(&attack.piece, &attack.from);
}

fn handle_en_passant(game: &mut BitBoard, attack: &PieceMove) {
    if let Some(en_passant_file_rank) = game.en_passant {
        if en_passant_file_rank == attack.target {
            let file_rank_mask = en_passant_file_rank.mask();
            let target_file_rank = if (file_rank_mask & RANK_3) > 0 {
                FileRank::get_from_mask(file_rank_mask >> 8).unwrap()
            } else {
                FileRank::get_from_mask(file_rank_mask << 8).unwrap()
            };
            game.clear_piece(
                &Piece {
                    color: game.turn.opposite(),
                    piece_type: PieceType::Pawn,
                },
                &target_file_rank,
            )
        }
    }
}
