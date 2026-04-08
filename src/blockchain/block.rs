use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum BlockchainError {
    #[error("Índice de bloque inválido: {0}")]
    InvalidBlockIndex(usize),
    
    #[error("Hash inválido: {0}")]
    InvalidHash(String),
    
    #[error("Bloque no encontrado: {0}")]
    BlockNotFound(String),
    
    #[error("Cadena inválida: {0}")]
    InvalidChain(String),
    
    #[error("Error de minería: {0}")]
    MiningError(String),
    
    #[error("Límite de bloques alcanzado: {0}")]
    MaxBlocksReached(usize),
}

impl std::fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub type BlockchainResult<T> = Result<T, BlockchainError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: u32,
    pub merkle_root: String,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String, difficulty: u32) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let merkle_root = Self::calculate_merkle_root(&data);
        
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            difficulty,
            merkle_root,
        }
    }

    pub fn genesis(difficulty: u32) -> Self {
        let mut block = Block::new(
            0,
            "Genesis Block - Quantum Secured Blockchain".to_string(),
            "0".to_string(),
            difficulty,
        );
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let content = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.data,
            self.previous_hash,
            self.nonce,
            self.merkle_root
        );
        
        let hash_val = Self::hash_string(&content);
        format!("{:016x}", hash_val)
    }

    fn hash_string(s: &str) -> u64 {
        s.bytes().fold(0u64, |acc, b| {
            acc.wrapping_mul(31).wrapping_add(b as u64)
        })
    }

    fn calculate_merkle_root(data: &str) -> String {
        let hash = Self::hash_string(data);
        format!("{:016x}", hash)
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn is_valid(&self) -> bool {
        let calculated = self.calculate_hash();
        calculated == self.hash && self.verify_difficulty()
    }

    pub fn verify_difficulty(&self) -> bool {
        let target = "0".repeat(self.difficulty as usize);
        self.hash.starts_with(&target)
    }

    pub fn get_target(&self) -> String {
        "0".repeat(self.difficulty as usize)
    }

    pub fn size_bytes(&self) -> usize {
        std::mem::size_of_val(self) + self.data.len() + self.hash.len()
    }
}

pub struct BlockBuilder {
    index: u64,
    data: String,
    previous_hash: String,
    difficulty: u32,
}

impl BlockBuilder {
    pub fn new() -> Self {
        BlockBuilder {
            index: 0,
            data: String::new(),
            previous_hash: String::new(),
            difficulty: 2,
        }
    }

    pub fn with_index(mut self, index: u64) -> Self {
        self.index = index;
        self
    }

    pub fn with_data(mut self, data: String) -> Self {
        self.data = data;
        self
    }

    pub fn with_previous_hash(mut self, hash: String) -> Self {
        self.previous_hash = hash;
        self
    }

    pub fn with_difficulty(mut self, difficulty: u32) -> Self {
        self.difficulty = difficulty;
        self
    }

    pub fn build(self) -> Block {
        Block::new(self.index, self.data, self.previous_hash, self.difficulty)
    }
}

impl Default for BlockBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(0, "Genesis".to_string(), "0".to_string(), 2);
        assert_eq!(block.index, 0);
        assert_eq!(block.previous_hash, "0");
    }

    #[test]
    fn test_block_hash() {
        let mut block = Block::new(0, "Test".to_string(), "0".to_string(), 1);
        let hash = block.calculate_hash();
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_block_validation() {
        let mut block = Block::new(0, "Test".to_string(), "0".to_string(), 1);
        let hash = block.calculate_hash();
        block.set_hash(hash);
        assert!(block.is_valid());
    }

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis(2);
        assert_eq!(genesis.index, 0);
        assert_eq!(genesis.previous_hash, "0");
    }

    #[test]
    fn test_block_builder() {
        let block = BlockBuilder::new()
            .with_index(1)
            .with_data("Test data".to_string())
            .with_difficulty(3)
            .build();
        
        assert_eq!(block.index, 1);
        assert_eq!(block.difficulty, 3);
    }
}