use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread::{self, JoinHandle},
};

use chess_core::{bitboard::GameState, fen::FenParser, types::PieceMove};

use crate::search_engine::SearchEngine;

pub struct Engine {
    pub use_multithreading: bool,
    pub is_running: bool,
    pub search_engine: SearchEngine,
    is_searching: bool,
    board: GameState,
    child_handle: Option<JoinHandle<String>>,
    cancellation_signal: Arc<AtomicBool>,
}

impl Engine {
    pub fn new() -> Self {
        let board: GameState = GameState::new_game();
        return Engine {
            board,
            is_running: true,
            is_searching: false,
            search_engine: SearchEngine::new(),
            use_multithreading: true,
            child_handle: None,
            cancellation_signal: Arc::new(AtomicBool::new(false)),
        };
    }
    pub fn go(&mut self, depth: Option<u8>) {
        self.cancellation_signal.store(false, Ordering::SeqCst);
        self.search_engine.max_depth = depth.unwrap_or(24);

        let mut search = self.search_engine.clone();
        let use_multithreading = self.use_multithreading;
        let board = self.board.clone();
        let cancellation_token = self.cancellation_signal.clone();

        // Spawn thread and store the handle
        self.child_handle = Some(thread::spawn(move || {
            if use_multithreading {
                search.rayon_search(&board, cancellation_token)
            } else {
                search.search(&board)
            }
        }));

        // Don’t `join` here – return immediately to allow background searching
        println!("Search started in background thread");
    }

    pub fn stop_thinking(&mut self) {
        self.cancellation_signal.store(true, Ordering::SeqCst);
        self.join_search();
        
    }

    pub fn join_search(&mut self) {
        // If the thread is still running, .take() the handle out of self.child

        if let Some(child_thread) = self.child_handle.take() {
            match child_thread.join() {
                Ok(result) => {
                    println!("bestmove: {:?}", result);
                }
                Err(err) => {
                    eprintln!("Thread panicked: {:?}", err);
                }
            }
        } else {
            println!("No background search to join, or already joined");
        }
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

    pub fn print(&self) {
        self.board.print();
    }

    pub fn apply_move(&mut self, uci_moves: Vec<String>) {
        self.new_game();
        for uci in uci_moves {
            let piece_move = PieceMove::from_uci(&uci, &self.board);
            self.board.make_move(&piece_move);
        }
    }
}
