use std::{result, thread};

use chess_core::{
    bitboard::{GameState, TEMP_VALID_MOVE_SIZE},
    types::{Color, PieceMove},
};
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    slice::ParallelSlice,
};

use crate::transposition_table::TranspositionTable;

pub struct SearchEngine {
    pub max_depth: u8,
    pub transposition_table: TranspositionTable,
    pub num_threads: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct SearchResult {
    pub depth: u8,
    pub hash: u64,
    pub score: i32,
    pub is_max: bool,
    pub node_type: NodeType,
}
impl PartialEq for SearchResult {
    fn eq(&self, other: &SearchResult) -> bool {
        std::ptr::eq(self, other)
    }
}

pub const EMPTY_SEARCH_RESULT: SearchResult = SearchResult {
    depth: u8::MAX,
    hash: u64::MAX,
    score: i32::MIN,
    is_max: false,
    node_type: NodeType::Exact,
};


#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
}

impl SearchEngine {
    pub fn search(&mut self, game_state: &GameState) -> String {
        let mut game = game_state.clone();
        game.calculate_pseudolegal_moves();
        let mut valid_moves = [PieceMove::default(); TEMP_VALID_MOVE_SIZE];
        let count = game.fill_valid_moves(&mut valid_moves);
        let mut result = Vec::new();

        for valid_move in &valid_moves[..count] {
            let cloned_game = &mut game;
            cloned_game.make_move(&valid_move);
            let mut buffer = [PieceMove::default(); TEMP_VALID_MOVE_SIZE];
            let score = self.min_max(i32::MIN, i32::MAX, 0, false, cloned_game, &mut buffer);
            cloned_game.unmake_move();
            result.push((valid_move.uci(), score));
        }
        if result.is_empty() {
            return "none".to_owned();
        }
        result.sort_by(|(_, a), (_, b)| b.cmp(a));
        println!("{:?}", result);
        println!();
        println!(
            "Warning: Number of hash collision: {}",
            self.transposition_table.collision_detected
        );

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
        mut buffer: &mut [PieceMove; TEMP_VALID_MOVE_SIZE],
    ) -> i32 {
        let original_alpha = alpha;
        let original_beta = beta;

        if let Some(score) = self
            .transposition_table
            .try_get_from_cache(node.hash, depth, alpha, beta, is_max)
        {
            return score;
        }

        let mut alpha = alpha;
        let mut beta = beta;

        node.calculate_pseudolegal_moves();
        let count = node.fill_valid_moves(&mut buffer);

        if depth == self.max_depth || count == 0 {
            let score = self.score_heuristic(&node);
            self.transposition_table.store_in_cache(
                node.hash,
                depth,
                score,
                is_max,
                NodeType::Exact,
            );
            return score;
        }

        let mut best_value;
        let new_buffer = &mut buffer.clone();

        if is_max {
            best_value = i32::MIN;
            for mv in &mut buffer[..count] {
                node.make_move(&mv);
                let eval = self.min_max(alpha, beta, depth + 1, false, node, new_buffer);
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
            for mv in &buffer[..count] {
                node.make_move(&mv);
                let eval = self.min_max(alpha, beta, depth + 1, true, node, new_buffer);
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

        self.transposition_table
            .store_in_cache(node.hash, depth, best_value, is_max, node_type);

        best_value
    }

    pub fn multi_threads_search(&mut self, game_state: &GameState) -> String {
        let mut game = game_state.clone();
        game.calculate_pseudolegal_moves();

        let mut valid_moves = [PieceMove::default(); TEMP_VALID_MOVE_SIZE];
        let count = game.fill_valid_moves(&mut valid_moves);
        println!("INFO: Number of valid moves {}", count);
        // is_max means "maximizing" (commonly for the side to move)
        // If White's turn, then is_max = false, else true
        let is_max = game.move_turn == Color::Black;

        // We'll split the valid moves into chunks:
        let chunks = &valid_moves[..count].chunks(count / self.num_threads as usize);
        // We'll collect final results here
        let mut all_results: Vec<(String, i32)> = Vec::with_capacity(count);

        // Single scope so that all threads are joined by the time we exit
        thread::scope(|scope| {
            // We'll keep the join handles of each thread here
            let mut handles = Vec::new();

            for chunk in chunks.clone().into_iter() {
                let cloned_game = game.clone();
                let max_depth = self.max_depth;

                // Spawn a child thread for each chunk
                let handle = scope.spawn(move || {
                    println!("INFO: Thread is starting with chunk size {}", chunk.len());

                    let mut thread_calculation = Vec::new();
                    let mut new_search = SearchEngine::new();
                    new_search.max_depth = max_depth;

                    for valid_move in chunk {
                        let mut current_game_for_thread = cloned_game.clone();
                        current_game_for_thread.make_move(valid_move);

                        let mut buffer = [PieceMove::default(); TEMP_VALID_MOVE_SIZE];
                        let score = new_search.min_max(
                            i32::MIN,
                            i32::MAX,
                            0,
                            is_max,
                            &mut current_game_for_thread,
                            &mut buffer,
                        );

                        // Undo the move
                        current_game_for_thread.unmake_move();

                        // Store (move_uci, score)
                        thread_calculation.push((valid_move.uci(), score));
                    }
                    println!("WARNING: hash collision {}", new_search.transposition_table.collision_detected);

                    // Return the per-chunk results from this thread
                    thread_calculation
                });
                // Collect the handle so we can join later
                handles.push(handle);
            }

            // Now that all threads have been spawned, join them to collect results
            for handle in handles {
                let partial = handle.join().expect("Child thread panicked!");
                all_results.extend(partial);
            }
        });

        // Now all threads are joined, and `all_results` has the moves & scores
        // Sort in descending order of score
        all_results.sort_by(|(_, a), (_, b)| b.cmp(a));

        // For debugging/logging
        println!("{:?}", all_results);
        println!();
        // The best move is presumably the first in the sorted list
        println!("score: {}", all_results[0].1);

        // Return the best move's UCI string
        all_results[0].0.clone()
    }

    pub fn rayon_search(&mut self, game_state: &GameState) -> String {
        let mut game = game_state.clone();
        game.calculate_pseudolegal_moves();
        let mut valid_moves = [PieceMove::default(); TEMP_VALID_MOVE_SIZE];
        let count = game.fill_valid_moves(&mut valid_moves);
        let is_max = if Color::White == game.move_turn {
            false
        } else {
            true
        };
        let slice_of_valid_moves = &valid_moves[..count];
        let results = &slice_of_valid_moves
            .par_chunks(slice_of_valid_moves.len() / 4)
            .into_par_iter()
            .map(|valid_moves| -> Vec<(String, i32)> {
                println!("INFO: Thread is starting with chunk size {}", valid_moves.len());
                let mut results = Vec::with_capacity(valid_moves.len());
                let cloned_game = game.clone();
                let max_depth = self.max_depth;
                let mut new_search: SearchEngine = SearchEngine::new();
                new_search.max_depth = max_depth;

                let mut current_game_for_thread = cloned_game.clone();
                for valid_move in valid_moves {
                    current_game_for_thread.make_move(&valid_move);
                    let mut buffer = [PieceMove::default(); TEMP_VALID_MOVE_SIZE];
                    let score = new_search.min_max(
                        i32::MIN,
                        i32::MAX,
                        0,
                        is_max,
                        &mut current_game_for_thread,
                        &mut buffer,
                    );
                    current_game_for_thread.unmake_move();
                    results.push((valid_move.uci(), score));
                }
                println!(
                    "Warning: Number of hash collision: {}",
                    new_search.transposition_table.collision_detected
                );
                return results;
            });

        let mut result: Vec<(String, i32)> = results.clone().flatten().collect();
        result.sort_by(|(_, a), (_, b)| b.cmp(a));
        println!("{:?}", result);
        println!();

        println!("score: {}", result[0].1);
        return result[0].0.clone();
    }

    fn score_heuristic(&self, game: &GameState) -> i32 {
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

    pub fn new() -> SearchEngine {
        return SearchEngine {
            num_threads: 4,
            max_depth: 6,
            transposition_table: TranspositionTable::new(),
        };
    }
}
