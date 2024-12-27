use crate::search_engine::{NodeType, SearchResult, EMPTY_SEARCH_RESULT};

pub const LOOKUP_SIZE: usize = 1024 * 1024;

pub const BUCKET_SIZE: usize = 1;

pub struct TranspositionTable {
    pub lookup_table: Vec<SearchResult>,
    pub collision_detected: usize,
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        TranspositionTable {
            lookup_table: vec![EMPTY_SEARCH_RESULT; LOOKUP_SIZE * BUCKET_SIZE],
            collision_detected: 0,
        }
    }

    pub fn clear_lookup_table(&mut self) {
        self.lookup_table = vec![EMPTY_SEARCH_RESULT; LOOKUP_SIZE * BUCKET_SIZE]
    }

    pub fn store_in_cache(
        &mut self,
        hash: u64,
        depth: u8,
        score: i32,
        is_max: bool,
        node_type: NodeType,
    ) -> Result<usize, &'static str> {
        let search_result = SearchResult {
            depth,
            hash,
            score,
            is_max,
            node_type,
        };
        let index = self.get_index(hash);

        let mut i = 0;
        while i < BUCKET_SIZE {
            let entity = self.lookup_table[index + i];
            if entity == EMPTY_SEARCH_RESULT {
                self.lookup_table[index + i] = search_result;
                return Ok(index + i);
            }
            i = i + 1;
        }
        self.collision_detected = self.collision_detected + 1;
        return Err("Error: bucket size reached, no empty place for record");
    }

    pub fn try_get_from_cache(
        &self,
        hash: u64,
        current_depth: u8,
        alpha: i32,
        beta: i32,
        is_max: bool,
    ) -> Option<i32> {
        let index = self.get_index(hash);
        let mut entity: Option<SearchResult> = None;
        let mut i = 0;
        while i < BUCKET_SIZE {
            let result = self.lookup_table[index + i];
            i = i + 1;
            if result.hash == hash {
                entity = Some(result);
                break;
            }
        }
        match entity {
            Some(value) => {
                if value.depth >= current_depth && value.is_max == is_max {
                    match value.node_type {
                        NodeType::Exact => {
                            return Some(value.score);
                        }
                        NodeType::LowerBound => {
                            if value.score >= beta {
                                return Some(value.score);
                            }
                        }
                        NodeType::UpperBound => {
                            if value.score <= alpha {
                                return Some(value.score);
                            }
                        }
                    }
                }
            }
            None => return None,
        }
        None
    }

    pub fn get_index(&self, hash: u64) -> usize {
        return (hash as usize % LOOKUP_SIZE) * BUCKET_SIZE;
    }
}
