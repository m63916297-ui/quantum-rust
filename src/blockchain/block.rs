use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: u32,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String, difficulty: u32) -> Self {
        Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            difficulty,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let content = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.data,
            self.previous_hash,
            self.nonce
        );
        
        let hash_val = content.bytes().fold(0u64, |acc, b| {
            acc.wrapping_mul(31).wrapping_add(b as u64)
        });
        
        format!("{:016x}", hash_val)
    }

    pub fn is_valid(&self) -> bool {
        let calculated = self.calculate_hash();
        calculated == self.hash
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn get_target(&self) -> String {
        "0".repeat(self.difficulty as usize)
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
        block.set_hash(hash.clone());
        assert_eq!(block.hash, hash);
    }

    #[test]
    fn test_block_validation() {
        let mut block = Block::new(0, "Test".to_string(), "0".to_string(), 1);
        let hash = block.calculate_hash();
        block.set_hash(hash);
        assert!(block.is_valid());
    }
}