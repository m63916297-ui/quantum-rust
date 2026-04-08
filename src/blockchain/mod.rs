mod block;
mod chain;
mod miner;
mod transaction;

pub use block::{Block, BlockBuilder};
pub use chain::{Blockchain, ChainMetadata, ChainError, ChainResult};
pub use miner::{Miner, MinerError, MinerResult};
pub use transaction::{Transaction, TransactionValidator};

use serde::{Deserialize, Serialize};

pub fn validate_transaction(tx: &Transaction) -> bool {
    tx.amount > 0.0 && !tx.from.is_empty() && !tx.to.is_empty()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    pub index: u64,
    pub hash: String,
    pub timestamp: u64,
    pub data: String,
    pub nonce: u64,
}

impl From<&Block> for BlockInfo {
    fn from(block: &Block) -> Self {
        BlockInfo {
            index: block.index,
            hash: block.hash.clone(),
            timestamp: block.timestamp,
            data: block.data.clone(),
            nonce: block.nonce,
        }
    }
}