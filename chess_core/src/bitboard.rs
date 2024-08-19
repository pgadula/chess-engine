use std::{array, borrow::Borrow, sync::Arc};

use crate::{
    magic_gen::MoveLookupTable,
    moves_gen::{fill_moves, get_king_attacks, get_knight_attacks, get_pawn_moves},
    types::{
        get_piece_from_char, AlgebraicNotationToken, Attack, BoardSide, Color, FileRank, Piece,
        PieceLocation, PieceType, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN,
        BLACK_ROOK, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
    },
    utility::{clear_bit, get_file_ranks, get_lsb_index, pop_lsb, print_as_board, set_bit},
};

#[derive(Debug, Clone)]
pub struct BitBoard {
    pub w_pawn: u64,
    pub w_bishop: u64,
    pub w_knight: u64,
    pub w_rook: u64,
    pub w_queen: u64,
    pub w_king: u64,
    pub b_pawn: u64,
    pub b_bishop: u64,
    pub b_knight: u64,
    pub b_rook: u64,
    pub b_queen: u64,
    pub b_king: u64,

    //state
    pub turn: Color,
    pub castling: Castling,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
    pub en_passant: Option<FileRank>,
    pub move_lookup_table: Arc<MoveLookupTable>,

    pub white_attacks_from: Vec<Vec<PieceLocation>>,
    pub white_attacked_squares: Vec<Vec<PieceLocation>>,
    pub black_attacks_from: Vec<Vec<PieceLocation>>,
    pub black_attacked_squares: Vec<Vec<PieceLocation>>,
    pub flat_white_attacks: Vec<Attack>,
    pub flat_black_attacks: Vec<Attack>,
    pub w_attacks_mask: u64,
    pub b_attacks_mask: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Castling {
    pub w_king_side: bool,
    pub w_queen_side: bool,
    pub b_king_side: bool,
    pub b_queen_side: bool,
}

pub trait FenParser {
    fn deserialize(fen: &str) -> BitBoard;
    fn serialize(self, output: &mut str) -> &str;
}

impl BitBoard {
    pub fn empty() -> BitBoard {
        Default::default()
    }
    pub fn get_king(&self, color: &Color) -> FileRank {
        let king = if *color == Color::White {
            self.w_king
        } else {
            self.b_king
        };
        FileRank::get_file_rank(get_lsb_index(king) as u8).unwrap()
    }

    pub fn calculate_pseudolegal_moves(&mut self) {
        let db = &self.move_lookup_table;
        let white = self.get_player_info(&Color::White);
        let black = self.get_player_info(&Color::Black);

        let (w_for_position, w_attacked, w_mask, w_flat_attacks) =
            self.get_pseudolegal_moves(&white);
        let (b_for_position, b_attacked, b_mask, b_flat_attacks) =
            self.get_pseudolegal_moves(&black);
        self.white_attacked_squares = w_attacked;
        self.white_attacks_from = w_for_position;

        self.black_attacked_squares = b_attacked;
        self.black_attacks_from = b_for_position;

        self.b_attacks_mask = b_mask;
        self.w_attacks_mask = w_mask;

        self.flat_white_attacks = w_flat_attacks;
        self.flat_black_attacks = b_flat_attacks;
    }

    pub fn detect_check(&self, king_mask: &u64, attacks_mask: &u64) -> bool {
        let mask = king_mask & attacks_mask;
        mask > 0
    }

