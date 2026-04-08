use super::block::Block;
use thiserror::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Error, Debug)]
pub enum MinerError {
    #[error("Mining fallido después de {0} iteraciones")]
    MaxIterationsExceeded(u64),
    
    #[error("Bloque inválido para minar")]
    InvalidBlock,
    
    #[error("Dificultad inválida: {0}")]
    InvalidDifficulty(u32),
}

pub type MinerResult<T> = Result<T, MinerError>;

pub struct Miner {
    max_iterations: u64,
    enable_progress: bool,
}

impl Miner {
    pub fn new() -> Self {
        Miner {
            max_iterations: 1_000_000,
            enable_progress: false,
        }
    }

    pub fn with_config(max_iterations: u64, enable_progress: bool) -> Self {
        Miner {
            max_iterations,
            enable_progress,
        }
    }

    pub fn mine(&self, block: &mut Block, difficulty: u32) -> MinerResult<bool> {
        if difficulty == 0 {
            return Err(MinerError::InvalidDifficulty(difficulty));
        }
        
        let target = "0".repeat(difficulty as usize);
        
        for nonce in 0..self.max_iterations {
            block.nonce = nonce;
            let hash = block.calculate_hash();
            
            if hash.starts_with(&target) {
                block.set_hash(hash);
                return Ok(true);
            }
            
            if self.enable_progress && nonce % 10000 == 0 {
                println!("Nonce: {}, Hash: {}", nonce, &hash[..8]);
            }
        }
        
        Err(MinerError::MaxIterationsExceeded(self.max_iterations))
    }

    pub fn mine_with_callback<F>(&self, block: &mut Block, difficulty: u32, mut callback: F) -> MinerResult<bool> 
    where
        F: FnMut(u64, &str) -> bool,
    {
        let target = "0".repeat(difficulty as usize);
        
        for nonce in 0..self.max_iterations {
            block.nonce = nonce;
            let hash = block.calculate_hash();
            
            if callback(nonce, &hash) {
                block.set_hash(hash);
                return Ok(true);
            }
            
            if hash.starts_with(&target) {
                block.set_hash(hash);
                return Ok(true);
            }
        }
        
        Err(MinerError::MaxIterationsExceeded(self.max_iterations))
    }

    pub fn verify_proof(&self, block: &Block) -> bool {
        let target = "0".repeat(block.difficulty as usize);
        block.hash.starts_with(&target) && block.hash == block.calculate_hash()
    }

    pub fn calculate_difficulty(block: &Block, previous_block: &Block, target_block_time: u64) -> u32 {
        if block.timestamp <= previous_block.timestamp {
            return previous_block.difficulty;
        }
        
        let time_diff = block.timestamp - previous_block.timestamp;
        let adjustment = time_diff as i64 - target_block_time as i64;
        
        if adjustment < -(target_block_time as i64 / 2) {
            previous_block.difficulty + 1
        } else if adjustment > (target_block_time as i64 / 2) {
            previous_block.difficulty.saturating_sub(1)
        } else {
            previous_block.difficulty
        }
    }

    pub fn estimate_hash_rate(nonce: u64, elapsed_ms: u128) -> f64 {
        if elapsed_ms == 0 {
            return 0.0;
        }
        (nonce as f64 / elapsed_ms as f64) * 1000.0
    }

    pub fn estimate_time_to_find(difficulty: u32, hash_rate: f64) -> f64 {
        if hash_rate == 0.0 {
            return f64::INFINITY;
        }
        
        let target_space = 16f64.powf(difficulty as f64);
        target_space / hash_rate
    }

    pub fn get_max_iterations(&self) -> u64 {
        self.max_iterations
    }

    pub fn set_progress(&mut self, enabled: bool) {
        self.enable_progress = enabled;
    }
}

impl Default for Miner {
    fn default() -> Self {
        Self::new()
    }
}

impl Block {
    pub fn mine(&mut self, difficulty: u32) {
        let miner = Miner::new();
        let _ = miner.mine(self, difficulty);
    }

    pub fn mine_with_timeout(&mut self, difficulty: u32, timeout_ms: u64) -> bool {
        let target = "0".repeat(difficulty as usize);
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let mut nonce = 0u64;
        
        while nonce < 1_000_000 {
            let elapsed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                - start;
            
            if elapsed > timeout_ms as u128 {
                return false;
            }
            
            self.nonce = nonce;
            let hash = self.calculate_hash();
            
            if hash.starts_with(&target) {
                self.set_hash(hash);
                return true;
            }
            
            nonce += 1;
        }
        
        false
    }
}

pub fn create_genesis_block(difficulty: u32) -> Block {
    Block::genesis(difficulty)
}

pub fn validate_block(block: &Block, previous_block: &Block) -> bool {
    block.previous_hash == previous_block.hash 
        && block.index == previous_block.index + 1
        && block.hash == block.calculate_hash()
        && block.verify_difficulty()
}

pub fn validate_block_with_errors(block: &Block, previous_block: &Block) -> Vec<String> {
    let mut errors = Vec::new();
    
    if block.previous_hash != previous_block.hash {
        errors.push("Hash anterior no coincide".to_string());
    }
    
    if block.index != previous_block.index + 1 {
        errors.push("Índice de bloque inválido".to_string());
    }
    
    if block.hash != block.calculate_hash() {
        errors.push("Hash del bloque inválido".to_string());
    }
    
    if !block.verify_difficulty() {
        errors.push("Proof of work inválido".to_string());
    }
    
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miner_creation() {
        let miner = Miner::new();
        assert_eq!(miner.get_max_iterations(), 1_000_000);
    }

    #[test]
    fn test_miner_config() {
        let miner = Miner::with_config(5000, true);
        assert_eq!(miner.get_max_iterations(), 5000);
    }

    #[test]
    fn test_mine_block() {
        let mut block = Block::new(1, "Test".to_string(), "previous".to_string(), 2);
        block.mine(1);
        assert!(!block.hash.is_empty());
    }

    #[test]
    fn test_verify_proof() {
        let mut block = Block::new(1, "Test".to_string(), "prev".to_string(), 1);
        block.mine(1);
        
        let miner = Miner::new();
        assert!(miner.verify_proof(&block));
    }

    #[test]
    fn test_genesis_creation() {
        let genesis = create_genesis_block(2);
        assert_eq!(genesis.index, 0);
    }

    #[test]
    fn test_validate_block() {
        let prev = Block::genesis(1);
        let mut block = Block::new(1, "Test".to_string(), prev.hash.clone(), 1);
        block.mine(1);
        
        assert!(validate_block(&block, &prev));
    }

    #[test]
    fn test_hash_rate_estimation() {
        let rate = Miner::estimate_hash_rate(1000, 100);
        assert!((rate - 10000.0).abs() < 0.01);
    }

    #[test]
    fn test_time_estimation() {
        let time = Miner::estimate_time_to_find(2, 1000.0);
        assert!(time > 0.0);
    }
}