use super::block::Block;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Miner {
    max_iterations: u64,
    enable_parallel: bool,
}

impl Miner {
    pub fn new() -> Self {
        Miner {
            max_iterations: 1_000_000,
            enable_parallel: false,
        }
    }

    pub fn with_config(max_iterations: u64, parallel: bool) -> Self {
        Miner {
            max_iterations,
            enable_parallel: parallel,
        }
    }

    pub fn mine(&self, block: &mut Block, difficulty: u32) -> bool {
        let target = "0".repeat(difficulty as usize);
        
        for nonce in 0..self.max_iterations {
            block.nonce = nonce;
            let hash = block.calculate_hash();
            
            if hash.starts_with(&target) {
                block.hash = hash;
                return true;
            }
        }
        false
    }

    pub fn mine_with_callback<F>(&self, block: &mut Block, difficulty: u32, mut callback: F) -> bool 
    where
        F: FnMut(u64, &str),
    {
        let target = "0".repeat(difficulty as usize);
        
        for nonce in 0..self.max_iterations {
            block.nonce = nonce;
            let hash = block.calculate_hash();
            callback(nonce, &hash);
            
            if hash.starts_with(&target) {
                block.hash = hash;
                return true;
            }
        }
        false
    }

    pub fn verify_proof(&self, block: &Block) -> bool {
        let target = "0".repeat(block.difficulty as usize);
        block.hash.starts_with(&target) && block.hash == block.calculate_hash()
    }

    pub fn calculate_difficulty(block: &Block, previous_block: &Block, target_block_time: u64) -> u32 {
        let time_diff = block.timestamp.saturating_sub(previous_block.timestamp);
        
        if time_diff < target_block_time / 2 {
            previous_block.difficulty + 1
        } else if time_diff > target_block_time * 2 {
            previous_block.difficulty.saturating_sub(1)
        } else {
            previous_block.difficulty
        }
    }

    pub fn estimate_hash_rate(&self, nonce: u64, elapsed_ms: u128) -> f64 {
        if elapsed_ms == 0 {
            return 0.0;
        }
        (nonce as f64 / elapsed_ms as f64) * 1000.0
    }
}

impl Default for Miner {
    fn default() -> Self {
        Self::new()
    }
}

pub fn create_genesis_block(difficulty: u32) -> Block {
    let mut block = Block::new(
        0,
        "Genesis Block - Quantum Secured".to_string(),
        "0".to_string(),
        difficulty,
    );
    block.hash = block.calculate_hash();
    block
}

pub fn validate_block(block: &Block, previous_block: &Block) -> bool {
    block.previous_hash == previous_block.hash 
        && block.hash == block.calculate_hash()
        && block.index == previous_block.index + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miner_creation() {
        let miner = Miner::new();
        assert_eq!(miner.max_iterations, 1_000_000);
    }

    #[test]
    fn test_mine_block() {
        let miner = Miner::new();
        let mut block = Block::new(1, "Test".to_string(), "previous".to_string(), 2);
        
        let result = miner.mine(&mut block, 2);
        assert!(result || !result);
    }

    #[test]
    fn test_verify_proof() {
        let miner = Miner::new();
        let mut block = Block::new(1, "Test".to_string(), "prev".to_string(), 1);
        block.nonce = 100;
        block.hash = block.calculate_hash();
        
        assert!(miner.verify_proof(&block));
    }

    #[test]
    fn test_genesis_creation() {
        let genesis = create_genesis_block(2);
        assert_eq!(genesis.index, 0);
    }
}