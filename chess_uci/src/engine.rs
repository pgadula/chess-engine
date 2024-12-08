use std::thread::{self, JoinHandle, Thread};

use chess_core::{
    bitboard::{FenParser, GameState},
    types::PieceMove,
    utility::{bit_count, pop_bit, pop_lsb},
};

pub struct Engine {
    pub is_running: bool,
    is_searching: bool,
    board: GameState,
    thread: Option<JoinHandle<()>>,
}

impl Engine {
    pub fn new() -> Self {
        return Engine {
            board: GameState::new_game(),
            is_running: true,
            is_searching: false,
            thread: None,
        };
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

    pub fn stop_thinking() {}

    pub fn go(&mut self) {
        let mut game = self.board.clone();
        game.calculate_pseudolegal_moves();
        let valid_moves = game.get_valid_moves();
        let mut result = Vec::new();
        for valid_move in valid_moves {
            let mut cloned_game = game.clone();
            cloned_game.make_move(valid_move);
            let score = self.min_max(0, true, cloned_game, 5);
            result.push((valid_move.uci(), score));
        }
        result.sort_by(|(_, a), (_, b)| a.cmp(b));
        println!("bestmove {}", result[0].0);
    }

    fn min_max(&self, depth: usize, is_max: bool, mut node: GameState, h: usize) -> i32 {
        if depth == h {
            return self.score_board(&node);
        }
        
        node.calculate_pseudolegal_moves();
        let cloned_game = node.clone();

        let valid_moves = cloned_game.get_valid_moves();

        if valid_moves.is_empty() {
            return self.score_board(&node);
        }

        if is_max {
            let mut max_eval = i32::MIN;

            let mut max_game = node.clone();
            for mv in valid_moves {
                max_game.make_move(mv); //
                let eval = self.min_max(depth + 1, false, max_game.clone(), h);
                max_eval = max_eval.max(eval);
                max_game.unmake_move();
            }

            return max_eval;
        } else {
            let mut min_eval = i32::MAX;

            let mut min_game = node.clone();
            for mv in valid_moves {

                min_game.make_move(mv); // Apply the move
                let eval = self.min_max(depth + 1, true, min_game.clone(), h); // Recurse to maximize on the next level
                min_eval = min_eval.min(eval);
                min_game.unmake_move();
            }

            return min_eval;
        }
    }

    fn score_board(&self, game: &GameState) -> i32 {
        let scoring_board: [usize; 6] = [1, 3, 3, 5, 9, 6];
        let mut boards = game.bitboard;
        let mut black_sum: i32 = 0;
        let mut white_sum: i32 = 0;
        let mut i = 0;
        for board in &mut boards {
            let num_bits = bit_count(*board);
            if i < 6 {
                black_sum = black_sum + (num_bits * scoring_board[i]) as i32
            } else {
                white_sum = white_sum + (num_bits * scoring_board[i % 6]) as i32
            }
            i = i + 1;
        }
        return white_sum - black_sum;
    }

    pub fn apply_move(&mut self, uci_moves: Vec<String>) {
        self.new_game();
        for uci in uci_moves {
            let piece_move = PieceMove::from_uci(&uci, &self.board);
            self.board.make_move(&piece_move);
        }
    }
}
