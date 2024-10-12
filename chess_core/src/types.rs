    use std::{fmt::Display, ops::AddAssign, slice::Iter};

    use self::FileRank::*;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum PieceType {
        Pawn,
        Bishop,
        Knight,
        Rook,
        Queen,
        King,
    }
    impl PieceType {
        pub fn get_symbol(&self)->char{
            return match self {
                PieceType::Pawn => 'p',
                PieceType::Knight => 'n',
                PieceType::Bishop => 'b',
                PieceType::Rook => 'r',
                PieceType::Queen => 'q',
                PieceType::King => 'k',
                _ => ' '
            };
        }
    }

    pub const PROMOTION_PIECES:[PieceType; 4] = [PieceType::Bishop, PieceType::Queen, PieceType::Knight, PieceType::Rook];

    #[derive(Copy, Clone)]
    pub enum PieceIndex {
        P,
        B,
        N,
        R,
        Q,
        K,
        p,
        b,
        n,
        r,
        q,
        k,
    }
    
    impl PieceIndex {
        pub fn idx(&self)->usize{
            *self as usize
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Color {
        White,
        Black,
    }
    impl Color {
        pub fn flip(&self)->Color{
            match self {
                Color::White => Color::Black,
                Color::Black => Color::White,
            }
        }
    }


    #[derive(Debug, PartialEq, Clone, Copy)]

    pub struct Piece {
        pub piece_type: PieceType,
        pub color: Color
    }

    impl Piece {
        pub fn from(piece_type: &PieceType, color: &Color) -> Self {
            Piece { 
                piece_type: *piece_type, 
                color: *color 
            }
        }
        pub fn bitboard_index(&self)->usize{
            PIECES_ARRAY.iter().position(|p| p == self).unwrap_or_else(|| panic!("Invalid piece"))
        }

        pub fn symbol(&self)->char{
            match (self.piece_type, self.color) {
                (PieceType::Pawn, Color::White) => 'P',
                (PieceType::Pawn, Color::Black) => 'p',
                (PieceType::Knight, Color::White) => 'N',
                (PieceType::Knight, Color::Black) => 'n',
                (PieceType::Bishop, Color::White) => 'B',
                (PieceType::Bishop, Color::Black) => 'b',
                (PieceType::Rook, Color::White) => 'R',
                (PieceType::Rook, Color::Black) => 'r',
                (PieceType::Queen, Color::White) => 'Q',
                (PieceType::Queen, Color::Black) => 'q',
                (PieceType::King, Color::White) => 'K',
                (PieceType::King, Color::Black) => 'k',
            }
        }

        pub fn piece_symbol(&self) -> char {
            match (self.piece_type, self.color) {
                (PieceType::Pawn, Color::White) => '♙',
                (PieceType::Pawn, Color::Black) => '♟',
                (PieceType::Knight, Color::White) => '♘',
                (PieceType::Knight, Color::Black) => '♞',
                (PieceType::Bishop, Color::White) => '♗',
                (PieceType::Bishop, Color::Black) => '♝',
                (PieceType::Rook, Color::White) => '♖',
                (PieceType::Rook, Color::Black) => '♜',
                (PieceType::Queen, Color::White) => '♕',
                (PieceType::Queen, Color::Black) => '♛',
                (PieceType::King, Color::White) => '♔',
                (PieceType::King, Color::Black) => '♚',
            }
        }
    }

    // Define constants for each piece
    pub const WHITE_PAWN: Piece = Piece { piece_type: PieceType::Pawn, color: Color::White };
    pub const WHITE_BISHOP: Piece = Piece { piece_type: PieceType::Bishop, color: Color::White };
    pub const WHITE_KNIGHT: Piece = Piece { piece_type: PieceType::Knight, color: Color::White };
    pub const WHITE_ROOK: Piece = Piece { piece_type: PieceType::Rook, color: Color::White };
    pub const WHITE_QUEEN: Piece = Piece { piece_type: PieceType::Queen, color: Color::White };
    pub const WHITE_KING: Piece = Piece { piece_type: PieceType::King, color: Color::White };
    pub const BLACK_PAWN: Piece = Piece { piece_type: PieceType::Pawn, color: Color::Black };
    pub const BLACK_BISHOP: Piece = Piece { piece_type: PieceType::Bishop, color: Color::Black };
    pub const BLACK_KNIGHT: Piece = Piece { piece_type: PieceType::Knight, color: Color::Black };
    pub const BLACK_ROOK: Piece = Piece { piece_type: PieceType::Rook, color: Color::Black };
    pub const BLACK_QUEEN: Piece = Piece { piece_type: PieceType::Queen, color: Color::Black };
    pub const BLACK_KING: Piece = Piece { piece_type: PieceType::King, color: Color::Black };

    pub(crate) const PIECES_ARRAY: [Piece; 12] = [
        WHITE_PAWN,    // 'P'
        WHITE_BISHOP,  // 'B'
        WHITE_KNIGHT,  // 'N'
        WHITE_ROOK,    // 'R'
        WHITE_QUEEN,   // 'Q'
        WHITE_KING,    // 'K'
        BLACK_PAWN,    // 'p'
        BLACK_BISHOP,  // 'b'
        BLACK_KNIGHT,  // 'n'
        BLACK_ROOK,    // 'r'
        BLACK_QUEEN,   // 'q'
        BLACK_KING,    // 'k'
    ];

    pub fn get_piece_from_char<'a>(piece: char) -> Option<&'a Piece> {    
        let index = get_char_value(piece);
        if index < 0{
            return None;
        }
        Some(&PIECES_ARRAY[index as usize])
    }

    pub(crate) fn get_char_value(c: char) -> i32 {
        match c {
            'P' => 0,  // Pawn (White)
            'B' => 1,  // Bishop (White)
            'N' => 2,  // Knight (White)
            'R' => 3,  // Rook (White)
            'Q' => 4,  // Queen (White)
            'K' => 5,  // King (White)
            'p' => 6,  // Pawn (Black)
            'b' => 7,  // Bishop (Black)
            'n' => 8,  // Knight (Black)
            'r' => 9,  // Rook (Black)
            'q' => 10, // Queen (Black)
            'k' => 11, // King (Black)
            _ => -1,   // Character not found
        }
    }

    #[repr(u8)]
     #[derive(Clone, Copy, Debug, PartialEq)]
     pub enum FileRank {
         A8, B8, C8, D8, E8, F8, G8, H8,
         A7, B7, C7, D7, E7, F7, G7, H7,
         A6, B6, C6, D6, E6, F6, G6, H6,
         A5, B5, C5, D5, E5, F5, G5, H5,
         A4, B4, C4, D4, E4, F4, G4, H4,
         A3, B3, C3, D3, E3, F3, G3, H3,
         A2, B2, C2, D2, E2, F2, G2, H2,
         A1, B1, C1, D1, E1, F1, G1, H1
      }

    impl FileRank {
         pub fn rank(self)->u8{
            ((self as u64) / 8) as u8
         }

         pub fn file(self)->u8{
           ((self as u64) % 8) as u8
         }

         pub fn index(self)->usize{
            (self.rank() * 8 + self.file()) as usize
        }

        pub fn mask(self) -> u64 {
            1u64 << self.index()
        }
        
     }
     impl Display for FileRank {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let file = FILE_RANK_CHAR[self.index()];
            write!(f, "{file}")
        }
     }

    pub(crate) const FILE_RANK_CHAR: [&'static str; 64] = [
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"
    ];

    pub const FILE_RANK: [FileRank; 64] = [   
        A8, B8, C8, D8, E8, F8, G8, H8,
        A7, B7, C7, D7, E7, F7, G7, H7,
        A6, B6, C6, D6, E6, F6, G6, H6,
        A5, B5, C5, D5, E5, F5, G5, H5,
        A4, B4, C4, D4, E4, F4, G4, H4,
        A3, B3, C3, D3, E3, F3, G3, H3,
        A2, B2, C2, D2, E2, F2, G2, H2,
        A1, B1, C1, D1, E1, F1, G1, H1
    ];

    impl FileRank {
         pub fn iter() -> Iter<'static, FileRank> {
            FILE_RANK.iter()
         }

         pub fn get_file_rank(value: u8) -> Option<FileRank> {
            if value >= FileRank::A8 as u8 && value <= FileRank::H1 as u8 {
                Some(unsafe { std::mem::transmute(value) })
            } else {
                None
            }
        }
        pub fn get_from_mask(mask: u64) -> Option<FileRank> {
            FileRank::get_file_rank(mask.trailing_zeros() as u8)
        }

        pub fn from_string(value: &str) -> Option<Self> {
            let index = FILE_RANK_CHAR
             .iter()
             .position(|&r| r == value);
             if let Some(i) = index{
                 return Some(FILE_RANK[i])
             }
             None
         }
     }
     pub struct PieceCollection {
        pub rooks: u64,
        pub bishops: u64,
        pub queens: u64,
        pub king: u64,
        pub knights: u64,
        pub pawns: u64,
    }

    pub struct BoardSide {
        pub rooks: u64,
        pub bishops: u64,
        pub queens: u64,
        pub king: u64,
        pub knights: u64,
        pub pawns: u64,

        pub opposite_rooks: u64,
        pub opposite_bishops: u64,
        pub opposite_queens: u64,
        pub opposite_king: u64,
        pub opposite_knights: u64,
        pub opposite_pawns: u64,

        pub opposite_attacks: u64,

        pub friendly_blockers: u64,
        pub opposite_blockers: u64,
        pub color:Color,

        pub king_mask_castling: u64,
        pub queen_mask_castling: u64,


        pub castling_queen_side:bool,
        pub castling_king_side: bool
    }

    #[derive(Debug)]
    pub enum AlgebraicNotationToken {
        Piece(char),
        File(char),
        Rank(char),
        Capture,
        Check,
        Checkmate,
        CastleKingSide,
        CastleQueenSide,
        Promotion(char),
        MoveIndicator,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct PieceMove{
        pub piece:Piece,
        pub from: FileRank,
        pub target: FileRank,
        pub move_type: MoveType
    }
    impl PieceMove {
        pub fn uci(&self)->String{
            match self.move_type {
                MoveType::Promotion(promotion_piece) | MoveType::CaptureWithPromotion(promotion_piece) => {
                    return format!("{}{}{}", self.from, self.target, promotion_piece.get_symbol());
                }
                _ => {
                    return format!("{0}{1}", self.from, self.target);
                }
            }   
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum MoveType{
        Quite,
        DoublePush(Option<FileRank>),
        Capture,
        Promotion(PieceType),
        CaptureWithPromotion(PieceType),
        CastleKingSide,
        CastleQueenSide
    }

const WHITE_CASTLING_RIGHTS_MASK: u64 = 0b1100;
const BLACK_CASTLING_RIGHTS_MASK: u64 = 0b0011;

const WHITE_CASTLING_KING_MASK: u64 = 0b1000;
const WHITE_CASTLING_QUEEN_MASK: u64 = 0b0100;

const BLACK_CASTLING_KING_MASK: u64 = 0b0010;
const BLACK_CASTLING_QUEEN_MASK: u64 = 0b0001;
    #[derive(Debug, Clone, Copy)]
pub struct Castling {
    pub mask: u64
}

 impl Castling {
     pub fn new() -> Castling {
         Castling {
            mask: WHITE_CASTLING_RIGHTS_MASK | BLACK_CASTLING_RIGHTS_MASK, // Full rights by default
        }
     }
 
     pub fn disable_white_castling_rights(&mut self) {
         self.mask &= !WHITE_CASTLING_RIGHTS_MASK;  
     }
 
     pub fn disable_black_castling_rights(&mut self) {
        self.mask &= !BLACK_CASTLING_RIGHTS_MASK;  
     }

     pub fn disable_king_side(&mut self, color: &Color) {
        match color {
            Color::White => {
                self.mask &= !WHITE_CASTLING_KING_MASK;
            },
            Color::Black => {
                self.mask &= !BLACK_CASTLING_KING_MASK;
            },
        }
    }
    
    pub fn disable_queen_side(&mut self, color: &Color) {
        match color {
            Color::White => {
                self.mask &= !WHITE_CASTLING_QUEEN_MASK;
            },
            Color::Black => {
                self.mask &= !BLACK_CASTLING_QUEEN_MASK;
            },
        }
    }

    pub fn get_king_side(&self, color: &Color) -> bool {
        match color {
            Color::White => self.mask & WHITE_CASTLING_KING_MASK != 0,
            Color::Black => self.mask & BLACK_CASTLING_KING_MASK != 0,
        }
    }
    
    pub fn get_queen_side(&self, color: &Color) -> bool {
        match color {
            Color::White => self.mask & WHITE_CASTLING_QUEEN_MASK != 0,
            Color::Black => self.mask & BLACK_CASTLING_QUEEN_MASK != 0,
        }
    }

 }

 #[derive(Clone, Debug)]
 pub struct Clock(u8);
 impl Clock {
        pub fn new() -> Self {
            Clock(0)
        }
        pub fn tick(&mut self){
            self.0 +=  1;
        }

        pub fn from_string(value:&str)->Clock{
            return Clock(value.parse().unwrap_or(0));
        } 
        
        pub fn reset(&mut self){
            self.0 = 0;
        }
    }

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.0;
        write!(f, "{value}")    }
}

impl AddAssign<u8> for Clock{
    fn add_assign(&mut self, rhs: u8) {
        self.0+= rhs;
    }
}