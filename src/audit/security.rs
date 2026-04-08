use thiserror::Error;
use crate::blockchain::Blockchain;
use crate::shor;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug, Clone)]
pub enum AuditError {
    #[error("Blockchain inválido: {0}")]
    InvalidBlockchain(String),
    
    #[error("Factorización fallida: {0}")]
    FactorizationFailed(String),
    
    #[error("Error de análisis: {0}")]
    AnalysisError(String),
}

pub type AuditResult<T> = Result<T, AuditError>;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AuditStatus {
    Pass,
    Warning,
    Fail,
    Unknown,
}

impl AuditStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditStatus::Pass => "PASS",
            AuditStatus::Warning => "WARNING",
            AuditStatus::Fail => "FAIL",
            AuditStatus::Unknown => "UNKNOWN",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditResult {
    pub test_name: String,
    pub status: AuditStatus,
    pub score: f64,
    pub details: String,
    pub recommendations: Vec<String>,
}

impl SecurityAuditResult {
    pub fn pass(test_name: &str, score: f64, details: &str) -> Self {
        SecurityAuditResult {
            test_name: test_name.to_string(),
            status: AuditStatus::Pass,
            score,
            details: details.to_string(),
            recommendations: vec![],
        }
    }

    pub fn warning(test_name: &str, score: f64, details: &str, recommendations: Vec<String>) -> Self {
        SecurityAuditResult {
            test_name: test_name.to_string(),
            status: AuditStatus::Warning,
            score,
            details: details.to_string(),
            recommendations,
        }
    }

    pub fn fail(test_name: &str, score: f64, details: &str, recommendations: Vec<String>) -> Self {
        SecurityAuditResult {
            test_name: test_name.to_string(),
            status: AuditStatus::Fail,
            score,
            details: details.to_string(),
            recommendations,
        }
    }
}

pub fn quantum_security_audit(blockchain: &Blockchain) -> Vec<SecurityAuditResult> {
    let mut results = Vec::new();
    
    if blockchain.blocks.is_empty() {
        results.push(SecurityAuditResult::fail(
            "blockchain_empty",
            0.0,
            "La blockchain está vacía",
            vec!["Crear al menos un bloque genesis".to_string()],
        ));
        return results;
    }
    
    results.push(audit_blockchain_integrity(blockchain));
    results.push(audit_proof_of_work(blockchain));
    results.push(audit_quantum_resistance(blockchain));
    results.push(audit_hash_distribution(blockchain));
    results.push(audit_chain_continuity(blockchain));
    results.push(audit_difficulty_calibration(blockchain));
    results.push(audit_block_timestamps(blockchain));
    
    results
}

pub fn audit_blockchain_integrity(blockchain: &Blockchain) -> SecurityAuditResult {
    let valid = blockchain.verify_integrity();
    
    if valid {
        SecurityAuditResult::pass(
            "integrity_check",
            1.0,
            &format!(
                "Integridad verificada - {} bloques válidos",
                blockchain.chain_length()
            ),
        )
    } else {
        let errors = blockchain.verify_integrity_detailed();
        let recommendations = vec![
            "Reconstruir la cadena desde el bloque genesis".to_string(),
            "Verificar manualmente cada bloque".to_string(),
        ];
        
        SecurityAuditResult::fail(
            "integrity_check",
            0.0,
            &format!("{} errores encontrados en la cadena", errors.len()),
            recommendations,
        )
    }
}

pub fn audit_proof_of_work(blockchain: &Blockchain) -> SecurityAuditResult {
    let total_work = blockchain.calculate_total_work();
    let avg_work = if !blockchain.blocks.is_empty() {
        total_work as f64 / blockchain.chain_length() as f64
    } else {
        0.0
    };
    
    if avg_work > 100.0 {
        SecurityAuditResult::pass(
            "proof_of_work",
            1.0,
            &format!("POW válido - trabajo total: {}, promedio: {:.1}", total_work, avg_work),
        )
    } else if avg_work > 10.0 {
        SecurityAuditResult::warning(
            "proof_of_work",
            0.6,
            &format!("POW bajo - promedio: {:.1}", avg_work),
            vec!["Considerar aumentar la dificultad".to_string()],
        )
    } else {
        let recommendations = vec![
            "Aumentar significativamente la dificultad".to_string(),
            "Revisar el algoritmo de minería".to_string(),
        ];
        
        SecurityAuditResult::fail(
            "proof_of_work",
            0.2,
            &format!("POW insuficiente - promedio: {:.1}", avg_work),
            recommendations,
        )
    }
}

