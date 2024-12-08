use std::i32;

use chess_core::{bitboard::GameState, utility::bit_count};

pub struct SearchEngine {
    pub max_depth: u8,
    pub board: GameState,
}
struct SearchInfo {
    depth: u8,
}

impl SearchEngine {
    pub fn search(&self) -> String {
        let mut game = self.board.clone();
        game.calculate_pseudolegal_moves();
        let valid_moves = game.get_valid_moves();
        let mut result = Vec::new();
        let mut alpha = i32::MIN;
        let mut beta = i32::MAX;

        for valid_move in &valid_moves {
            let cloned_game = &mut game;
            cloned_game.make_move(&valid_move);

            let score = self.min_max(&mut alpha, &mut beta, 0, true, cloned_game);
            cloned_game.unmake_move();
            result.push((valid_move.uci(), score));
        }
        if result.is_empty(){
            return "none".to_owned();
        }
        result.sort_by(|(_, a), (_, b)| b.cmp(a));
        return result[0].0.clone();
    }

    fn min_max(
        &self,
        alpha: &mut i32,
        beta: &mut i32,
        depth: u8,
        is_max: bool,
        node: &mut GameState,
    ) -> i32 {
        if depth == self.max_depth {
            return self.score_board(&node);
        }

        node.calculate_pseudolegal_moves();
        let valid_moves = node.get_valid_moves();

        if valid_moves.is_empty() {
            return self.score_board(&node);
        }
        match is_max {
            true => {
                let mut max_eval = i32::MIN;
                for mv in valid_moves {
                    node.make_move(&mv);
                    let eval = self.min_max(alpha, beta, depth + 1, !is_max, node);
                    node.unmake_move();
                    max_eval = i32::max(max_eval, eval);
                    *alpha = i32::max(*alpha, eval);
                    if beta <= alpha {
                        break;
                    }
                }
                return max_eval;
            }
            false => {
                let mut min_eval = i32::MAX;
                for mv in valid_moves {
                    node.make_move(&mv);
                    let eval = self.min_max(alpha, beta, depth + 1, !is_max, node);
                    node.unmake_move();
                    min_eval = i32::min(min_eval, eval);
                    *beta = i32::min(*beta, eval);
                    if beta <= alpha {
                        break;
                    }
                }
                return min_eval;
            }
        }
    }

    fn score_board(&self, game: &GameState) -> i32 {
        let scoring_board: [usize; 6] = [1, 3, 3, 5, 9, 0];
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
        return black_sum - white_sum ;
    }
}
