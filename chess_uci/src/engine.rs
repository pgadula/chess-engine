use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Instant,
};

use chess_core::{bitboard::GameState, fen::FenParser, types::PieceMove};

use crate::{lexer::Lexer, search_engine::SearchEngine};

pub struct Engine {
    pub use_multithreading: bool,
    pub is_running: bool,
    pub search_engine: SearchEngine,
    pub child_handle: Option<JoinHandle<String>>,
    board: GameState,
    cancellation_signal: Arc<AtomicBool>,
    start_thinking_time: Option<Instant>,
}

impl Engine {
    pub fn new() -> Self {
        let board: GameState = GameState::new_game();
        return Engine {
            board,
            is_running: true,
            search_engine: SearchEngine::new(),
            use_multithreading: true,
            child_handle: None,
            cancellation_signal: Arc::new(AtomicBool::new(false)),
            start_thinking_time: None,
        };
    }
    pub fn go(&mut self, depth: Option<u8>) {
        self.cancellation_signal.store(false, Ordering::SeqCst);
        self.search_engine.max_depth = depth.unwrap_or(24);
        self.start_thinking_time = Some(Instant::now());

        let mut search = self.search_engine.clone();
        let use_multithreading = self.use_multithreading;
        let board = self.board.clone();
        let cancellation_token = self.cancellation_signal.clone();

        self.child_handle = Some(thread::spawn(move || {
            if use_multithreading {
                search.rayon_search(&board, cancellation_token)
            } else {
                search.search(&board)
            }
        }));
    }

    pub fn stop_thinking(&mut self) {
        self.cancellation_signal.store(true, Ordering::Relaxed);
        if let Some(now) = self.start_thinking_time {
            let elapsed = now.elapsed();
            println!("Thinking time: {:.2?}", elapsed);
        }
        self.start_thinking_time = None;
        self.join_search();
    }

    fn join_search(&mut self) {
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

    pub fn process_command(&mut self, chars: &Vec<char>) {
        let mut lexer = Lexer::from(chars);
        while let Some(token) = lexer.next() {
            match token.as_str() {
                "uci" => {
                    println!("id name PrzemChess Chess");
                    println!("id author Przemyslaw Gadula");
                    println!("uciok");
                }
                "isready" => {
                    println!("readyok");
                }
                "position" => {
                    if let Some(next_token) = lexer.next() {
                        match next_token.as_str() {
                            "startpos" => {
                                let mut tokens = Vec::new(); // Buffer to collect tokens

                                while let Some(token) = lexer.next() {
                                    if token == "moves" {
                                        continue;
                                    }
                                    tokens.push(token);
                                }

                                let uci_moves: Vec<String> = tokens
                                    .join(" ")
                                    .split_whitespace()
                                    .map(|s| s.to_string())
                                    .collect();

                                if uci_moves.len() > 0 {
                                    self.apply_move(uci_moves);
                                } else {
                                    self.new_game();
                                }
                            }
                            "fen" => {
                                let mut positon = Vec::new();
                                while let Some(position_token) = lexer.next() {
                                    positon.push(position_token);
                                }
                                let fen = positon.concat();
                                println!("fen, {}", fen);
                                self.from(&fen);
                            }
                            _ => {
                                println!("
                                Invalid usage of position command:
                                  position startpos       : Sets the starting position.
                                  position fen [FEN]      : Sets the position using a FEN string.
                                  position startpos moves [moves] : Sets the position from the starting position and the list of moves.
                                ");
                            }
                        }
                    } else {
                        println!("
                        Invalid usage of positon \n 
                            position startpos: Sets the starting position.
                            position fen [FEN]: Sets the position using a FEN string.
                            position startpos moves [moves]: Sets the position from the starting position and the list of moves.
                        ")
                    }
                }
                "go" => {
                    if self.child_handle.is_none() {
                        println!("UciGo");
                        self.go(Some(24));
                    } else {
                        println!("engine has started already!");
                    }
                }
                "ucinewgame" => {
                    self.new_game();
                }
                "stop" => {
                    self.stop_thinking();
                }
                "quit" => {
                    self.quit();
                }
                "d" => {
                    self.print();
                }
                _ => {
                    eprintln!("Unknown command: {:?}", token);
                }
            }
        }
    }
}
