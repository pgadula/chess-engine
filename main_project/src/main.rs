use chess_core::{algebraic_notation::AlgebraicNotation, bitboard::{BitBoard, FenParser}, types::{AlgebraicNotationToken, Color, FileRank}};


fn main() {
    let fen = "1nbqkbnr/pppppppp/R7/8/4P3/2rR4/PPPP1PPP/RNBQKBNR w KQkq e3 0 1";

    let mut game = BitBoard::deserialize(fen);
    let board: u64 = 0xF;

    game.print();
    game.calculate_moves();
    
    
    // game.print_attacked_squares(Color::White);
    game.print_attacked_squares(Color::White);

    // for ele in &game.white_legal_moves[FileRank::A6.index()] {
    //     print!("{:?} ", ele );
    // }
    for ele in &game.white_attacked_squares[FileRank::A3.index()] {
        print!("{:?} ", ele);
    }
    FileRank::iter().for_each(|fr| {
        let w_attacked = &game.white_attacked_squares[fr.index()];

        if !w_attacked.is_empty(){
            print!("{:?} attacked by ", fr);
            println!("{:?}", w_attacked );
        }
    });
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
