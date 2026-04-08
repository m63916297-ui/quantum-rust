use super::block::{Block, BlockBuilder};
use thiserror::Error;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

const MAX_CHAIN_LENGTH: usize = 10000;
const ORPHAN_BUFFER_SIZE: usize = 100;

#[derive(Error, Debug)]
pub enum ChainError {
    #[error("Bloque inválido: {0}")]
    InvalidBlock(String),
    
    #[error("Conexión inválida: {0}")]
    InvalidLink(String),
    
    #[error("Cadena alternativas más corta")]
    ShorterAlternativeChain,
    
    #[error("Límite de capacidad alcanzado")]
    CapacityExceeded,
}

pub type ChainResult<T> = Result<T, ChainError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: u32,
    orphan_blocks: VecDeque<Block>,
    pub metadata: ChainMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChainMetadata {
    pub total_transactions: usize,
    pub total_work: u64,
    pub created_at: u64,
    pub last_updated: u64,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        let genesis = Block::genesis(difficulty);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Blockchain {
            blocks: vec![genesis],
            difficulty,
            orphan_blocks: VecDeque::new(),
            metadata: ChainMetadata {
                total_transactions: 0,
                total_work: 0,
                created_at: now,
                last_updated: now,
            },
        }
    }

    pub fn with_capacity(difficulty: u32, capacity: usize) -> Self {
        let mut bc = Self::new(difficulty);
        bc.blocks.reserve(capacity);
        bc
    }

    pub fn add_block(&mut self, data: String) -> Option<&Block> {
        if self.blocks.len() >= MAX_CHAIN_LENGTH {
            return None;
        }
        
        let previous_hash = self.blocks.last()?.hash.clone();
        let index = self.blocks.len() as u64;
        
        let mut block = BlockBuilder::new()
            .with_index(index)
            .with_data(data.clone())
            .with_previous_hash(previous_hash)
            .with_difficulty(self.difficulty)
            .build();
        
        block.mine(self.difficulty);
        
        self.metadata.total_transactions += 1;
        self.metadata.total_work += block.nonce;
        self.metadata.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.blocks.push(block);
        Some(self.blocks.last().unwrap())
    }

    pub fn add_block_with_builder<F>(&mut self, builder_fn: F) -> Option<&Block>
    where
        F: FnOnce(BlockBuilder) -> BlockBuilder,
    {
        if self.blocks.len() >= MAX_CHAIN_LENGTH {
            return None;
        }
        
        let previous_hash = self.blocks.last()?.hash.clone();
        let index = self.blocks.len() as u64;
        
        let builder = BlockBuilder::new()
            .with_index(index)
            .with_previous_hash(previous_hash)
            .with_difficulty(self.difficulty);
        
        let mut block = builder_fn(builder).build();
        block.mine(self.difficulty);
        
        self.metadata.total_work += block.nonce;
        self.blocks.push(block);
        Some(self.blocks.last().unwrap())
    }

    pub fn add_block_direct(&mut self, mut block: Block) -> Result<&Block, ChainError> {
        let last = self.blocks.last().ok_or(ChainError::InvalidBlock("Cadena vacía".to_string()))?;
        
        if block.previous_hash != last.hash {
            return Err(ChainError::InvalidLink(format!(
                "Hash anterior no coincide: {} != {}",
                block.previous_hash, last.hash
            )));
        }
        
        block.set_hash(block.calculate_hash());
        
        if !block.verify_difficulty() {
            return Err(ChainError::InvalidBlock("Proof of work inválido".to_string()));
        }
        
        self.metadata.total_work += block.nonce;
        self.blocks.push(block);
        Ok(self.blocks.last().unwrap())
    }

    pub fn get_block(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        self.blocks.iter().find(|b| b.hash == hash)
    }

    pub fn verify_integrity(&self) -> bool {
        if self.blocks.is_empty() {
            return false;
        }
        
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];
            
            if current.previous_hash != previous.hash {
                return false;
            }
            
            if current.hash != current.calculate_hash() {
                return false;
            }
            
            if !current.verify_difficulty() {
                return false;
            }
        }
        true
    }

    pub fn verify_integrity_detailed(&self) -> Vec<ChainError> {
        let mut errors = Vec::new();
        
        if self.blocks.is_empty() {
            errors.push(ChainError::InvalidBlock("Cadena vacía".to_string()));
            return errors;
        }
        
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];
            
            if current.previous_hash != previous.hash {
                errors.push(ChainError::InvalidLink(format!(
                    "Bloque {}: hash anterior no coincide",
                    i
                )));
            }
            
            if current.hash != current.calculate_hash() {
                errors.push(ChainError::InvalidBlock(format!("Bloque {}: hash inválido", i)));
            }
            
            if !current.verify_difficulty() {
                errors.push(ChainError::InvalidBlock(format!("Bloque {}: PoW inválido", i)));
            }
        }
        
        errors
    }

    pub fn get_latest_hash(&self) -> String {
        self.blocks.last().map(|b| b.hash.clone()).unwrap_or_default()
    }

    pub fn chain_length(&self) -> usize {
        self.blocks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn replace_chain(&mut self, new_chain: Vec<Block>) -> Result<bool, ChainError> {
        if new_chain.len() <= self.blocks.len() {
            return Err(ChainError::ShorterAlternativeChain);
        }
        
        let mut temp_chain = new_chain;
        
        for i in 1..temp_chain.len() {
            if temp_chain[i].previous_hash != temp_chain[i - 1].hash {
                return Err(ChainError::InvalidLink(format!(
                    "Nueva cadena inválida en bloque {}",
                    i
                )));
            }
            if temp_chain[i].hash != temp_chain[i].calculate_hash() {
                return Err(ChainError::InvalidBlock(format!(
                    "Nueva cadena con hash inválido en bloque {}",
                    i
                )));
            }
        }
        
        self.blocks = temp_chain;
        self.metadata.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(true)
    }

    pub fn calculate_total_work(&self) -> u64 {
        self.metadata.total_work
    }

    pub fn get_work_per_block(&self) -> f64 {
        if self.blocks.is_empty() {
            return 0.0;
        }
        self.metadata.total_work as f64 / self.blocks.len() as f64
    }

    pub fn get_difficulty(&self) -> u32 {
        self.difficulty
    }

    pub fn adjust_difficulty(&mut self, target_time: u64) -> u32 {
        if self.blocks.len() < 2 {
            return self.difficulty;
        }
        
        let last_block = &self.blocks[self.blocks.len() - 1];
        let prev_block = &self.blocks[self.blocks.len() - 2];
        
        let time_diff = last_block.timestamp.saturating_sub(prev_block.timestamp);
        
        if time_diff < target_time / 2 {
            self.difficulty += 1;
        } else if time_diff > target_time * 2 {
            self.difficulty = self.difficulty.saturating_sub(1);
        }
        
        self.difficulty
    }

    pub fn find_block_by_data(&self, data: &str) -> Option<&Block> {
        self.blocks.iter().find(|b| b.data.contains(data))
    }

    pub fn get_blocks_in_range(&self, start: usize, end: usize) -> Vec<&Block> {
        self.blocks[start..end.min(self.blocks.len())].iter().collect()
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let bc = Blockchain::new(2);
        assert_eq!(bc.chain_length(), 1);
        assert!(bc.verify_integrity());
    }

    #[test]
    fn test_add_block() {
        let mut bc = Blockchain::new(1);
        let result = bc.add_block("Test data".to_string());
        assert!(result.is_some());
        assert_eq!(bc.chain_length(), 2);
    }

    #[test]
    fn test_verify_integrity() {
        let mut bc = Blockchain::new(1);
        bc.add_block("Block 1".to_string());
        bc.add_block("Block 2".to_string());
        assert!(bc.verify_integrity());
    }

    #[test]
    fn test_get_work() {
        let mut bc = Blockchain::new(1);
        bc.add_block("test".to_string());
        assert!(bc.calculate_total_work() > 0);
    }

    #[test]
    fn test_difficulty_adjustment() {
        let mut bc = Blockchain::new(2);
        let original = bc.difficulty;
        bc.adjust_difficulty(60);
        assert!(bc.difficulty >= original.saturating_sub(1));
    }
}