use std::slice::Iter;
use self::FileRank::*;

#[derive(Debug)]
pub enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

static PIECE_CHAR_ARRAY: [(Piece, Color); 12] = 
[
    (Piece::Pawn, Color::White),   // 'P'
    (Piece::Bishop, Color::White), // 'B'
    (Piece::Knight, Color::White), // 'N'
    (Piece::Rook, Color::White),   // 'R'
    (Piece::Queen, Color::White),  // 'Q'
    (Piece::King, Color::White),   // 'K'
    (Piece::Pawn, Color::Black),   // 'p'
    (Piece::Bishop, Color::Black), // 'b'
    (Piece::Knight, Color::Black), // 'n'
    (Piece::Rook, Color::Black),   // 'r'
    (Piece::Queen, Color::Black),  // 'q'
    (Piece::King, Color::Black)    // 'k'
];

pub fn get_piece_from_char<'a>(piece: char) -> Option<&'a (Piece, Color)> {    
    let index = get_char_value(piece);
    if index < 0{
        return None;
    }
    Some(&PIECE_CHAR_ARRAY[index as usize])
}
fn get_char_value(c: char) -> i32 {
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
 } 

const FILE_RANK_CHAR: [&'static str; 64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "r8", "h8",
    "a7", "b7", "c7", "d7", "e7", "f7", "r7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "r6", "h6",
    "a5", "b5", "c5", "d5", "e5", "f5", "r5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "r4", "h4",
    "a3", "b3", "c3", "d3", "e3", "f3", "r3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "r2", "h2",
    "a1", "b1", "c1", "d1", "e1", "f1", "r1", "h1"
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

    /// get iterator with order starting from A8 to H1
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
 }

 #[derive(Clone, Copy, Debug)]
 pub struct Move {
    pub from: u8,
    pub to: u8,
 }

 impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let from = FILE_RANK_CHAR[self.from as usize];
        let to: &str = FILE_RANK_CHAR[self.to as usize];

        write!(f, "from: {}, to: {}", from, to)
    }
}

impl std::fmt::Display for Moves {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Moves:\n")?;
        for v in &self.0 {
            writeln!(f, "{} ", v)?;
        }
        Ok(())
    }
}
pub struct Moves(pub Vec<Move>);

pub struct Side {
    pub rooks: u64,
    pub bishops: u64,
    pub queens: u64,
    pub king: u64,
    pub knights: u64,
    pub pawns: u64,
    pub friendly_blockers: u64,
}

#[derive(Debug)]
pub enum Token {
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