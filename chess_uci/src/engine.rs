use std::thread::Thread;

use chess_core::{
    bitboard::{FenParser, GameState},
    types::PieceMove,
};

pub struct Engine {
    pub is_running: bool,
    is_searching: bool,
    board: Option<GameState>,
    thread: Option<Thread>,
}

impl Engine {
    pub fn new() -> Self {
        return Engine {
            board: None,
            is_running: true,
            is_searching: false,
            thread: None,
        };
    }

    pub fn new_game(&mut self) {
        self.board = Some(GameState::new_game());
    }

    pub fn from(&mut self, fen: &str) {
        self.board = Some(GameState::deserialize(fen));
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }

    pub fn stop(&mut self) {
        self.is_searching = false;
    }

    pub fn print(&self) {
        if let Some(board) = &self.board {
            board.print();
        }
    }

    pub fn apply_move(&mut self, uci_moves: Vec<String>) {
        self.new_game();
        if let Some(game_board) = &mut self.board {  
            for uci in uci_moves {
                let piece_move = PieceMove::from_uci(&uci, game_board);
                game_board.make_move(&piece_move); 
            }
        }
    }
}
