use std::{array, borrow::Borrow, sync::Arc};

use crate::{
    magic_gen::MoveLookupTable,
    moves_gen::{fill_moves, get_king_attacks, get_knight_attacks, get_pawn_moves},
    types::{
        get_piece_from_char, AlgebraicNotationToken, Attack, BoardSide, Color, FileRank, Piece,
        PieceIndex, PieceLocation, PieceType, PIECE_CHAR_ARRAY
    },
    utility::{clear_bit, get_file_ranks, get_lsb_index, pop_lsb, print_as_board, set_bit},
};

#[derive(Debug, Clone)]
pub struct BitBoard {
    pub bitboard: [u64; 12],

    pub turn: Color,
    pub castling: Castling,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
    pub en_passant: Option<FileRank>,
    pub move_lookup_table: Arc<MoveLookupTable>,

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
            self.bitboard[PieceIndex::K.index()]
        } else {
            self.bitboard[PieceIndex::k.index()]
        };
        FileRank::get_file_rank(get_lsb_index(king) as u8).unwrap()
    }

    pub fn calculate_pseudolegal_moves(&mut self) {
        let db = &self.move_lookup_table;
        let white = self.get_player_info(&Color::White);
        let black = self.get_player_info(&Color::Black);

        let (w_mask, w_flat_attacks) = self.get_pseudolegal_moves(&white);
        let (b_mask, b_flat_attacks) = self.get_pseudolegal_moves(&black);

        self.b_attacks_mask = b_mask;
        self.w_attacks_mask = w_mask;

        self.flat_white_attacks = w_flat_attacks;
        self.flat_black_attacks = b_flat_attacks;
    }

    pub fn detect_check(&self, king_mask: &u64, attacks_mask: &u64) -> bool {
        let mask = king_mask & attacks_mask;
        mask > 0
    }

    fn get_pseudolegal_moves(&self, side: &BoardSide) -> (u64, Vec<Attack>) {
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

        let mut flat_attacks: Vec<Attack> = Vec::with_capacity(50);
        let mut attack_mask = 0u64;

        for rook_position in get_file_ranks(rooks) {
            let i = rook_position.index();
            let mut rook_move: u64 =
                lookup_table.get_rook_attack(rook_position, self) & rev_friendly_blockers;
            attack_mask |= rook_move;
            fill_moves(
                rook_position,
                Piece::from(&PieceType::Rook, &color),
                rook_move,
                &mut flat_attacks,
            );
        }
        for bishop_position in get_file_ranks(bishops) {
            let i = bishop_position.index();
            let bishop_moves: u64 =
                lookup_table.get_bishop_attack(bishop_position, self) & rev_friendly_blockers;
            attack_mask |= bishop_moves;
            fill_moves(
                bishop_position,
                Piece::from(&PieceType::Bishop, &color),
                bishop_moves,
                &mut flat_attacks,
            );
        }

        for queen_position in get_file_ranks(queens) {
            let i = queen_position.index();
            let bishop_moves: u64 = lookup_table.get_bishop_attack(queen_position, &self);
            let rook_moves: u64 = lookup_table.get_rook_attack(queen_position, &self);
            let sliding_moves = (bishop_moves | rook_moves) & rev_friendly_blockers;
            attack_mask |= sliding_moves;
            fill_moves(
                queen_position,
                Piece::from(&PieceType::Queen, &color),
                sliding_moves,
                &mut flat_attacks,
            );
        }

        for knight_position in get_file_ranks(knights) {
            let i = knight_position.index();
            let attacks = get_knight_attacks(knight_position) & rev_friendly_blockers;
            attack_mask |= attacks;
            fill_moves(
                knight_position,
                Piece::from(&PieceType::Knight, &color),
                attacks,
                &mut flat_attacks,
            );
        }

        get_pawn_moves(
            side.color,
            side.pawns,
            self.empty_square(),
            side.opposite_blockers,
            &self.en_passant,
            &mut attack_mask,
            &mut flat_attacks,
        );

        for king_position in get_file_ranks(king) {
            let i = king_position.index();
            let mut attacks = get_king_attacks(king_position) & rev_friendly_blockers;
            attack_mask |= attacks;
            fill_moves(
                king_position,
                Piece::from(&PieceType::King, &color),
                attacks,
                &mut flat_attacks,
            );
        }
        (attack_mask, flat_attacks)
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
        return &mut self.bitboard[piece.bitboard_index()];
    }

    pub fn get_all_pieces(&self) -> u64 {
        return (self.get_white_pieces()) | (self.get_black_pieces());
    }
    pub fn get_white_pieces(&self) -> u64 {
        let result = self.bitboard[0..6]
            .iter()
            .copied()
            .reduce(|acc, b| acc | b)
            .unwrap_or(0u64);
        result
    }

    pub fn get_black_pieces(&self) -> u64 {
        let result = self.bitboard[6..12]
            .iter()
            .copied()
            .reduce(|acc, b| acc | b)
            .unwrap_or(0u64);
        result
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
        for piece in PIECE_CHAR_ARRAY {
            if BitBoard::get(self.bitboard[piece.bitboard_index()], file_rank) {
                return Some(piece);
            }
        }

        None
    }

    pub fn get_player_info(&self, color: &Color) -> BoardSide {
        if *color == Color::White {
            BoardSide {
                bishops: self.bitboard[PieceIndex::B.index()],
                king: self.bitboard[PieceIndex::K.index()],
                knights: self.bitboard[PieceIndex::N.index()],
                pawns: self.bitboard[PieceIndex::P.index()],
                queens: self.bitboard[PieceIndex::Q.index()],
                rooks: self.bitboard[PieceIndex::R.index()],

                opposite_bishops: self.bitboard[PieceIndex::b.index()],
                opposite_king: self.bitboard[PieceIndex::k.index()],
                opposite_knights: self.bitboard[PieceIndex::n.index()],
                opposite_pawns: self.bitboard[PieceIndex::p.index()],
                opposite_queens: self.bitboard[PieceIndex::q.index()],
                opposite_rooks: self.bitboard[PieceIndex::r.index()],
                opposite_attacks: self.b_attacks_mask,

                opposite_blockers: self.get_black_pieces(),
                friendly_blockers: self.get_white_pieces(),

                color: Color::White,
            }
        } else {
            BoardSide {
                bishops: self.bitboard[PieceIndex::b.index()],
                king: self.bitboard[PieceIndex::k.index()],
                knights: self.bitboard[PieceIndex::n.index()],
                pawns: self.bitboard[PieceIndex::p.index()],
                queens: self.bitboard[PieceIndex::q.index()],
                rooks: self.bitboard[PieceIndex::r.index()],

                opposite_bishops: self.bitboard[PieceIndex::B.index()],
                opposite_king: self.bitboard[PieceIndex::K.index()],
                opposite_knights: self.bitboard[PieceIndex::N.index()],
                opposite_pawns: self.bitboard[PieceIndex::P.index()],
                opposite_queens: self.bitboard[PieceIndex::Q.index()],
                opposite_rooks: self.bitboard[PieceIndex::R.index()],
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
            bitboard: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
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
            flat_black_attacks: Vec::new(),
            flat_white_attacks: Vec::new(),
            w_attacks_mask: 0u64,
            b_attacks_mask: 0u64,
        }
    }
}
