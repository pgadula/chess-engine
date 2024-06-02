use crate::types::{Color, Piece};
use phf::phf_map;
use std::slice::Iter;
use self::FileRank::*;

static PIECE_CHAR_MAP: phf::Map<char, (Piece, Color)> = phf_map! {
   'p'=> (Piece::Pawn, Color::White),
   'b'=> (Piece::Bishop, Color::White),
   'n'=> (Piece::Knight, Color::White),
   'r'=> (Piece::Rook, Color::White),
   'q'=> (Piece::Queen, Color::White),
   'k'=> (Piece::King, Color::White),
   'P'=> (Piece::Pawn, Color::Black),
   'B'=> (Piece::Bishop, Color::Black),
   'N'=> (Piece::Knight, Color::Black),
   'R'=> (Piece::Rook, Color::Black),
   'Q'=> (Piece::Queen, Color::Black),
   'K'=> (Piece::King, Color::Black),
};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum FileRank {
   A1,B1,C1,D1,E1,F1,G1,H1,
   A2,B2,C2,D2,E2,F2,G2,H2,
   A3,B3,C3,D3,E3,F3,G3,H3,
   A4,B4,C4,D4,E4,F4,G4,H4,
   A5,B5,C5,D5,E5,F5,G5,H5,
   A6,B6,C6,D6,E6,F6,G6,H6,
   A7,B7,C7,D7,E7,F7,G7,H7,
   A8,B8,C8,D8,E8,F8,G8,H8
}

impl FileRank {
    pub fn iterator() -> Iter<'static, FileRank> {
    static FILE_RANK: [FileRank; 64] = [   
       A1, B1, C1, D1, E1, F1, G1, H1,
       A2, B2, C2, D2, E2, F2, G2, H2,
       A3, B3, C3, D3, E3, F3, G3, H3,
       A4, B4, C4, D4, E4, F4, G4, H4,
       A5, B5, C5, D5, E5, F5, G5, H5,
       A6, B6, C6, D6, E6, F6, G6, H6,
       A7, B7, C7, D7, E7, F7, G7, H7,
       A8, B8, C8, D8, E8, F8, G8, H8
    ];
    FILE_RANK.iter()
    }
}
fn get_file_rank(value: u8) -> Option<FileRank> {
    if value >= FileRank::A1 as u8 && value <= FileRank::H8 as u8 {
        Some(unsafe { std::mem::transmute(value) })
    } else {
        None
    }
}

pub struct Game {
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
    pub w_turn: bool,
    pub castling: Castling,
}