pub fn audit_quantum_resistance(blockchain: &Blockchain) -> SecurityAuditResult {
    let mut vulnerable_count = 0;
    let total_blocks = blockchain.chain_length();
    let mut vulnerable_hashes = Vec::new();
    
    for block in &blockchain.blocks {
        let hash_int = u64::from_str_radix(&block.hash, 16).unwrap_or(0);
        
        if hash_int > 100 {
            if let Some((p, q)) = shor::shor_algorithm(hash_int) {
                if p * q == hash_int && p > 1 && q > 1 {
                    vulnerable_count += 1;
                    vulnerable_hashes.push((block.index, hash_int, p, q));
                }
            }
        }
    }
    
    let vulnerability_ratio = if total_blocks > 0 {
        vulnerable_count as f64 / total_blocks as f64
    } else {
        0.0
    };
    
    if vulnerability_ratio < 0.1 {
        SecurityAuditResult::pass(
            "quantum_resistance",
            1.0 - vulnerability_ratio,
            &format!(
                "Alta resistencia cuántica - {}/{} hashes vulnerables",
                vulnerable_count, total_blocks
            ),
        )
    } else if vulnerability_ratio < 0.5 {
        let mut recommendations = vec![
            "Considerar usar hashes más grandes".to_string(),
            "Implementar algoritmos post-quantum".to_string(),
        ];
        
        SecurityAuditResult::warning(
            "quantum_resistance",
            0.5,
            &format!(
                "Resistencia cuántica media - {}/{} hashes vulnerables",
                vulnerable_count, total_blocks
            ),
            recommendations,
        )
    } else {
        let mut recommendations = vec![
            "URGENTE: Actualizar a algoritmos post-quantum".to_string(),
            "Implementar firmas post-quantum".to_string(),
            "Considerar migración a加密post-cuántica".to_string(),
        ];
        
        SecurityAuditResult::fail(
            "quantum_resistance",
            0.0,
            &format!(
                "BAJA resistencia cuántica - {}/{} hashes vulnerables",
                vulnerable_count, total_blocks
            ),
            recommendations,
        )
    }
}

pub fn audit_hash_distribution(blockchain: &Blockchain) -> SecurityAuditResult {
    let hashes: Vec<u64> = blockchain.blocks.iter()
        .filter_map(|b| u64::from_str_radix(&b.hash, 16).ok())
        .collect();
    
    if hashes.is_empty() {
        return SecurityAuditResult::fail(
            "hash_distribution",
            0.0,
            "No se pudieron analizar los hashes",
            vec!["Verificar formato de hashes".to_string()],
        );
    }
    
    let sum: u64 = hashes.iter().sum();
    let avg = sum as f64 / hashes.len() as f64;
    
    let variance: f64 = hashes.iter()
        .map(|&h| {
            let diff = h as f64 - avg;
            diff * diff
        })
        .sum::<f64>() / hashes.len() as f64;
    
    let std_dev = variance.sqrt();
    let coefficient_of_variation = std_dev / avg;
    
    if coefficient_of_variation > 0.3 {
        SecurityAuditResult::pass(
            "hash_distribution",
            1.0,
            &format!("Distribución buena - CV: {:.3}", coefficient_of_variation),
        )
    } else if coefficient_of_variation > 0.1 {
        SecurityAuditResult::warning(
            "hash_distribution",
            0.7,
            &format!("Distribución moderada - CV: {:.3}", coefficient_of_variation),
            vec!["Monitorear distribución de hashes".to_string()],
        )
    } else {
        SecurityAuditResult::fail(
            "hash_distribution",
            0.3,
            &format!("Distribución pobre - CV: {:.3}", coefficient_of_variation),
            vec!["Posible patrón predecible en hashes".to_string()],
        )
    }
}

pub fn audit_chain_continuity(blockchain: &Blockchain) -> SecurityAuditResult {
    let mut continuity_errors = 0;
    
    for i in 1..blockchain.chain_length() {
        let current = &blockchain.blocks[i];
        let previous = &blockchain.blocks[i - 1];
        
        if current.previous_hash != previous.hash {
            continuity_errors += 1;
        }
        
        if current.index != previous.index + 1 {
            continuity_errors += 1;
        }
    }
    
    if continuity_errors == 0 {
        SecurityAuditResult::pass(
            "chain_continuity",
            1.0,
            "Todos los bloques están correctamente enlazados",
        )
    } else {
        SecurityAuditResult::fail(
            "chain_continuity",
            0.0,
            &format!("{} errores de continuidad encontrados", continuity_errors),
            vec!["Verificar la cadena manualmente".to_string()],
        )
    }
}

