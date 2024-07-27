use std::{array, iter::empty, sync::Arc};

use base_types::{get_piece_from_char, AlgebraicNotationToken, Color, FileRank, Piece, PieceLocation, PieceType, BoardSide, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK};
use file_rank::FILES_CHAR;
use magic_gen::MoveLookupTable;
use moves_gen::{fill_moves, get_king_attacks, get_knight_attacks, get_pawn_moves};
use utility::bits::{clear_bit, pop_lsb, set_bit};
pub mod algebraic_notation;
pub mod base_types;
pub mod file_rank;
pub mod magic_gen;
pub mod moves_gen;
mod precalculated;
pub mod utility;

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
    pub db: Arc<MoveLookupTable>,

    pub white_moves: Vec<Vec<usize>>,
    pub black_moves: Vec<Vec<usize>>,
    pub attacked_squared: Vec<Vec<usize>>,
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

    pub fn calculate_moves(&mut self) {
        let mut move_counter: u8 = 0;
        let db = &self.db;
        let white = BoardSide {
            rooks: self.w_rook,
            bishops: self.w_bishop,
            queens: self.w_queen,
            king: self.w_king,
            pawns: self.w_pawn,
            knights: self.w_knight,
            friendly_blockers: self.get_white_pieces(),
            color: Color::White,
        };
        let black = BoardSide {
            rooks: self.b_rook,
            bishops: self.b_bishop,
            queens: self.b_queen,
            king: self.b_king,
            pawns: self.b_pawn,
            knights: self.b_knight,
            friendly_blockers: self.get_black_pieces(),
            color: Color::Black,
        };

        self.white_moves = self.get_moves_for_color(&white);
        self.black_moves = self.get_moves_for_color(&black);
    }

    fn get_moves_for_color(&self, side: &BoardSide) -> Vec<Vec<usize>> {
        let BoardSide {
            mut bishops,
            mut friendly_blockers,
            mut king,
            mut queens,
            mut rooks,
            mut pawns,
            mut knights,
            color,
        } = side;
        let color = color.clone();
        let db = self.db.clone();
        let rev_friendly_blockers = !friendly_blockers;

        let mut moves: [Vec<usize>; 64] = array::from_fn(|_| Vec::with_capacity(64));
        let mut attacked_squares: [Vec<PieceLocation>; 64] =
            array::from_fn(|_| Vec::with_capacity(64));

        while rooks > 0 {
            let i = pop_lsb(&mut rooks) as usize;
            let position: &mut Vec<usize> = &mut moves[i];
            let file_rank = FileRank::get_file_rank(i as u8).unwrap();
            let mut rook_move: u64 = db.get_rook_attack(file_rank, self) & rev_friendly_blockers;

            fill_moves(file_rank, PieceType::Rook, rook_move, position);
            for index in position {
                attacked_squares[*index as usize].push(PieceLocation {
                    file_rank,
                    piece: PieceType::Rook,
                    color,
                })
            }
        }

        while bishops > 0 {
            let index = pop_lsb(&mut bishops) as usize;
            let file_rank = FileRank::get_file_rank(index as u8).unwrap();
            let position: &mut Vec<usize> = &mut moves[index];
            let mut bishop_moves: u64 =
                db.get_bishop_attack(file_rank, self) & rev_friendly_blockers;
            fill_moves(file_rank, PieceType::Bishop, bishop_moves, position);
            for index in position {
                attacked_squares[*index as usize].push(PieceLocation {
                    file_rank,
                    piece: PieceType::Bishop,
                    color,
                })
            }
        }

        while queens > 0 {
            let index = pop_lsb(&mut queens) as usize;
            let file_rank = FileRank::get_file_rank(index as u8).unwrap();
            let position: &mut Vec<usize> = &mut moves[index];
            let mut bishop_moves: u64 = db.get_bishop_attack(file_rank, self);
            let mut rook_moves: u64 = db.get_rook_attack(file_rank, self);
            let mut sliding_moves = (bishop_moves | rook_moves) & rev_friendly_blockers;
            fill_moves(file_rank, PieceType::Queen, sliding_moves, position);
            for index in position {
                attacked_squares[*index as usize].push(PieceLocation {
                    file_rank,
                    piece: PieceType::Queen,
                    color,
                })
            }
        }

        while knights > 0 {
            let index = pop_lsb(&mut knights) as usize;
            let file_rank = FileRank::get_file_rank(index as u8).unwrap();
            let position: &mut Vec<usize> = &mut moves[index];
            let mut attacks = get_knight_attacks(file_rank) & rev_friendly_blockers;
            fill_moves(file_rank, PieceType::Knight, attacks, position);
            for index in position {
                attacked_squares[*index as usize].push(PieceLocation {
                    file_rank,
                    piece: PieceType::Knight,
                    color,
                })
            }
        }

        let pawns = if color == Color::White {
            self.w_pawn
        } else {
            self.b_pawn
        };
        get_pawn_moves(color, pawns, self.empty_square(), &mut moves);

        while king > 0 {
            let index = pop_lsb(&mut king) as usize;
            let file_rank = FileRank::get_file_rank(index as u8).unwrap();
            let position: &mut Vec<usize> = &mut moves[index];
            let mut attacks = get_king_attacks(file_rank) & rev_friendly_blockers;
            fill_moves(file_rank, PieceType::King, attacks, position);
            for index in position {
                attacked_squares[*index as usize].push(PieceLocation {
                    file_rank,
                    piece: PieceType::King,
                    color,
                })
            }
        }

        moves.to_vec()
    }

    pub fn new_game() -> BitBoard {
        BitBoard::deserialize("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn set_piece(&mut self, piece: &Piece, file_rank: FileRank) {
        match (piece.piece_type, piece.color) {
            (PieceType::Pawn, Color::White) => set_bit(&mut self.w_pawn, file_rank),
            (PieceType::Bishop, Color::White) => set_bit(&mut self.w_bishop, file_rank),
            (PieceType::Knight, Color::White) => set_bit(&mut self.w_knight, file_rank),
            (PieceType::Rook, Color::White) => set_bit(&mut self.w_rook, file_rank),
            (PieceType::Queen, Color::White) => set_bit(&mut self.w_queen, file_rank),
            (PieceType::King, Color::White) => set_bit(&mut self.w_king, file_rank),
            (PieceType::Pawn, Color::Black) => set_bit(&mut self.b_pawn, file_rank),
            (PieceType::Bishop, Color::Black) => set_bit(&mut self.b_bishop, file_rank),
            (PieceType::Knight, Color::Black) => set_bit(&mut self.b_knight, file_rank),
            (PieceType::Rook, Color::Black) => set_bit(&mut self.b_rook, file_rank),
            (PieceType::Queen, Color::Black) => set_bit(&mut self.b_queen, file_rank),
            (PieceType::King, Color::Black) => set_bit(&mut self.b_king, file_rank),
        }
    }
    
    pub fn clear_piece(&mut self, piece: &Piece, file_rank: FileRank) {
        match (piece.piece_type, piece.color) {
            (PieceType::Pawn, Color::White) => clear_bit(&mut self.w_pawn, file_rank),
            (PieceType::Bishop, Color::White) => clear_bit(&mut self.w_bishop, file_rank),
            (PieceType::Knight, Color::White) => clear_bit(&mut self.w_knight, file_rank),
            (PieceType::Rook, Color::White) => clear_bit(&mut self.w_rook, file_rank),
            (PieceType::Queen, Color::White) => clear_bit(&mut self.w_queen, file_rank),
            (PieceType::King, Color::White) => clear_bit(&mut self.w_king, file_rank),
            (PieceType::Pawn, Color::Black) => clear_bit(&mut self.b_pawn, file_rank),
            (PieceType::Bishop, Color::Black) => clear_bit(&mut self.b_bishop, file_rank),
            (PieceType::Knight, Color::Black) => clear_bit(&mut self.b_knight, file_rank),
            (PieceType::Rook, Color::Black) => clear_bit(&mut self.b_rook, file_rank),
            (PieceType::Queen, Color::Black) => clear_bit(&mut self.b_queen, file_rank),
            (PieceType::King, Color::Black) => clear_bit(&mut self.b_king, file_rank),
        }
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

    pub fn get(bit_board: u64, file_rank: FileRank) -> bool {
        let file_rank_num = file_rank as u8;
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

            let piece = self.get_piece_at(file_rank.clone());

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
            }else{
              print!(". ");
            }

        });

        println!("|1");
        println!(" +----------------+");
        println!("  a b c d e f g h")
    }

    pub fn get_piece_at(&self, file_rank: FileRank) -> Option<Piece> {
        if BitBoard::get(self.w_pawn, file_rank) {
            return Some(WHITE_PAWN)
        } else if BitBoard::get(self.w_bishop, file_rank) {
            return Some(WHITE_BISHOP)
        } else if BitBoard::get(self.w_knight, file_rank) {
            return Some(WHITE_KNIGHT)
        } else if BitBoard::get(self.w_rook, file_rank) {
            return Some(WHITE_ROOK)
        } else if BitBoard::get(self.w_queen, file_rank) {
            return Some(WHITE_QUEEN)
        } else if BitBoard::get(self.w_king, file_rank) {
            return Some(WHITE_KING)
        } else if BitBoard::get(self.b_pawn, file_rank) {
            return Some(BLACK_PAWN)
        } else if BitBoard::get(self.b_bishop, file_rank) {
            return Some(BLACK_BISHOP)
        } else if BitBoard::get(self.b_knight, file_rank) {
            return Some(BLACK_KNIGHT)
        } else if BitBoard::get(self.b_rook, file_rank) {
            return Some(BLACK_ROOK)
        } else if BitBoard::get(self.b_queen, file_rank) {
            return Some(BLACK_QUEEN)
        } else if BitBoard::get(self.b_king, file_rank) {
            return Some(BLACK_KING)
        }
        None
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
        let _en_passant = parts.next().unwrap_or("");
        let halfmove_clock = parts.next().unwrap_or("");
        let fullmove_number = parts.next().unwrap_or("");

        let mut game = BitBoard::empty();
        let mut row: u8 = 0;
        let mut col: u8 = 0;

        for char in piece_placement.chars() {
            if let Some(piece) = get_piece_from_char(char) {
                let index = (row * 8) + col;
                if let Some(rank_file) = FileRank::get_file_rank(index) {
                    game.set_piece(piece, rank_file);
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

        game
    }

    fn serialize(self, _output: &mut str) -> &str {
        todo!()
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        let db = Arc::new(MoveLookupTable::init());
        Self {
            db,
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
            halfmove_clock: 0u8,
            fullmove_number: 0u8,
            black_moves: Vec::new(),
            white_moves: Vec::new(),
            attacked_squared: Vec::new(),
        }
    }
}
