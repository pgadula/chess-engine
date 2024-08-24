mod test_cases;

use chess_core::{
    algebraic_notation::AlgebraicNotation,
    bitboard::{self, BitBoard, FenParser},
    file_rank::RANK_3,
    types::{AlgebraicNotationToken, Attack, BoardSide, Color, FileRank, Piece, PieceLocation, PieceType},
    utility::{bit_count, get_heatmap, print_as_board, print_heatmap},
};
use test_cases::TEST_CASES;

fn main() {
    let fen = "1nbqkbnr/pppppppp/R7/8/4P3/2rR4/PPPP1PPP/RNBQKBNR w KQkq e3 0 1";


    print_as_board(white_queen_castle_mask); 
    // for test_position in TEST_CASES.iter().filter(|e| e.depth == 1) {
    // }
    // debug_move_generator(&TEST_CASES[5]);

}

fn debug_move_generator(test_position: &test_cases::TestPosition) {
    let mut game: BitBoard = BitBoard::deserialize(&test_position.fen);
    game.calculate_pseudolegal_moves();
    let attacks = if game.turn == Color::White { &game.flat_white_attacks } else { &game.flat_black_attacks }; 

    let valid_attacks:Vec<&Attack> = attacks
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

fn handle_normal_attack(cloned_game: &mut BitBoard, attack: &Attack) {
    let target_piece = cloned_game.get_piece_at(&attack.target);
            
    if let Some(target_piece) = target_piece {
        cloned_game.clear_piece(&target_piece, &attack.target);
    }
            
    cloned_game.set_piece(&attack.piece, &attack.target);
    cloned_game.clear_piece(&attack.piece, &attack.from);
}

fn handle_en_passant(game: &mut BitBoard, attack: &Attack) {
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





































fn parse_notation(unhandled: Vec<&str>) {
    for c in unhandled {
        let mut tokenizer: AlgebraicNotation = AlgebraicNotation::new(c);
        let mut tokens: Vec<AlgebraicNotationToken> = Vec::with_capacity(5);
        while let Some(token) = tokenizer.next_token() {
            tokens.push(token)
        }
        let len = tokens.len();
        match len {
            1 => {
                let pattern = tokens.as_slice();
                match pattern {
                    [AlgebraicNotationToken::CastleKingSide] => {
                        print!("CastleKingSide");
                    }
                    [AlgebraicNotationToken::CastleQueenSide] => {
                        print!("CastleQueenSide");
                    }
                    _ => {
                        println!("ERROR token#:{} unknown move {:?}", pattern.len(), pattern)
                    }
                }
            }
            2 => {
                let pattern = tokens.as_slice();
                match pattern {
                    [AlgebraicNotationToken::File(_), AlgebraicNotationToken::Rank(_)] => {
                        print!("Pawn move {:?}", pattern);
                    }
                    _ => {
                        println!("ERROR token#:{} unknown move {:?}", pattern.len(), pattern)
                    }
                }
            }
            3 => {
                let pattern: &[AlgebraicNotationToken] = tokens.as_slice();
                match pattern {
                    [AlgebraicNotationToken::Piece(_), AlgebraicNotationToken::File(_), AlgebraicNotationToken::Rank(_)] =>
                    {
                        print!("Piece move {:?}", pattern);
                    }
                    _ => {
                        println!("ERROR token#:{} unknown move {:?}", pattern.len(), pattern)
                    }
                }
            }
            4 => {
                let pattern: &[AlgebraicNotationToken] = tokens.as_slice();

                match pattern {
                    [AlgebraicNotationToken::Piece(_), AlgebraicNotationToken::File(_), AlgebraicNotationToken::File(_), AlgebraicNotationToken::Rank(_)] =>
                    {
                        print!("Piece move {:?}", pattern);
                    }
                    [AlgebraicNotationToken::Piece(_), AlgebraicNotationToken::Rank(_), AlgebraicNotationToken::File(_), AlgebraicNotationToken::Rank(_)] =>
                    {
                        print!("Piece Disambiguation check {:?}", pattern);
                    }

                    [AlgebraicNotationToken::Piece(_), AlgebraicNotationToken::Capture, AlgebraicNotationToken::File(_), AlgebraicNotationToken::Rank(_)] =>
                    {
                        print!("Piece capture {:?}", pattern);
                    }
                    [AlgebraicNotationToken::File(_), AlgebraicNotationToken::Capture, AlgebraicNotationToken::File(_), AlgebraicNotationToken::Rank(_)] =>
                    {
                        print!("Pawn capture {:?}", pattern);
                    }

                    [AlgebraicNotationToken::Piece(_), AlgebraicNotationToken::File(_), AlgebraicNotationToken::Rank(_), AlgebraicNotationToken::Checkmate] =>
                    {
                        print!("Piece chechmate {:?}", pattern);
                    }
                    [AlgebraicNotationToken::Piece(_), AlgebraicNotationToken::File(_), AlgebraicNotationToken::Rank(_), AlgebraicNotationToken::Check] =>
                    {
                        print!("Piece check {:?}", pattern);
                    }

                    _ => {
                        println!("ERROR token#:{} unknown move {:?}", pattern.len(), pattern)
                    }
                }
            }
            5 => {
                let pattern: &[AlgebraicNotationToken] = tokens.as_slice();
                match pattern {
                    [AlgebraicNotationToken::Piece(_), AlgebraicNotationToken::Capture, AlgebraicNotationToken::File(_), AlgebraicNotationToken::Rank(_), AlgebraicNotationToken::Checkmate] =>
                    {
                        print!("Piece move {:?}", pattern);
                    }
                    [AlgebraicNotationToken::Piece(_), AlgebraicNotationToken::Capture, AlgebraicNotationToken::File(_), AlgebraicNotationToken::Rank(_), AlgebraicNotationToken::Check] =>
                    {
                        print!("Piece move {:?}", pattern);
                    }
                    _ => {
                        println!("ERROR token#:{} unknown move {:?}", pattern.len(), pattern)
                    }
                }
            }
            _ => {
                println!("ERROR token#:{} unknown move {:?}", tokens.len(), tokens)
            }
        }

        println!();
    }
}