    fn get_pseudolegal_moves(
        &self,
        side: &BoardSide,
    ) -> (
        Vec<Vec<PieceLocation>>,
        Vec<Vec<PieceLocation>>,
        u64,
        Vec<Attack>,
    ) {
        let BoardSide {
            mut bishops,
            mut friendly_blockers,
            mut opposite_blockers,
            mut king,
            mut queens,
            mut rooks,
            mut pawns,
            mut knights,
            color,
            ..
        } = side;
        let color = color.clone();
        let lookup_table = self.move_lookup_table.clone();
        let rev_friendly_blockers = !friendly_blockers;

        let mut moves: [Vec<PieceLocation>; 64] = array::from_fn(|_| Vec::with_capacity(64));
        let mut attacked_squares: [Vec<PieceLocation>; 64] =
            array::from_fn(|_| Vec::with_capacity(64));
        let mut flat_attacks: Vec<Attack> = Vec::with_capacity(50);
        let mut attack_mask = 0u64;

        for rook_position in get_file_ranks(rooks) {
            let i = rook_position.index();
            let position: &mut Vec<PieceLocation> = &mut moves[i];
            let mut rook_move: u64 =
                lookup_table.get_rook_attack(rook_position, self) & rev_friendly_blockers;
            attack_mask |= rook_move;
            fill_moves(
                rook_position,
                Piece::from(&PieceType::Rook, &color),
                rook_move,
                position,
                &mut attacked_squares[i],
                &mut flat_attacks,
            );
        }
        for bishop_position in get_file_ranks(bishops) {
            let i = bishop_position.index();
            let position: &mut Vec<PieceLocation> = &mut moves[i];
            let bishop_moves: u64 =
                lookup_table.get_bishop_attack(bishop_position, self) & rev_friendly_blockers;
            attack_mask |= bishop_moves;
            fill_moves(
                bishop_position,
                Piece::from(&PieceType::Bishop, &color),
                bishop_moves,
                position,
                &mut attacked_squares[i],
                &mut flat_attacks,
            );
        }

        for queen_position in get_file_ranks(queens) {
            let i = queen_position.index();
            let position: &mut Vec<PieceLocation> = &mut moves[i];
            let bishop_moves: u64 = lookup_table.get_bishop_attack(queen_position, &self);
            let rook_moves: u64 = lookup_table.get_rook_attack(queen_position, &self);
            let sliding_moves = (bishop_moves | rook_moves) & rev_friendly_blockers;
            attack_mask |= sliding_moves;
            fill_moves(
                queen_position,
                Piece::from(&PieceType::Queen, &color),
                sliding_moves,
                position,
                &mut attacked_squares[i],
                &mut flat_attacks,
            );
        }

        for knight_position in get_file_ranks(knights) {
            let i = knight_position.index();
            let position: &mut Vec<PieceLocation> = &mut moves[i];
            let attacks = get_knight_attacks(knight_position) & rev_friendly_blockers;
            attack_mask |= attacks;
            fill_moves(
                knight_position,
                Piece::from(&PieceType::Knight, &color),
                attacks,
                position,
                &mut attacked_squares[i],
                &mut flat_attacks,
            );
        }

        get_pawn_moves(
            side.color,
            side.pawns,
            self.empty_square(),
            side.opposite_blockers,
            &self.en_passant,
            &mut moves,
            &mut attacked_squares,
            &mut attack_mask,
            &mut flat_attacks,
        );

        for king_position in get_file_ranks(king) {
            let i = king_position.index();
            let position: &mut Vec<PieceLocation> = &mut moves[i];
            let mut attacks = get_king_attacks(king_position) & rev_friendly_blockers;
            attack_mask |= attacks;
            fill_moves(
                king_position,
                Piece::from(&PieceType::King, &color),
                attacks,
                position,
                &mut attacked_squares[i],
                &mut flat_attacks,
            );
        }
        let collection_of_attacked_squares = attacked_squares.to_vec();
        (
            moves.to_vec(),
            attacked_squares.to_vec(),
            attack_mask,
            flat_attacks,
        )
    }

