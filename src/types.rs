use phf::phf_map;
use std::{path::Display, slice::Iter};
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
#[derive(Debug)]

pub enum Color {
    White,
    Black,
}

pub static PIECE_CHAR_MAP: phf::Map<char, (Piece, Color)> = phf_map! {
    'p'=> (Piece::Pawn, Color::Black),
    'b'=> (Piece::Bishop, Color::Black),
    'n'=> (Piece::Knight, Color::Black),
    'r'=> (Piece::Rook, Color::Black),
    'q'=> (Piece::Queen, Color::Black),
    'k'=> (Piece::King, Color::Black),
    'P'=> (Piece::Pawn, Color::White),
    'B'=> (Piece::Bishop, Color::White),
    'N'=> (Piece::Knight, Color::White),
    'R'=> (Piece::Rook, Color::White),
    'Q'=> (Piece::Queen, Color::White),
    'K'=> (Piece::King, Color::White),
 };
 
 #[repr(u8)]
 #[derive(Clone, Copy, Debug)]
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

static _FILE_RANK_CHAR: [&'static str; 64] = [
    "A8", "B8", "C8", "D8", "E8", "F8", "G8", "H8",
    "A7", "B7", "C7", "D7", "E7", "F7", "G7", "H7",
    "A6", "B6", "C6", "D6", "E6", "F6", "G6", "H6",
    "A5", "B5", "C5", "D5", "E5", "F5", "G5", "H5",
    "A4", "B4", "C4", "D4", "E4", "F4", "G4", "H4",
    "A3", "B3", "C3", "D3", "E3", "F3", "G3", "H3",
    "A2", "B2", "C2", "D2", "E2", "F2", "G2", "H2",
    "A1", "B1", "C1", "D1", "E1", "F1", "G1", "H1"
];
static FILE_RANK: [FileRank; 64] = [   
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
     pub fn iterator() -> Iter<'static, FileRank> {

     FILE_RANK.iter()
     }

     pub fn get_file_rank(value: u8) -> Option<FileRank> {
        if value >= FileRank::A8 as u8 && value <= FileRank::H1 as u8 {
            Some(unsafe { std::mem::transmute(value) })
        } else {
            println!("value none {}",value);
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
        let fr = _FILE_RANK_CHAR[self.from as usize];
        let to = _FILE_RANK_CHAR[self.to as usize];

        write!(f, "(from: {}, to: {})", fr,to)
    }
}

pub struct Moves(pub Vec<Move>);

impl std::fmt::Display for Moves {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Moves:\n")?;
        for v in &self.0 {
            writeln!(f, "{} ", v)?;
        }
        Ok(())
    }
}