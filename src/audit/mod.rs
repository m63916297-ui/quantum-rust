mod security;
mod report;

pub use security::*;
pub use report::*;

use std::collections::HashMap;

pub type AuditResults = HashMap<String, AuditResult>;

#[derive(Debug, Clone)]
pub struct AuditResult {
    pub status: AuditStatus,
    pub score: f64,
    pub details: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuditStatus {
    Pass,
    Warning,
    Fail,
}

impl AuditResult {
    pub fn pass(score: f64, details: &str) -> Self {
        AuditResult {
            status: AuditStatus::Pass,
            score,
            details: details.to_string(),
        }
    }

    pub fn warning(score: f64, details: &str) -> Self {
        AuditResult {
            status: AuditStatus::Warning,
            score,
            details: details.to_string(),
        }
    }

    pub fn fail(score: f64, details: &str) -> Self {
        AuditResult {
            status: AuditStatus::Fail,
            score,
            details: details.to_string(),
        }
    }
}

pub fn run_full_audit(blockchain: &super::blockchain::Blockchain) -> AuditResults {
    let mut results = security::quantum_security_audit(blockchain);
    
    let integrity = security::audit_blockchain_integrity(blockchain);
    results.insert("integrity_check".to_string(), integrity);
    
    let pow = security::audit_proof_of_work(blockchain);
    results.insert("proof_of_work".to_string(), pow);
    
    let resistance = security::evaluate_quantum_resistance(blockchain);
    results.insert("quantum_resistance".to_string(), resistance);
    
    results
}

pub fn calculate_overall_score(results: &AuditResults) -> f64 {
    if results.is_empty() {
        return 0.0;
    }
    
    let total: f64 = results.values().map(|r| r.score).sum();
    total / results.len() as f64
}

pub fn generate_summary(results: &AuditResults) -> String {
    let score = calculate_overall_score(results);
    let passed = results.values().filter(|r| r.status == AuditStatus::Pass).count();
    let warnings = results.values().filter(|r| r.status == AuditStatus::Warning).count();
    let failed = results.values().filter(|r| r.status == AuditStatus::Fail).count();
    
    format!(
        "Audit Score: {:.1}%\nPassed: {}, Warnings: {}, Failed: {}",
        score * 100.0,
        passed,
        warnings,
        failed
    )
}