use crate::{bitboard::GameState, types::{get_piece_from_char, Castling, Clock, Color, FileRank, BLACK_CASTLING_KING_MASK, BLACK_CASTLING_QUEEN_MASK, WHITE_CASTLING_KING_MASK, WHITE_CASTLING_QUEEN_MASK}};

pub trait FenParser {
    fn deserialize(fen: &str) -> GameState;
    fn serialize(&self) -> String;
}

impl FenParser for GameState {
    fn deserialize(fen: &str) -> GameState {
        let mut parts = fen.split_whitespace();
        let piece_placement = parts.next().unwrap_or("");
        let active_color = parts.next().unwrap_or("");
        let castling_rights = parts.next().unwrap_or("");
        let en_passant = parts.next().unwrap_or("");
        let halfmove_clock = parts.next().unwrap_or("");
        let fullmove_number = parts.next().unwrap_or("");

        let mut game = GameState::empty();
        let mut row: u8 = 0;
        let mut col: u8 = 0;

        game.board = core::array::from_fn(|_|{'-'});

        for char in piece_placement.chars() {
            if let Some(piece) = get_piece_from_char(char) {
                let index = (row * 8) + col;
                if let Some(file_rank) = FileRank::get_file_rank(index) {
                    game.set_piece(piece, &file_rank);
                }
                col += 1;
            } else {
                match char {
                    '/' => {
                        row += 1;
                        col = 0;
                    }
                    '1'..='8' => {
                        if let Some(offset) = char.to_digit(10) {
                            col += offset as u8;
                        }
                    }
                    _ => {}
                }
            }
        }

        game.move_turn = match active_color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => game.move_turn, // default value
        };

        let mut castling = Castling::new();
        for char in castling_rights.chars() {
            match char {
                'K' => castling.mask |= WHITE_CASTLING_KING_MASK,
                'Q' => castling.mask |= WHITE_CASTLING_QUEEN_MASK,
                'k' => castling.mask |= BLACK_CASTLING_KING_MASK,
                'q' => castling.mask |= BLACK_CASTLING_QUEEN_MASK,
                _ => {}
            }
        }

        game.halfmove_clock = Clock::from_string(halfmove_clock);
        game.fullmove_number = Clock::from_string(fullmove_number);
        game.castling = castling;
        game.en_passant = FileRank::from_string(en_passant);

        game.hash = game.zobrist_hashing.get_hash_from_scratch(&game);
        game
    }

    fn serialize(&self) -> String {
        let mut piece_placement = String::new();
        let active_color = if self.move_turn == Color::White {
            "w"
        } else {
            "b"
        };
        let mut castling_rights = String::new();
        let mut en_passant = String::from("-");
        let halfmove_clock = self.halfmove_clock.to_string();
        let fullmove_number = self.fullmove_number.to_string();

        for rank in 0..8 {
            let mut empty_count = 0;
            for file in 0..8 {
                let index = rank * 8 + file;
                let file_rank = FileRank::get_file_rank(index).unwrap();
                if let Some(piece) = self.get_piece_at(&file_rank) {
                    if empty_count > 0 {
                        piece_placement.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    piece_placement.push(piece.symbol());
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                piece_placement.push_str(&empty_count.to_string());
            }
            if rank < 7 {
                piece_placement.push('/');
            }
        }

        if self.castling.get_king_side(&Color::White) {
            castling_rights.push('K');
        }
        if self.castling.get_queen_side(&Color::White) {
            castling_rights.push('Q');
        }
        if self.castling.get_king_side(&Color::Black) {
            castling_rights.push('k');
        }
        if self.castling.get_queen_side(&Color::Black) {
            castling_rights.push('q');
        }
        if castling_rights.is_empty() {
            castling_rights.push('-');
        }

        if let Some(en_passant_square) = self.en_passant {
            en_passant = en_passant_square.to_string();
        }

        let fen_string = format!(
            "{} {} {} {} {} {}",
            piece_placement,
            active_color,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number
        );

        fen_string
    }
}