impl Game {
    pub fn empty() -> Game {
        Game {
            w_pawn: 0,
            w_bishop: 0,
            w_knight: 0,
            w_rook: 0,
            w_queen: 0,
            w_king: 0,
            b_pawn: 0,
            b_bishop: 0,
            b_knight: 0,
            b_rook: 0,
            b_queen: 0,
            b_king: 0,
            w_turn: false,
            castling: Castling {
                b_king_side: false,
                b_queen_side: false,
                w_king_side: false,
                w_queen_side: false,
            },
        }
    }
    pub fn from_fen(fen: &str) -> Game {
        let mut iter = fen.chars();
        let mut game = Game::empty();

        let mut row: u8 = 0;
        let mut col: u8 = 0;
        while let Some(char) = iter.next() {
            if let Some(piece) = PIECE_CHAR_MAP.get(&char) {
                let rank_file: FileRank = get_file_rank((row * 8) + col).unwrap();
                game.set_piece(piece, rank_file);
                col += 1;
            } else {
                match char {
                    '/' => {
                        row += 1;
                        col = 0;
                    }
                    '1'..='9' => {
                        let offset = char.to_digit(10).unwrap() as u8;
                        col += offset;
                    }
                    ' ' => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        let mut castling = Castling::new();
        while let Some(char) = iter.next() {
            match char {
                'b' => {
                    game.w_turn = false;
                }
                'w' => {
                    game.w_turn = true;
                }
                'k' => {
                    castling.w_king_side = true;
                }
                'q' => {
                    castling.w_queen_side = true;
                }
                'K' => {
                    castling.b_king_side = true;
                }

                'Q' => {
                    castling.b_queen_side = true;
                }

                _ => {}
            }
        }
        game.castling = castling;

        game
    }

    pub fn new_game() -> Game {
        Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
    pub fn set_bit(bit_board: &mut u64, file_rank: FileRank) {
        let file_rank_num = file_rank as u8;

        let mask: u64 = 0x1 << file_rank_num;
        *bit_board |= mask;
    }
    pub fn clear_bit(bit_board: &mut u64, file_rank: FileRank) {
        let file_rank_num = file_rank as u8;

        let mask: u64 = 0x1 << file_rank_num;
        *bit_board ^= mask;
    }

    fn generate_move() {
        todo!()
    }

    pub fn set_piece(&mut self, mv: &(Piece, Color), file_rank: FileRank) {
        match mv {
            (Piece::Pawn, Color::White) => Game::set_bit(&mut self.w_pawn, file_rank),
            (Piece::Bishop, Color::White) => Game::set_bit(&mut self.w_bishop, file_rank),
            (Piece::Knight, Color::White) => Game::set_bit(&mut self.w_knight, file_rank),
            (Piece::Rook, Color::White) => Game::set_bit(&mut self.w_rook, file_rank),
            (Piece::Queen, Color::White) => Game::set_bit(&mut self.w_queen, file_rank),
            (Piece::King, Color::White) => Game::set_bit(&mut self.w_king, file_rank),
            (Piece::Pawn, Color::Black) => Game::set_bit(&mut self.b_pawn, file_rank),
            (Piece::Bishop, Color::Black) => Game::set_bit(&mut self.b_bishop, file_rank),
            (Piece::Knight, Color::Black) => Game::set_bit(&mut self.b_knight, file_rank),
            (Piece::Rook, Color::Black) => Game::set_bit(&mut self.b_rook, file_rank),
            (Piece::Queen, Color::Black) => Game::set_bit(&mut self.b_queen, file_rank),
            (Piece::King, Color::Black) => Game::set_bit(&mut self.b_king, file_rank),
        }
    }
    pub fn get(bit_board: u64, file_rank: FileRank) -> bool {
        let file_rank_num = file_rank as u8;
        let mask: u64 = 0x1 << file_rank_num;
        return (bit_board & mask) != 0;
    }

    pub fn print(self) {
        println!("  a b c d e f g h");
        println!(" +----------------");

        FileRank::iterator().enumerate().for_each(|(index, file_rank)| {
            let f_r = file_rank.clone();
            let row = 7 - (index / 8); // Calculate the row in reverse
            let col = index % 8;

            // Print the row number at the start of each row
            if col == 0 {
                if row != 7 { // To avoid an extra newline before the first row
                    println!("|{}", row + 2);
                }
                print!("{}|", row + 1);
            }

            if Game::get(self.w_pawn, f_r) {
                print!("♟︎ ");
            } else if Game::get(self.w_bishop, f_r) {
                print!("♝ ");
            } else if Game::get(self.w_knight, f_r) {
                print!("♞ ");
            } else if Game::get(self.w_rook, f_r) {
                print!("♜ ");
            } else if Game::get(self.w_queen, f_r) {
                print!("♛ ");
            } else if Game::get(self.w_king, f_r) {
                print!("♚ ");
            } else if Game::get(self.b_pawn, f_r) {
                print!("♙ ");
            } else if Game::get(self.b_bishop, f_r) {
                print!("♗ ");
            } else if Game::get(self.b_knight, f_r) {
                print!("♘ ");
            } else if Game::get(self.b_rook, f_r) {
                print!("♖ ");
            } else if Game::get(self.b_queen, f_r) {
                print!("♕ ");
            } else if Game::get(self.b_king, f_r) {
                print!("♔ ");
            } else {
                print!(". ");
            }
        });

        println!("|1");
        println!(" +----------------");
        println!("  a b c d e f g h");
    }
}


pub struct Castling {
    w_king_side: bool,
    w_queen_side: bool,
    b_king_side: bool,
    b_queen_side: bool,
}
impl Castling {
    fn new() -> Castling {
        Castling {
            b_king_side: false,
            b_queen_side: false,
            w_king_side: false,
            w_queen_side: false,
        }
    }
}
