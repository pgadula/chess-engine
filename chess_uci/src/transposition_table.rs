use crate::search_engine::{NodeType, SearchResult, EMPTY_SEARCH_RESULT};

pub const LOOKUP_SIZE: usize = 1 << 19; // 524,288
pub const BUCKET_SIZE: usize = 16;

#[derive(Debug, Clone)]
pub struct TranspositionTable {
    pub lookup_table: Vec<SearchResult>,
    pub collision_detected: usize,
    pub collision_strategy: CollisionStrategy
}

#[derive(Debug, Clone)]
pub enum CollisionStrategy {
    ReplaceWithRandHash,
    ReplaceWithShallowDepth,
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        TranspositionTable {
            lookup_table: vec![EMPTY_SEARCH_RESULT; LOOKUP_SIZE * BUCKET_SIZE],
            collision_detected: 0,
            collision_strategy: CollisionStrategy::ReplaceWithShallowDepth
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
            let idx = index + i;
            let entity = self.lookup_table[idx];

            if entity.hash == hash{
                if entity.depth < search_result.depth{
                    self.lookup_table[idx] = search_result;
                    return Ok(idx)
                }
            }
            if entity.hash == EMPTY_SEARCH_RESULT.hash {
                self.lookup_table[idx] = search_result;
                return Ok(idx);
            }
            self.collision_detected = self.collision_detected + 1;

            i = i + 1;
        }


        match self.collision_strategy {
            CollisionStrategy::ReplaceWithRandHash => self.replace_wth_random_index(search_result, index),
            CollisionStrategy::ReplaceWithShallowDepth =>  self.replace_with_shallow_depth(search_result, index),
        }
       
        return Err("Error: bucket size reached, no empty place for record");
    }

    fn replace_wth_random_index(&mut self, search_result: SearchResult, index: usize) {
        let r_idx = (search_result.hash as usize) % BUCKET_SIZE;
        self.lookup_table[index + r_idx] = search_result;
    }


    fn replace_with_shallow_depth(&mut self, search_result: SearchResult, index: usize){
        let mut i = 0;
        while i < BUCKET_SIZE {
            let idx = index + i;
            let entity = self.lookup_table[idx];
            if entity.depth < search_result.depth{
                self.lookup_table[idx] =  search_result;
                break;
            }
            i = i + 1;
        }
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
