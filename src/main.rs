fn main() {
    let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
    let fen2 = "8/8/8/4p1K1/2k1P3/8/8/8 b - - 0 1";
    let fen3 = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50";

    let mut game: Game = Game::from_FEN(fen);
    game.print();
    game = Game::from_FEN(&fen2);
    println!();
    game.print();
    game = Game::from_FEN(&fen3);
    println!();

    game.print();

    // println!("flags: {:#064b}", game.w_pawn);
}

struct Game {
    w_pawn: u64,
    w_bishop: u64,
    w_knight: u64,
    w_rook: u64,
    w_queen: u64,
    w_king: u64,
    b_pawn: u64,
    b_bishop: u64,
    b_knight: u64,
    b_rook: u64,
    b_queen: u64,
    b_king: u64,

    w_turn: bool,
    w_king_side: bool,
    w_queen_side: bool,
    b_king_side: bool,
    b_queen_side: bool,
}

impl Game {
    fn new() -> Game {
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
            w_king_side: false,
            w_queen_side: false,
            b_king_side: false,
            b_queen_side: false,
        }
    }
    fn set_bit(bit_board: &mut u64, row: u8, col: u8) {
        let mask: u64 = 0x1 << row * 8 + col;
        *bit_board |= mask;
    }
    fn clear_bit(bit_board: &mut u64, row: u64, col: u64) {
        let mask: u64 = 0x1 << row * 8 + col;
        *bit_board ^= mask;
    }

    fn set_piece(&mut self, mv: (Piece, Color), row: u8, col: u8) {
        match mv {
            (Piece::Pawn, Color::White) => Game::set_bit(&mut self.w_pawn, row, col),
            (Piece::Bishop, Color::White) => Game::set_bit(&mut self.w_bishop, row, col),
            (Piece::Knight, Color::White) => Game::set_bit(&mut self.w_knight, row, col),
            (Piece::Rook, Color::White) => Game::set_bit(&mut self.w_rook, row, col),
            (Piece::Queen, Color::White) => Game::set_bit(&mut self.w_queen, row, col),
            (Piece::King, Color::White) => Game::set_bit(&mut self.w_king, row, col),
            (Piece::Pawn, Color::Black) => Game::set_bit(&mut self.b_pawn, row, col),
            (Piece::Bishop, Color::Black) => Game::set_bit(&mut self.b_bishop, row, col),
            (Piece::Knight, Color::Black) => Game::set_bit(&mut self.b_knight, row, col),
            (Piece::Rook, Color::Black) => Game::set_bit(&mut self.b_rook, row, col),
            (Piece::Queen, Color::Black) => Game::set_bit(&mut self.b_queen, row, col),
            (Piece::King, Color::Black) => Game::set_bit(&mut self.b_king, row, col),
        }
    }
    fn get(bit_board: u64, row: u8, col: u8) -> bool {
        let mask: u64 = 0x1 << row * 8 + col;
        return (bit_board & mask) != 0;
    }

    fn from_FEN(fen: &str) -> Game {
        let mut iter = fen.chars();
        let mut game = Game::new();

        let mut row: u8 = 7;
        let mut col: u8 = 0;
        while let Some(char) = iter.next() {
            match char {
                '/' => {
                    row -= 1;
                    col = 0;
                }
                'p' => {
                    game.set_piece((Piece::Pawn, Color::White), row, col);
                    col += 1;
                }
                'r' => {
                    game.set_piece((Piece::Rook, Color::White), row, col);
                    col += 1;
                }
                'b' => {
                    game.set_piece((Piece::Bishop, Color::White), row, col);
                    col += 1;
                }
                'q' => {
                    game.set_piece((Piece::Queen, Color::White), row, col);
                    col += 1;
                }
                'n' => {
                    game.set_piece((Piece::Knight, Color::White), row, col);
                    col += 1;
                }
                'k' => {
                    game.set_piece((Piece::King, Color::White), row, col);
                    col += 1;
                }

                'P' => {
                    game.set_piece((Piece::Pawn, Color::Black), row, col);
                    col += 1;
                }
                'R' => {
                    game.set_piece((Piece::Rook, Color::Black), row, col);
                    col += 1;
                }
                'B' => {
                    game.set_piece((Piece::Bishop, Color::Black), row, col);
                    col += 1;
                }
                'Q' => {
                    game.set_piece((Piece::Queen, Color::Black), row, col);
                    col += 1;
                }
                'N' => {
                    game.set_piece((Piece::Knight, Color::Black), row, col);
                    col += 1;
                }
                'K' => {
                    game.set_piece((Piece::King, Color::Black), row, col);
                    col += 1;
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
        while let Some(char) = iter.next() {
            match char {
                'b' => {
                    game.w_turn = false;
                }
                'w' => {
                    game.w_turn = true;
                }
                'k' => {
                    game.w_king_side = true;
                }
                'q' => {
                    game.w_queen_side = true;
                }
                'K' => {
                    game.b_king_side = true;
                }

                'Q' => {
                    game.b_queen_side = true;
                }

                _ => {}
            }
        }

        game
    }

    fn print(self) {
        println!("  a b c d e f g h");
        println!(" +----------------");

        for row in (0..8).rev() {
            print!("{}|", row + 1);
            for col in 0..8 {
                if Game::get(self.w_pawn, row, col) {
                    print!("♙ ");
                } else if Game::get(self.w_bishop, row, col) {
                    print!("♗ ");
                } else if Game::get(self.w_knight, row, col) {
                    print!("♘ ");
                } else if Game::get(self.w_rook, row, col) {
                    print!("♖ ");
                } else if Game::get(self.w_queen, row, col) {
                    print!("♕ ");
                } else if Game::get(self.w_king, row, col) {
                    print!("♔ ");
                } else if Game::get(self.b_pawn, row, col) {
                    print!("♟︎ ");
                } else if Game::get(self.b_bishop, row, col) {
                    print!("♝ ");
                } else if Game::get(self.b_knight, row, col) {
                    print!("♞ ");
                } else if Game::get(self.b_rook, row, col) {
                    print!("♜ ");
                } else if Game::get(self.b_queen, row, col) {
                    print!("♛ ");
                } else if Game::get(self.b_king, row, col) {
                    print!("♚ ");
                } else {
                    print!(". ");
                }
            }
            println!("|{}", row + 1);
        }

        println!(" +----------------");
        println!("  a b c d e f g h");
    }
}

enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}
enum Color {
    White,
    Black,
}
