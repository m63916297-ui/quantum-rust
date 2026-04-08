mod block;
mod chain;
mod miner;

pub use block::Block;
pub use chain::Blockchain;
pub use miner::Miner;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: f64) -> Self {
        Transaction {
            from,
            to,
            amount,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

pub fn validate_transaction(tx: &Transaction) -> bool {
    tx.amount > 0.0 && !tx.from.is_empty() && !tx.to.is_empty()
}