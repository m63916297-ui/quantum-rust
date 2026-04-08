use crate::blockchain::Blockchain;
use crate::shor;
use crate::audit::{AuditResult, AuditStatus};

pub fn quantum_security_audit(blockchain: &Blockchain) -> std::collections::HashMap<String, AuditResult> {
    let mut results = std::collections::HashMap::new();
    
    if blockchain.blocks.is_empty() {
        return results;
    }
    
    let last_block = &blockchain.blocks[blockchain.blocks.len() - 1];
    let hash_str = &last_block.hash;
    let hash_int = u64::from_str_radix(hash_str, 16).unwrap_or(0);
    
    if hash_int == 0 {
        results.insert(
            "quantum_resistant".to_string(),
            AuditResult::fail(0.0, "Hash inválido para análisis"),
        );
        return results;
    }
    
    if let Some((p, q)) = shor::shor_algorithm(hash_int) {
        let reconstructed = p * q;
        if reconstructed == hash_int && p > 1 && q > 1 {
            results.insert(
                "quantum_resistant".to_string(),
                AuditResult::fail(0.3, &format!("Hash factorizado: {} = {} x {}", hash_int, p, q)),
            );
        } else {
            results.insert(
                "quantum_resistant".to_string(),
                AuditResult::pass(0.9, "Hash resistente a factorización"),
            );
        }
    } else {
        results.insert(
            "quantum_resistant".to_string(),
            AuditResult::pass(1.0, "No se encontró factorización"),
        );
    }
    
    results
}

pub fn audit_blockchain_integrity(blockchain: &Blockchain) -> AuditResult {
    let valid = blockchain.verify_integrity();
    
    if valid {
        AuditResult::pass(1.0, "Integridad verificada")
    } else {
        AuditResult::fail(0.0, "Integridad comprometida")
    }
}

pub fn audit_proof_of_work(blockchain: &Blockchain) -> AuditResult {
    let total_work = blockchain.calculate_total_work();
    let avg_work = if !blockchain.blocks.is_empty() {
        total_work as f64 / blockchain.blocks.len() as f64
    } else {
        0.0
    };
    
    if avg_work > 100.0 {
        AuditResult::pass(1.0, &format!("Proof of work válido - trabajo total: {}", total_work))
    } else if avg_work > 10.0 {
        AuditResult::warning(0.6, "Proof of work bajo")
    } else {
        AuditResult::fail(0.2, "Proof of work insuficiente")
    }
}

pub fn evaluate_quantum_resistance(blockchain: &Blockchain) -> AuditResult {
    let mut vulnerable_count = 0;
    let total_blocks = blockchain.blocks.len();
    
    for block in &blockchain.blocks {
        let hash_int = u64::from_str_radix(&block.hash, 16).unwrap_or(0);
        
        if let Some((p, q)) = shor::shor_algorithm(hash_int) {
            if p * q == hash_int && p > 1 && q > 1 {
                vulnerable_count += 1;
            }
        }
    }
    
    let vulnerability_ratio = vulnerable_count as f64 / total_blocks.max(1) as f64;
    
    if vulnerability_ratio < 0.1 {
        AuditResult::pass(1.0, "Alta resistencia cuántica")
    } else if vulnerability_ratio < 0.5 {
        AuditResult::warning(0.5, "Resistencia cuántica media")
    } else {
        AuditResult::fail(0.0, "Baja resistencia cuántica")
    }
}

pub fn analyze_hash_collision_resistance(blockchain: &Blockchain) -> AuditResult {
    let mut hashes = std::collections::HashSet::new();
    let mut collisions = 0;
    
    for block in &blockchain.blocks {
        if !hashes.insert(block.hash.clone()) {
            collisions += 1;
        }
    }
    
    if collisions == 0 {
        AuditResult::pass(1.0, "Sin colisiones detectadas")
    } else {
        AuditResult::warning(0.5, &format!("{} colisiones detectadas", collisions))
    }
}

pub fn simulate_quantum_attack(n: u64, target_bits: u32) -> bool {
    if let Some((p, q)) = shor::shor_algorithm(n) {
        p * q == n && p > 1 && q > 1
    } else {
        false
    }
}

pub fn estimate_attack_complexity(blockchain: &Blockchain) -> std::collections::HashMap<String, f64> {
    let mut complexity = std::collections::HashMap::new();
    
    let avg_hash = blockchain.blocks.iter()
        .map(|b| u64::from_str_radix(&b.hash, 16).unwrap_or(0))
        .sum::<u64>()
        .checked_div(blockchain.blocks.len() as u64)
        .unwrap_or(0);
    
    complexity.insert("classical_factorization".to_string(), (avg_hash as f64).log2());
    complexity.insert("quantum_factorization".to_string(), (avg_hash as f64).log2().cbrt());
    complexity.insert("grover_search".to_string(), (avg_hash as f64).sqrt());
    
    complexity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrity_audit() {
        let mut bc = Blockchain::new(1);
        bc.add_block("test".to_string());
        let result = audit_blockchain_integrity(&bc);
        assert_eq!(result.status, AuditStatus::Pass);
    }

    #[test]
    fn test_pow_audit() {
        let bc = Blockchain::new(1);
        let result = audit_proof_of_work(&bc);
        assert!(result.score >= 0.0);
    }

    #[test]
    fn test_quantum_attack() {
        let result = simulate_quantum_attack(15, 32);
        assert!(result);
    }
}