pub fn audit_difficulty_calibration(blockchain: &Blockchain) -> SecurityAuditResult {
    let difficulties: Vec<u32> = blockchain.blocks.iter()
        .map(|b| b.difficulty)
        .collect();
    
    if difficulties.is_empty() {
        return SecurityAuditResult::fail(
            "difficulty_calibration",
            0.0,
            "No hay bloques para analizar",
            vec![],
        );
    }
    
    let unique_difficulties = difficulties.iter().collect::<std::collections::HashSet<_>>().len();
    
    if unique_difficulties == 1 {
        SecurityAuditResult::pass(
            "difficulty_calibration",
            1.0,
            &format!("Dificultad consistente: {}", difficulties[0]),
        )
    } else if unique_difficulties <= 3 {
        SecurityAuditResult::warning(
            "difficulty_calibration",
            0.7,
            &format!("{} diferentes niveles de dificultad encontrados", unique_difficulties),
            vec!["Verificar ajuste de dificultad".to_string()],
        )
    } else {
        SecurityAuditResult::fail(
            "difficulty_calibration",
            0.3,
            &format!("Demasiados cambios de dificultad: {}", unique_difficulties),
            vec!["Revisar algoritmo de ajuste de dificultad".to_string()],
        )
    }
}

pub fn audit_block_timestamps(blockchain: &Blockchain) -> SecurityAuditResult {
    let mut timestamp_errors = 0;
    
    for i in 1..blockchain.chain_length() {
        let current = &blockchain.blocks[i];
        let previous = &blockchain.blocks[i - 1];
        
        if current.timestamp <= previous.timestamp {
            timestamp_errors += 1;
        }
    }
    
    if timestamp_errors == 0 {
        SecurityAuditResult::pass(
            "block_timestamps",
            1.0,
            "Todos los timestamps son válidos",
        )
    } else {
        SecurityAuditResult::fail(
            "block_timestamps",
            0.5,
            &format!("{} timestamps inválidos encontrados", timestamp_errors),
            vec!["Verificar sincronización del sistema".to_string()],
        )
    }
}

pub fn simulate_quantum_attack(n: u64, target_bits: u32) -> (bool, Option<(u64, u64)>) {
    if let Some(factors) = shor::shor_algorithm(n) {
        let success = factors.0 * factors.1 == n && factors.0 > 1 && factors.1 > 1;
        (success, Some(factors))
    } else {
        (false, None)
    }
}

pub fn estimate_attack_complexity(blockchain: &Blockchain) -> std::collections::HashMap<String, f64> {
    let mut complexity = std::collections::HashMap::new();
    
    if blockchain.blocks.is_empty() {
        return complexity;
    }
    
    let avg_hash = blockchain.blocks.iter()
        .filter_map(|b| u64::from_str_radix(&b.hash, 16).ok())
        .sum::<u64>()
        .checked_div(blockchain.blocks.len() as u64)
        .unwrap_or(0);
    
    if avg_hash > 0 {
        complexity.insert("classical_factorization".to_string(), (avg_hash as f64).log2());
        complexity.insert("quantum_factorization".to_string(), (avg_hash as f64).log2().cbrt());
        complexity.insert("grover_search".to_string(), (avg_hash as f64).sqrt());
    }
    
    complexity
}

pub fn calculate_security_score(results: &[SecurityAuditResult]) -> f64 {
    if results.is_empty() {
        return 0.0;
    }
    
    let total: f64 = results.iter().map(|r| r.score).sum();
    total / results.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_blockchain() {
        let mut bc = Blockchain::new(1);
        bc.add_block("test".to_string());
        
        let results = quantum_security_audit(&bc);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_security_score() {
        let results = vec![
            SecurityAuditResult::pass("test1", 1.0, "OK"),
            SecurityAuditResult::pass("test2", 0.8, "OK"),
        ];
        
        let score = calculate_security_score(&results);
        assert!((score - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_quantum_attack() {
        let (success, factors) = simulate_quantum_attack(15, 32);
        assert!(success);
        assert!(factors.is_some());
    }
}