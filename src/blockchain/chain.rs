use super::block::Block;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

const MAX_CHAIN_LENGTH: usize = 10000;
const ORPHAN_BUFFER_SIZE: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: u32,
    pub orphan_blocks: VecDeque<Block>,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        let genesis = Block::new(0, "Genesis Block".to_string(), "0".to_string(), difficulty);
        let mut bc = Blockchain {
            blocks: vec![genesis],
            difficulty,
            orphan_blocks: VecDeque::new(),
        };
        bc.blocks[0].hash = bc.blocks[0].calculate_hash();
        bc
    }

    pub fn add_block(&mut self, data: String) -> Option<&Block> {
        if self.blocks.len() >= MAX_CHAIN_LENGTH {
            return None;
        }
        
        let previous_hash = self.blocks.last()?.hash.clone();
        let index = self.blocks.len() as u64;
        
        let mut block = Block::new(index, data, previous_hash, self.difficulty);
        block.mine(self.difficulty);
        
        self.blocks.push(block);
        Some(self.blocks.last().unwrap())
    }

    pub fn add_block_direct(&mut self, mut block: Block) -> bool {
        let last = match self.blocks.last() {
            Some(b) => b,
            None => return false,
        };
        
        if block.previous_hash != last.hash {
            return false;
        }
        
        block.set_hash(block.calculate_hash());
        self.blocks.push(block);
        true
    }

    pub fn get_block(&self, index: u64) -> Option<&Block> {
        self.blocks.get(index as usize)
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        self.blocks.iter().find(|b| b.hash == hash)
    }

    pub fn verify_integrity(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];
            
            if current.previous_hash != previous.hash {
                return false;
            }
            
            if current.hash != current.calculate_hash() {
                return false;
            }
        }
        true
    }

    pub fn get_latest_hash(&self) -> String {
        self.blocks.last().map(|b| b.hash.clone()).unwrap_or_default()
    }

    pub fn chain_length(&self) -> usize {
        self.blocks.len()
    }

    pub fn replace_chain(&mut self, new_chain: Vec<Block>) -> bool {
        if new_chain.len() <= self.blocks.len() {
            return false;
        }
        
        let mut temp_chain = new_chain;
        for i in 1..temp_chain.len() {
            if temp_chain[i].previous_hash != temp_chain[i - 1].hash {
                return false;
            }
            if temp_chain[i].hash != temp_chain[i].calculate_hash() {
                return false;
            }
        }
        
        self.blocks = temp_chain;
        true
    }

    pub fn calculate_total_work(&self) -> u64 {
        self.blocks.iter().map(|b| b.nonce as u64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let bc = Blockchain::new(2);
        assert_eq!(bc.chain_length(), 1);
    }

    #[test]
    fn test_add_block() {
        let mut bc = Blockchain::new(1);
        bc.add_block("Test data".to_string());
        assert_eq!(bc.chain_length(), 2);
    }

    #[test]
    fn test_verify_integrity() {
        let mut bc = Blockchain::new(1);
        bc.add_block("Block 1".to_string());
        bc.add_block("Block 2".to_string());
        assert!(bc.verify_integrity());
    }
}