use std::{
    result,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use chess_core::{
    bitboard::{GameState, TEMP_VALID_MOVE_SIZE},
    types::{Color, PieceMove},
};
use rayon::{
    iter::ParallelIterator,
    slice::ParallelSlice,
};

use crate::transposition_table::TranspositionTable;

#[derive(Debug, Clone)]
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
            let score = self.min_max(
                i32::MIN,
                i32::MAX,
                0,
                false,
                cloned_game,
                &mut buffer,
                Arc::new(AtomicBool::new(false)),
            );
            cloned_game.unmake_move();
            result.push((valid_move.uci(), score));
        }
        if result.is_empty() {
            return "none".to_owned();
        }
        result.sort_by(|(_, a), (_, b)| b.cmp(a));
        println!("{:?}", result);
        println!();
        // println!(
        //     "Warning: Number of hash collision: {}",
        //     self.transposition_table.collision_detected
        // );

        println!("score: {}", result[0].1);
        return result[0].0.clone();
    }

    #[inline(always)]
    fn min_max(
        &mut self,
        alpha: i32,
        beta: i32,
        depth: u8,
        is_max: bool,
        node: &mut GameState,
        mut buffer: &mut [PieceMove; TEMP_VALID_MOVE_SIZE],
        stop_signal: Arc<AtomicBool>,
    ) -> i32 {
        if stop_signal.load(Ordering::SeqCst) {
            return 0;
        }
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
                let eval = self.min_max(
                    alpha,
                    beta,
                    depth + 1,
                    false,
                    node,
                    new_buffer,
                    stop_signal.clone(),
                );
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
                let eval = self.min_max(
                    alpha,
                    beta,
                    depth + 1,
                    true,
                    node,
                    new_buffer,
                    stop_signal.clone(),
                );
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

    pub fn rayon_search(
        &mut self,
        game_state: &GameState,
        stop_signal: Arc<AtomicBool>,
    ) -> String {
        let mut game = game_state.clone();
        game.calculate_pseudolegal_moves();
        let mut valid_moves = [PieceMove::default(); TEMP_VALID_MOVE_SIZE];
        let count = game.fill_valid_moves(&mut valid_moves);
        let is_max = matches!(game.move_turn, Color::Black);
        let slice_of_valid_moves = &valid_moves[..count];
        let max_depth = self.max_depth;

        // Determine the number of chunks; ensure it's at least 1 to avoid division by zero
        let num_chunks = 4.min(count.max(1));
        let chunk_size = (count + num_chunks - 1) / num_chunks; // Ceiling division

        // Parallel processing with Rayon
        let results = slice_of_valid_moves
            .par_chunks(chunk_size)
            .map(|valid_moves_chunk| -> (Vec<(String, i32, u8)>, u8) {
                let mut thread_results: Vec<(String, i32, u8)> = Vec::new();
                let mut last_completed_depth = 0;
                let thread_id = thread::current().id();

                for depth in 1..=max_depth {
                    // println!(
                    //     "INFO: Depth:{} {:?} processing chunk of size {}",
                    //     depth,
                    //     thread_id,
                    //     valid_moves_chunk.len()
                    // );
    

                    // Check for stop signal before starting a new depth
                    if stop_signal.load(Ordering::SeqCst) {
                        println!("INFO: Stop signal received before starting depth {depth}");
                        break;
                    }

                    let cloned_game = game.clone();
                    let mut new_search = SearchEngine::new();
                    new_search.max_depth = depth;

                    let mut current_game = cloned_game.clone();
                    let mut depth_results: Vec<(String, i32, u8)> = Vec::new();

                    for valid_move in valid_moves_chunk {
                        current_game.make_move(valid_move);

                        let mut buffer = [PieceMove::default(); TEMP_VALID_MOVE_SIZE];
                        let score = new_search.min_max(
                            i32::MIN,
                            i32::MAX,
                            0,
                            is_max,
                            &mut current_game,
                            &mut buffer,
                            stop_signal.clone(),
                        );

                        current_game.unmake_move();

                        // Check for stop signal after each move
                        if stop_signal.load(Ordering::SeqCst) {
                            println!("INFO: Stop signal received during processing at depth {depth}");
                            // Discard partial results for this depth
                            return (thread_results, last_completed_depth);
                        }

                        depth_results.push((valid_move.uci(), score, depth));
                    }

                    // If all moves at this depth are processed without interruption
                    thread_results.extend(depth_results);
                    last_completed_depth = depth;
                }

                (thread_results, last_completed_depth)
            })
            .collect::<Vec<(Vec<(String, i32, u8)>, u8)>>();

        // Determine the minimum completed depth across all threads
        let max_completed_depth = results
            .iter()
            .map(|(_, depth)| *depth)
            .min()
            .unwrap_or(0);

        println!("INFO: Maximum fully completed depth across all threads: {max_completed_depth}");

        // Collect all results at the max_completed_depth
        let mut final_results: Vec<(String, i32, u8)> = results
            .iter()
            .flat_map(|(moves, _)| moves.iter())
            .filter(|(_, _, depth)| *depth == max_completed_depth)
            .cloned()
            .collect();

        // Handle case where no results are available
        if final_results.is_empty() {
            println!("WARN: No results available at depth {max_completed_depth}");
            return "no_moves".to_string();
        }

        // Sort the results based on the score
        if is_max {
            // For maximizing player (Black), higher scores are better
            // Sort in descending order
            final_results.sort_by(|a, b| b.1.cmp(&a.1));
        } else {
            // For minimizing player (White), lower scores are better
            // Sort in ascending order
            final_results.sort_by(|a, b| a.1.cmp(&b.1));
        }

        println!("{:?}", final_results);
        // Select the best move
        if let Some((best_uci, best_score, _)) = final_results.first() {
            println!("INFO: Best move at depth {max_completed_depth}: {best_uci} with score {best_score}");
            best_uci.clone()
        } else {
            // Fallback if no moves are found
            "no_moves".to_string()
        }
    }

    #[inline(always)]
    fn score_heuristic(&self, game: &GameState) -> i32 {
        const PIECE_VALUES: [i32; 6] = [100, 320, 330, 500, 900, 0];
        let mut white_sum = 0;
        let mut black_sum = 0;
        for (i, &board) in game.bitboard.iter().enumerate() {
            let num_bits = board.count_ones();
            let score = num_bits * PIECE_VALUES[i % 6] as u32;
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
