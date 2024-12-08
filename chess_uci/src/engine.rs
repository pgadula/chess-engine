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
            for valid_move in valid_moves  {
                let mut cloned_game = game.clone();
                cloned_game.make_move(valid_move);
                let score =  self.min_max(0, true, cloned_game, 0);
                result.push((valid_move.uci(), score));
            }
            result.sort_by(|(_, a), (_, b)| a.cmp(b));
            println!("bestmove {}", result[0].0);
            // self.thread = Some(thread::spawn(move || {
            //     cloned_game.calculate_pseudolegal_moves();
            //     cloned_game.get_valid_moves();
            // }));
            
    }

    fn min_max(&self ,depth: usize, is_max:bool, mut node: GameState, h: usize)->usize {
        if depth == h{
            return self.score_board(&node)
        }
        node.calculate_pseudolegal_moves();
        let valid_moves = node.get_valid_moves();
        0
    }

    fn score_board(&self ,game: &GameState)->usize {
        let scoring_board: [usize; 6] = [1, 3, 3, 5, 9, 6];
        let mut boards = game.bitboard;
        let mut black_sum: usize = 0;
        let mut white_sum: usize = 0;
        let mut i = 0;
        for board in &mut boards {
            let num_bits = bit_count(*board);
            if i < 6 {
                black_sum = black_sum + num_bits * scoring_board[i]
            } else {
                white_sum = white_sum + num_bits * scoring_board[i % 6]
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
