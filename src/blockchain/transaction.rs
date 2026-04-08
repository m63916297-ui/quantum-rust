use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum TransactionError {
    #[error("Monto inválido: {0}")]
    InvalidAmount(f64),
    
    #[error("Dirección inválida: {0}")]
    InvalidAddress(String),
    
    #[error("Firma inválida")]
    InvalidSignature,
    
    #[error("Balance insuficiente: requerido {0}, disponible {1}")]
    InsufficientBalance(f64, f64),
}

pub type TransactionResult<T> = Result<T, TransactionError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub timestamp: u64,
    pub fee: f64,
    pub signature: Option<String>,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: f64) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let id = Self::generate_id(&from, &to, amount, timestamp);
        
        Transaction {
            id,
            from,
            to,
            amount,
            timestamp,
            fee: 0.0,
            signature: None,
            status: TransactionStatus::Pending,
        }
    }

    fn generate_id(from: &str, to: &str, amount: f64, timestamp: u64) -> String {
        let content = format!("{}:{}:{}:{}", from, to, amount, timestamp);
        let hash = content.bytes().fold(0u64, |acc, b| {
            acc.wrapping_mul(31).wrapping_add(b as u64)
        });
        format!("{:016x}", hash)
    }

    pub fn with_fee(mut self, fee: f64) -> Self {
        self.fee = fee;
        self
    }

    pub fn sign(&mut self, signature: String) {
        self.signature = Some(signature);
    }

    pub fn confirm(&mut self) {
        self.status = TransactionStatus::Confirmed;
    }

    pub fn fail(&mut self) {
        self.status = TransactionStatus::Failed;
    }

    pub fn is_valid(&self) -> bool {
        self.amount > 0.0 
            && !self.from.is_empty() 
            && !self.to.is_empty()
            && self.from != self.to
    }

    pub fn total_amount(&self) -> f64 {
        self.amount + self.fee
    }
}

pub struct TransactionValidator;

impl TransactionValidator {
    pub fn validate(tx: &Transaction) -> TransactionResult<()> {
        if tx.amount <= 0.0 {
            return Err(TransactionError::InvalidAmount(tx.amount));
        }
        
        if tx.from.is_empty() {
            return Err(TransactionError::InvalidAddress("Remitente vacío".to_string()));
        }
        
        if tx.to.is_empty() {
            return Err(TransactionError::InvalidAddress("Destinatario vacío".to_string()));
        }
        
        if tx.from == tx.to {
            return Err(TransactionError::InvalidAddress("No se puede enviar a sí mismo".to_string()));
        }
        
        Ok(())
    }

    pub fn validate_transfer(tx: &Transaction, balance: f64) -> TransactionResult<()> {
        Self::validate(tx)?;
        
        if balance < tx.total_amount() {
            return Err(TransactionError::InsufficientBalance(
                tx.total_amount(),
                balance,
            ));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new(
            "Alice".to_string(),
            "Bob".to_string(),
            10.0,
        );
        
        assert_eq!(tx.from, "Alice");
        assert_eq!(tx.to, "Bob");
        assert_eq!(tx.amount, 10.0);
        assert_eq!(tx.status, TransactionStatus::Pending);
    }

    #[test]
    fn test_transaction_valid() {
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
        assert!(tx.is_valid());
    }

    #[test]
    fn test_transaction_invalid_amount() {
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), -10.0);
        assert!(!tx.is_valid());
    }

    #[test]
    fn test_transaction_same_address() {
        let tx = Transaction::new("Alice".to_string(), "Alice".to_string(), 10.0);
        assert!(!tx.is_valid());
    }

    #[test]
    fn test_transaction_with_fee() {
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0)
            .with_fee(0.5);
        
        assert_eq!(tx.fee, 0.5);
        assert_eq!(tx.total_amount(), 10.5);
    }

    #[test]
    fn test_transaction_sign() {
        let mut tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
        tx.sign("signature123".to_string());
        assert!(tx.signature.is_some());
    }

    #[test]
    fn test_validator() {
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
        assert!(TransactionValidator::validate(&tx).is_ok());
    }

    #[test]
    fn test_validator_insufficient_balance() {
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 100.0);
        let result = TransactionValidator::validate_transfer(&tx, 50.0);
        assert!(result.is_err());
    }
}