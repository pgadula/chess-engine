use std::thread::Thread;

use chess_core::bitboard::{FenParser, GameState};

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
    pub fn new_game(&mut self){
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
}
