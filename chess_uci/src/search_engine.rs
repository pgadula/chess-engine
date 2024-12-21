use std::{collections::HashMap, i32};

use chess_core::bitboard::GameState;

pub struct SearchEngine {
    pub max_depth: u8,
    pub cache: HashMap<u64, SearchResult>,
}

#[derive(Debug)]
pub struct SearchResult {
    depth: u8,
    hash: u64,
    score: i32,
    is_max: bool,
    node_type: NodeType,
}

#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
}

impl SearchEngine {
    pub fn search(&mut self, board: &GameState) -> String {
        let mut game = board.clone();
        game.calculate_pseudolegal_moves();
        let valid_moves = game.get_valid_moves();
        let mut result = Vec::new();

        for valid_move in &valid_moves {
            let cloned_game = &mut game;
            cloned_game.make_move(&valid_move);

            let score = self.min_max(i32::MIN, i32::MAX, 0, false, cloned_game);
            cloned_game.unmake_move();
            result.push((valid_move.uci(), score));
        }
        if result.is_empty() {
            return "none".to_owned();
        }
        result.sort_by(|(_, a), (_, b)| b.cmp(a));
        println!("{:?}", result);
        println!();
        println!("score: {}", result[0].1);
        return result[0].0.clone();
    }

    fn min_max(
        &mut self,
        alpha: i32,
        beta: i32,
        depth: u8,
        is_max: bool,
        node: &mut GameState,
    ) -> i32 {
        let original_alpha = alpha;
        let original_beta = beta;
    
        if let Some(score) = self.try_get_from_cache(node.hash, depth, alpha, beta, is_max) {
            return score;
        }

        let mut alpha = alpha;
        let mut beta = beta;
    
        node.calculate_pseudolegal_moves();
        let valid_moves = node.get_valid_moves();
    
        if depth == self.max_depth || valid_moves.is_empty() {
            let score = self.score_board(&node);
            let search_result = SearchResult {
                depth,
                hash: node.hash,
                score,
                is_max,
                node_type: NodeType::Exact,
            };
            self.cache.insert(node.hash, search_result);
            return score;
        }
    
        let mut best_value;
        if is_max {
            best_value = i32::MIN;
            for mv in valid_moves {
                node.make_move(&mv);
                let eval = self.min_max(alpha, beta, depth + 1, false, node);
                node.unmake_move();
    
                if eval > best_value {
                    best_value = eval;
                }
                if best_value > alpha {
                    alpha = best_value;
                }
                if beta <= alpha {
                    break;
                }
            }
        } else {
            best_value = i32::MAX;
            for mv in valid_moves {
                node.make_move(&mv);
                let eval = self.min_max(alpha, beta, depth + 1, true, node);
                node.unmake_move();
    
                if eval < best_value {
                    best_value = eval;
                }
                if best_value < beta {
                    beta = best_value;
                }
                if beta <= alpha {
                    break;
                }
            }
        }
    
        let node_type = if best_value <= original_alpha {
            NodeType::UpperBound
        } else if best_value >= original_beta {
            NodeType::LowerBound
        } else {
            NodeType::Exact
        };
    
        self.store_in_cache(node.hash, depth, best_value, is_max, node_type);
    
        best_value
    }

    fn try_get_from_cache(
        &self,
        hash: u64,
        current_depth: u8,
        alpha: i32,
        beta: i32,
        is_max: bool,
    ) -> Option<i32> {
        if let Some(result) = self.cache.get(&hash) {
            if result.depth >= current_depth && result.is_max == is_max {
                match result.node_type {
                    NodeType::Exact => {
                        return Some(result.score);
                    }
                    NodeType::LowerBound => {
                        if result.score >= beta {
                            return Some(result.score);
                        }
                    }
                    NodeType::UpperBound => {
                        if result.score <= alpha {
                            return Some(result.score);
                        }
                    }
                }
            }
        }
        None
    }

    fn score_board(&self, game: &GameState) -> i32 {
        let scoring_board: [usize; 6] = [1, 3, 3, 5, 9, 0];
        let mut white_sum = 0;
        let mut black_sum = 0;
        for (i, &board) in game.bitboard.iter().enumerate() {
            let num_bits = board.count_ones();
            let score = num_bits * scoring_board[i % 6] as u32;
            if i < 6 {
                white_sum += score as i32;
            } else {
                black_sum += score as i32;
            }
        }
        white_sum - black_sum
    }

    fn store_in_cache(
        &mut self,
        hash: u64,
        depth: u8,
        score: i32,
        is_max: bool,
        node_type: NodeType,
    ) {
        let search_result = SearchResult {
            depth,
            hash,
            score,
            is_max,
            node_type,
        };
        self.cache.insert(hash, search_result);
    }

    pub fn new(max_depth: u8)->SearchEngine{
        SearchEngine{
            max_depth,
            cache: HashMap::new()
        }
    }

    pub fn clear_lookup_table(& mut self){
        self.cache.clear();
    }
}