    pub fn new_game() -> BitBoard {
        BitBoard::deserialize("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn set_piece(&mut self, piece: &Piece, file_rank: &FileRank) {
        let mut bitboard = self.get_piece_bitboard(piece, file_rank);
        set_bit(&mut bitboard, &file_rank);
    }
    pub fn clear_piece(&mut self, piece: &Piece, file_rank: &FileRank) {
        {
            let mut bitboard = self.get_piece_bitboard(piece, file_rank);
            clear_bit(&mut bitboard, file_rank);
        }
    }
    fn get_piece_bitboard(&mut self, piece: &Piece, file_rank: &FileRank) -> &mut u64 {
        return match (piece.piece_type, piece.color) {
            (PieceType::Pawn, Color::White) => &mut self.w_pawn,
            (PieceType::Bishop, Color::White) => &mut self.w_bishop,
            (PieceType::Knight, Color::White) => &mut self.w_knight,
            (PieceType::Rook, Color::White) => &mut self.w_rook,
            (PieceType::Queen, Color::White) => &mut self.w_queen,
            (PieceType::King, Color::White) => &mut self.w_king,
            (PieceType::Pawn, Color::Black) => &mut self.b_pawn,
            (PieceType::Bishop, Color::Black) => &mut self.b_bishop,
            (PieceType::Knight, Color::Black) => &mut self.b_knight,
            (PieceType::Rook, Color::Black) => &mut self.b_rook,
            (PieceType::Queen, Color::Black) => &mut self.b_queen,
            (PieceType::King, Color::Black) => &mut self.b_king,
        };
    }

    pub fn get_all_pieces(&self) -> u64 {
        return (self.get_white_pieces()) | (self.get_black_pieces());
    }

    pub fn get_white_pieces(&self) -> u64 {
        self.w_pawn | self.w_bishop | self.w_knight | self.w_rook | self.w_queen | self.w_king
    }

    pub fn get_black_pieces(&self) -> u64 {
        self.b_pawn | self.b_bishop | self.b_knight | self.b_rook | self.b_queen | self.b_king
    }

    pub fn empty_square(&self) -> u64 {
        return !self.get_all_pieces();
    }

    pub fn get(bit_board: u64, file_rank: &FileRank) -> bool {
        let file_rank_num = (*file_rank) as u8;
        let mask = 1u64 << file_rank_num;
        return (bit_board & mask) != 0;
    }

    pub fn get_by_index(bit_board: u64, index: u8) -> bool {
        let mask: u64 = 1u64 << index;
        return (bit_board & mask) != 0;
    }

    pub fn deserialize_move(&self, input: &str) -> Option<AlgebraicNotationToken> {
        let mut iterator = input.chars();
        if let Some(c) = iterator.next() {
            match c {
                'K' | 'Q' | 'R' | 'B' | 'N' | 'P' | 'k' | 'q' | 'r' | 'b' | 'n' | 'p' => {
                    return Some(AlgebraicNotationToken::Piece(c));
                }
                'a'..='h' => return Some(AlgebraicNotationToken::File(c)),
                '1'..='8' => return Some(AlgebraicNotationToken::Rank(c)),
                'x' => return Some(AlgebraicNotationToken::Capture),
                _ => {
                    return None;
                }
            }
        }
        return None;
    }

    pub fn print(&self) {
        println!("  a b c d e f g h");
        println!(" +----------------+");

        FileRank::iter().for_each(|file_rank| {
            let row = 7 - file_rank.rank();
            let col = file_rank.file();

            if col == 0 {
                if row != 7 {
                    println!("|{}", row + 2);
                }
                print!("{}|", row + 1);
            }

            let piece = self.get_piece_at(&file_rank);

            if let Some(p) = piece {
                match (p.piece_type, p.color) {
                    (PieceType::Pawn, Color::White) => print!("♙ "),
                    (PieceType::Bishop, Color::White) => print!("♗ "),
                    (PieceType::Knight, Color::White) => print!("♘ "),
                    (PieceType::Rook, Color::White) => print!("♖ "),
                    (PieceType::Queen, Color::White) => print!("♕ "),
                    (PieceType::King, Color::White) => print!("♔ "),
                    (PieceType::Pawn, Color::Black) => print!("♟︎ "),
                    (PieceType::Bishop, Color::Black) => print!("♝ "),
                    (PieceType::Knight, Color::Black) => print!("♞ "),
                    (PieceType::Rook, Color::Black) => print!("♜ "),
                    (PieceType::Queen, Color::Black) => print!("♛ "),
                    (PieceType::King, Color::Black) => print!("♚ "),
                }
            } else {
                print!(". ");
            }
        });

        println!("|1");
        println!(" +----------------+");
        println!("  a b c d e f g h")
    }

    pub fn get_piece_at(&self, file_rank: &FileRank) -> Option<Piece> {
        let pieces = [
            (self.w_pawn, WHITE_PAWN),
            (self.w_bishop, WHITE_BISHOP),
            (self.w_knight, WHITE_KNIGHT),
            (self.w_rook, WHITE_ROOK),
            (self.w_queen, WHITE_QUEEN),
            (self.w_king, WHITE_KING),
            (self.b_pawn, BLACK_PAWN),
            (self.b_bishop, BLACK_BISHOP),
            (self.b_knight, BLACK_KNIGHT),
            (self.b_rook, BLACK_ROOK),
            (self.b_queen, BLACK_QUEEN),
            (self.b_king, BLACK_KING),
        ];

        for &(bitboard, piece) in &pieces {
            if BitBoard::get(bitboard, file_rank) {
                return Some(piece);
            }
        }

        None
    }

    pub fn print_attacked_squares(&self, color: Color) {
        let mut board = [['.'; 8]; 8];

        let attacked_squares = match color {
            Color::White => &self.white_attacked_squares,
            Color::Black => &self.black_attacked_squares,
        };

        for (square, pieces) in attacked_squares.iter().enumerate() {
            if !pieces.is_empty() {
                let file_rank = FileRank::get_file_rank(square as u8).unwrap();
                let row = 7 - file_rank.rank();
                let col = file_rank.file();
                board[row as usize][col as usize] = 'x';
            }
        }

        println!("  a b c d e f g h");
        println!(" +----------------+");

        for (i, row) in board.iter().rev().enumerate() {
            print!("{}|", 8 - i);
            for col in row {
                print!("{} ", col);
            }
            println!("|{}", 8 - i);
        }

        println!(" +----------------+");
        println!("  a b c d e f g h");
    }
    pub fn get_player_info(&self, color: &Color) -> BoardSide {
        if *color == Color::White {
            BoardSide {
                bishops: self.w_bishop,
                king: self.w_king,
                knights: self.w_knight,
                pawns: self.w_pawn,
                queens: self.w_queen,
                rooks: self.w_rook,

                opposite_bishops: self.b_bishop,
                opposite_king: self.b_king,
                opposite_knights: self.b_knight,
                opposite_pawns: self.b_pawn,
                opposite_queens: self.b_queen,
                opposite_rooks: self.b_rook,
                opposite_attacks: self.b_attacks_mask,

                opposite_blockers: self.get_black_pieces(),
                friendly_blockers: self.get_white_pieces(),

                color: Color::White,
            }
        } else {
            BoardSide {
                bishops: self.b_bishop,
                king: self.b_king,
                knights: self.b_knight,
                pawns: self.b_pawn,
                queens: self.b_queen,
                rooks: self.b_rook,

                opposite_bishops: self.w_bishop,
                opposite_king: self.w_king,
                opposite_knights: self.w_knight,
                opposite_pawns: self.w_pawn,
                opposite_queens: self.w_queen,
                opposite_rooks: self.w_rook,
                opposite_attacks: self.w_attacks_mask,

                opposite_blockers: self.get_white_pieces(),
                friendly_blockers: self.get_black_pieces(),
                color: Color::Black,
            }
        }
    }
}

impl Castling {
    pub fn new() -> Castling {
        Castling {
            b_king_side: false,
            b_queen_side: false,
            w_king_side: false,
            w_queen_side: false,
        }
    }
}

impl FenParser for BitBoard {
    fn deserialize(fen: &str) -> BitBoard {
        let mut parts = fen.split_whitespace();
        let piece_placement = parts.next().unwrap_or("");
        let active_color = parts.next().unwrap_or("");
        let castling_rights = parts.next().unwrap_or("");
        let en_passant = parts.next().unwrap_or("");
        let halfmove_clock = parts.next().unwrap_or("");
        let fullmove_number = parts.next().unwrap_or("");

        let mut game = BitBoard::empty();
        let mut row: u8 = 0;
        let mut col: u8 = 0;

        for char in piece_placement.chars() {
            if let Some(piece) = get_piece_from_char(char) {
                let index = (row * 8) + col;
                if let Some(rank_file) = FileRank::get_file_rank(index) {
                    game.set_piece(piece, &rank_file);
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

        game.turn = match active_color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => game.turn, // default value
        };

        let mut castling = Castling::new();
        for char in castling_rights.chars() {
            match char {
                'K' => castling.w_king_side = true,
                'Q' => castling.w_queen_side = true,
                'k' => castling.b_king_side = true,
                'q' => castling.b_queen_side = true,
                _ => {}
            }
        }

        game.halfmove_clock = halfmove_clock.parse().unwrap_or(0);
        game.fullmove_number = fullmove_number.parse().unwrap_or(1);
        game.castling = castling;
        game.en_passant = FileRank::from_string(en_passant);

        game
    }

    fn serialize(self, _output: &mut str) -> &str {
        todo!()
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        let move_lookup_table = Arc::new(MoveLookupTable::init());
        Self {
            move_lookup_table,
            w_pawn: 0u64,
            w_bishop: 0u64,
            w_knight: 0u64,
            w_rook: 0u64,
            w_queen: 0u64,
            w_king: 0u64,
            b_pawn: 0u64,
            b_bishop: 0u64,
            b_knight: 0u64,
            b_rook: 0u64,
            b_queen: 0u64,
            b_king: 0u64,
            turn: Color::White,
            castling: Castling {
                b_king_side: false,
                b_queen_side: false,
                w_king_side: false,
                w_queen_side: false,
            },
            en_passant: None,
            halfmove_clock: 0u8,
            fullmove_number: 0u8,
            black_attacks_from: Vec::new(),
            white_attacks_from: Vec::new(),
            white_attacked_squares: Vec::new(),
            black_attacked_squares: Vec::new(),
            flat_black_attacks: Vec::new(),
            flat_white_attacks: Vec::new(),
            w_attacks_mask: 0u64,
            b_attacks_mask: 0u64,
        }
    }
}
