use std::thread::JoinHandle;

use chess_core::{
    bitboard::{FenParser, GameState},
    types::PieceMove,
};

use crate::search_engine::SearchEngine;

pub struct Engine {
    pub is_running: bool,
    is_searching: bool,
    board: GameState,
    thread: Option<JoinHandle<()>>,
    search_engine: SearchEngine,
}

impl Engine {
    pub fn new() -> Self {
        let board: GameState = GameState::new_game();
        return Engine {
            board: board,
            is_running: true,
            is_searching: false,
            thread: None,
            search_engine: SearchEngine::new(6)
        };
    }

    pub fn go(&mut self) {
        let result = self.search_engine.search(&self.board);
        println!("bestmove {}", result);
    }

    pub fn new_game(&mut self) {
        self.board = GameState::new_game();
    }

    pub fn from(&mut self, fen: &str) {
        self.board = GameState::deserialize(fen);
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }

    pub fn stop(&mut self) {
        self.is_searching = false;
    }

    pub fn print(&self) {
        self.board.print();
    }

    pub fn stop_thinking() {
        todo!()
    }
    pub fn apply_move(&mut self, uci_moves: Vec<String>) {
        self.new_game();
        for uci in uci_moves {
            let piece_move = PieceMove::from_uci(&uci, &self.board);
            self.board.make_move(&piece_move);
        }
    }
}
