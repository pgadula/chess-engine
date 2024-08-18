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

    // for test_position in TEST_CASES.iter().filter(|e| e.depth == 1) {
    // }
    debug_move_generator(&TEST_CASES[0]);


        // for attack in valid_attacks {
        //     println!("{:?}", attack);
        // }
        // let n_attacks = &valid_attacks.count();
        // if  (*n_attacks) == 0 {
        //     println!("Check mate");
        // }
        // for (index, ele) in game.black_attacked_squares.iter().enumerate() {
        //     let square_file_rank = FileRank::get_file_rank(index as u8).unwrap();
        //     for p in ele {
        //         let mut clone_game = game.clone();
        //         clone_game.clear_piece(&p.piece, &p.file_rank);
        //         clone_game.set_piece(&p.piece, &square_file_rank);
        //         if let Some(en_passant_file_rank) = clone_game.en_passant{
        //             if en_passant_file_rank == square_file_rank {
        //                 println!("{:?}", en_passant_file_rank);
        //                 let upper_file_rank = FileRank::get_file_rank((en_passant_file_rank.index() - 8) as  u8).unwrap();
        //                 println!("upper file rank {:?}", upper_file_rank);
        //                 clone_game.clear_piece(  &Piece{
        //                     color: Color::White,
        //                     piece_type: PieceType::Pawn
        //                 }, &upper_file_rank)
        //             }

        //         }

        //         clone_game.calculate_moves();

        //         let has_check = clone_game.detect_check(&clone_game.b_king, &clone_game.w_attacks_mask);
        //         if has_check == false {
        //             clone_game.print();
        //             println!("valid :{:?} {:?}", &square_file_rank, &p.piece);
        //         }
        //     }
        // }
        // println!("FEN: {:?}", ele.fen);
        // println!("en_passant: {:?}", game.en_passant);

        // for mv in &game.black_attacks_from {
        //     println!("moves: {:?}", mv);
        // }
        // print_heatmap(&game);

        // print_as_board(game.b_attacks_mask);
        // println!("expected nodes: {}, received: {}", ele.nodes, count);

    // let board: u64 = 0xF;

    // game.print();

    // // game.print_attacked_squares(Color::White);
    // game.print_attacked_squares(Color::White);
    // let heatmap = print_heatmap(&game);
    // println!("heatmap {:?}", heatmap);

    // // for ele in &game.white_legal_moves[FileRank::A6.index()] {
    // //     print!("{:?} ", ele );
    // // }
    // for ele in &game.white_attacked_squares[FileRank::A3.index()] {
    //     print!("{:?} ", ele);
    // }
    // FileRank::iter().for_each(|fr| {
    //     let w_attacked = &game.white_attacked_squares[fr.index()];

    //     if !w_attacked.is_empty() {
    //         print!("{:?} attacked by ", fr);
    //         println!("{:?}", w_attacked);
    //     }
    // });
    // let notations = [
    //     "e4",
    //     "d5",
    //     "Nf3",
    //     "Ng5",
    //     "Bc4",
    //     "Bf6",
    //     "Ra1",
    //     "Rd8",
    //     "Qe2",
    //     "Qh5",
    //     "Ke1",
    //     "Kg8",
    //     "exd5",
    //     "cxd4",
    //     "Nxf7",
    //     "Bxc6",
    //     "O-O",
    //     "O-O-O",
    //     "Qe7+",
    //     "Rxf7+",
    //     "Qh7#",
    //     "Rg8#",
    //     "e8=Q",
    //     "a1=R",
    //     "exd6 e.p.",
    //     "Nbd2",
    //     "R1d1",
    // ];

    //    let notation = [
    //         "Nbd2",           // Disambiguation
    //         "R1d1",           // Disambiguation
    //         "e8=Q+",          // Pawn promotion with check
    //         "a1=R#",          // Pawn promotion with checkmate
    //         "exd6 e.p. +",    // En Passant with check
    //         "exd6 e.p. #",    // En Passant with checkmate
    //         "Nf7+ Nxd8++",    // Double check
    //         "Rxf7 Nf6+",      // Multi-move notation
    //         "1. e4 e5 2. Nf3 Nc6 3. O-O", // Complex sequence
    //         "Qd1",            // Non-standard notation
    //         "Kf1"             // Non-standard notation
    //     ];
    //     parse_notation(notation.to_vec());
}

fn debug_move_generator(test_position: &test_cases::TestPosition) {
    let mut game: BitBoard = BitBoard::deserialize(&test_position.fen);
    game.calculate_moves();


    let valid_attacks:Vec<&Attack> = game
        .flat_black_attacks
        .iter()
        .map(|attack| {
            let mut cloned_game: BitBoard = game.clone();

            if let Some(target_piece) = cloned_game.get_piece_at(&attack.target) {
                cloned_game.clear_piece(&target_piece, &attack.target)
            };
            cloned_game.set_piece(&attack.piece, &attack.target);
            cloned_game.clear_piece(&attack.piece, &attack.from);
            if let Some(en_passant_file_rank) = cloned_game.en_passant {
                if en_passant_file_rank == attack.target {
                    let file_rank_mask = en_passant_file_rank.mask();
                    let target_file_rank = if (file_rank_mask & RANK_3) > 0 {
                        FileRank::get_from_mask(file_rank_mask >> 8).unwrap()
                    } else {
                        FileRank::get_from_mask(file_rank_mask << 8).unwrap()
                    };
                    cloned_game.clear_piece(
                        &Piece {
                            color: cloned_game.turn.opposite(),
                            piece_type: PieceType::Pawn,
                        },
                        &target_file_rank,
                    )
                }
            }

            cloned_game.calculate_moves();
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
