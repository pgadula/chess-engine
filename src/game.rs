use crate::types::{Color, FileRank, Piece, PIECE_CHAR_MAP};

#[derive(Debug, Clone, Copy)]
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
    pub halfmove_clock:u8,
    pub fullmove_number:u8
}

#[derive(Debug, Clone, Copy)]
pub struct Castling {
    pub w_king_side: bool,
    pub w_queen_side: bool,
    pub b_king_side: bool,
    pub b_queen_side: bool,
}

pub trait FenParser {
    fn deserialize(fen: &str) -> Game;
    fn serialize(self, output: &mut str) -> &str;
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
            halfmove_clock:0,
            fullmove_number:0
        }
    }
   
    pub fn new_game() -> Game {
        Game::deserialize("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn set_bit(bit_board: &mut u64, file_rank: FileRank) {
        let file_rank_num = file_rank as u8;
        let mask: u64 = 0x1 << file_rank_num;
        *bit_board |= mask;
    }
    pub fn set_bit_by_index(bit_board: &mut u64, index:u8) {
        let mask: u64 = 0x1 << index;
        *bit_board |= mask;
    }

    pub fn clear_bit(bit_board: &mut u64, file_rank: FileRank) {
        let file_rank_num = file_rank as u8;
        let mask: u64 = 0x1 << file_rank_num;
        *bit_board ^= mask;
    }
    
    pub fn clear_bit_by_index(bit_board: &mut u64, index:u8) {
        let mask: u64 = 0x1 << index;
        *bit_board ^= mask;
    }
    
    //Kernighan’s algorithm
    pub fn bit_count(bit_board: u64) -> usize {
        let mut b = bit_board;
        let mut count = 0;
        while b != 0 {
            b &= b - 1; // Clears the lowest set bit
            count += 1;
        }
        count
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
    pub fn get_all_pieces(self)->u64{        
        return 
        self.w_pawn   |
        self.w_bishop |
        self.w_knight | 
        self.w_rook   |
        self.w_queen  |
        self.w_king   |
        self.b_pawn   |
        self.b_bishop |
        self.b_knight | 
        self.b_rook   |
        self.b_queen  |
        self.b_king   
    }
    pub fn empty_square(self)->u64{
        return !self.get_all_pieces()
    }

    pub fn get(bit_board: u64, file_rank: FileRank) -> bool {
        let file_rank_num = file_rank as u8;
        let mask: u64 = 0x1 << file_rank_num;
        return (bit_board & mask) != 0;
    }

    pub fn print(self) {
        println!("  a b c d e f g h");
        println!(" +----------------+");

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
        println!(" +----------------+");
        println!("  a b c d e f g h");
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

impl FenParser for Game {
    fn deserialize(fen: &str) -> Game {
       let mut parts = fen.split_whitespace();
       let piece_placement = parts.next().unwrap_or("");
       let active_color = parts.next().unwrap_or("");
       let castling_rights = parts.next().unwrap_or("");
       let _en_passant = parts.next().unwrap_or("");
       let halfmove_clock = parts.next().unwrap_or("");
       let fullmove_number = parts.next().unwrap_or("");
   
       let mut game = Game::empty();
       let mut row: u8 = 0;
       let mut col: u8 = 0;
   
       for char in piece_placement.chars() {
           if let Some(piece) = PIECE_CHAR_MAP.get(&char) {
               let index = (row * 8) + col;
               if let Some(rank_file) = FileRank::get_file_rank(index) {
                   game.set_piece(piece, rank_file);
                   // println!("Placed piece {:?} {:?} at {:?}", piece.0, piece.1, rank_file);

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
   
       game.w_turn = match active_color {
           "w" => true,
           "b" => false,
           _ => game.w_turn, // default value
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
