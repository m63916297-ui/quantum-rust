use std::collections::HashMap;

const MAX_QUBITS: usize = 20;
const MAX_ITERATIONS: usize = 100;

#[derive(Clone, Debug)]
struct QuantumState {
    amplitudes: HashMap<u64, f64>,
}

impl QuantumState {
    fn new(size: usize) -> Self {
        let mut state = QuantumState {
            amplitudes: HashMap::new(),
        };
        state.amplitudes.insert(0, 1.0);
        for _ in 1..size {
            let new_amplitudes = HashMap::new();
            state = QuantumState { amplitudes: new_amplitudes };
            state.amplitudes.insert(0, 1.0);
        }
        state
    }

    fn apply_hadamard(&mut self) {
        let mut new_amplitudes = HashMap::new();
        let factor = 1.0 / (2.0_f64.sqrt());
        
        for (state, amp) in &self.amplitudes {
            for i in 0..2 {
                let new_state = (state << 1) | i as u64;
                let entry = new_amplitudes.entry(new_state).or_insert(0.0);
                *entry += factor * amp;
            }
        }
        self.amplitudes = new_amplitudes;
    }

    fn measure(&self) -> u64 {
        let total_prob: f64 = self.amplitudes.values().map(|x| x * x).sum();
        let mut rand_val = fast_random_f64() * total_prob;
        
        for (state, amp) in &self.amplitudes {
            rand_val -= amp * amp;
            if rand_val <= 0.0 {
                return *state;
            }
        }
        *self.amplitudes.keys().next().unwrap()
    }
}

fn fast_random_f64() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64) % 1000.0 / 1000.0
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    (g, x, y)
}

fn modular_pow(base: u64, exp: u64, mod_val: u64) -> u64 {
    let mut result = 1u64;
    let mut base = base % mod_val;
    let mut exp = exp;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % mod_val;
        }
        exp >>= 1;
        base = (base * base) % mod_val;
    }
    result
}

fn find_order_simulated(a: u64, n: u64) -> Option<u64> {
    for r in 1..=n {
        if modular_pow(a, r, n) == 1 {
            return Some(r);
        }
    }
    None
}

fn quantum_order_finding(n: u64, a: u64) -> Option<u64> {
    let num_qubits = ((n as f64).log2().ceil() as usize).max(2).min(MAX_QUBITS);
    
    if modular_pow(a, 1, n) == 1 {
        return Some(1);
    }
    
    let r = find_order_simulated(a, n);
    r
}

fn shor_algorithm(n: u64) -> Option<(u64, u64)> {
    if n % 2 == 0 {
        return Some((2, n / 2));
    }
    
    for _ in 0..MAX_ITERATIONS {
        let a = fast_random_f64() * (n as f64 - 2.0) as u64 + 2;
        
        if a >= n {
            continue;
        }
        
        let (g, _, _) = extended_gcd(a as i64, n as i64);
        if g > 1 {
            return Some((g as u64, n / g as u64));
        }
        
        if let Some(r) = quantum_order_finding(n, a) {
            if r % 2 == 0 {
                let x = modular_pow(a, r / 2, n);
                
                let (g1, _, _) = extended_gcd(x as i64 + 1, n as i64);
                let (g2, _, _) = extended_gcd(x as i64 - 1, n as i64);
                
                if g1 > 1 && g1 < n {
                    return Some((g1 as u64, n / g1 as u64));
                }
                if g2 > 1 && g2 < n {
                    return Some((g2 as u64, n / g2 as u64));
                }
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
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
        }
    }

    fn calculate_hash(&self) -> String {
        let mut content = format!(
            "{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash
        );
        
        for _ in 0..self.nonce {
            content.push_str("0");
        }
        
        let hash_val = content.bytes().fold(0u64, |acc, b| {
            acc.wrapping_mul(31).wrapping_add(b as u64)
        });
        
        format!("{:016x}", hash_val)
    }

    fn mine(&mut self, difficulty: u32) {
        let target = "0".repeat(difficulty as usize);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

struct Blockchain {
    chain: Vec<Block>,
    difficulty: u32,
}

impl Blockchain {
    fn new(difficulty: u32) -> Self {
        Blockchain {
            chain: Vec::new(),
            difficulty,
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_hash = if let Some(last) = self.chain.last() {
            last.hash.clone()
        } else {
            String::from("0")
        };
        
        let mut block = Block::new(self.chain.len() as u64, data, previous_hash);
        block.mine(self.difficulty);
        self.chain.push(block);
    }

    fn verify_integrity(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            
            if current.previous_hash != previous.hash {
                return false;
            }
            
            if current.calculate_hash() != current.hash {
                return false;
            }
        }
        true
    }
}

fn quantum_security_audit(blockchain: &Blockchain) -> HashMap<String, bool> {
    let mut results = HashMap::new();
    
    let chain_length = blockchain.chain.len();
    if chain_length > 0 {
        let last_block = &blockchain.chain[chain_length - 1];
        let hash_num = u64::from_str_radix(&last_block.hash, 16).unwrap_or(0);
        
        if let Some((p, q)) = shor_algorithm(hash_num) {
            let security_score = (p * q) as f64 / (hash_num as f64 + 1.0);
            results.insert("quantum_resistant".to_string(), security_score < 0.5);
        } else {
            results.insert("quantum_resistant".to_string(), true);
        }
    } else {
        results.insert("quantum_resistant".to_string(), true);
    }
    
    results.insert("integrity_verified".to_string(), blockchain.verify_integrity());
    
    let min_nonce = blockchain.chain.iter().map(|b| b.nonce).min().unwrap_or(0);
    results.insert("proof_of_work_valid".to_string(), min_nonce > 0);
    
    results
}

fn simulate_quantum_attack(n: u64, target_bits: u32) -> bool {
    if let Some((p, q)) = shor_algorithm(n) {
        let reconstructed = p * q;
        return reconstructed == n && p > 1 && q > 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shor_algorithm() {
        let result = shor_algorithm(15);
        assert!(result.is_some());
        let (p, q) = result.unwrap();
        assert_eq!(p * q, 15);
    }

    #[test]
    fn test_blockchain() {
        let mut blockchain = Blockchain::new(2);
        blockchain.add_block("Transaccion 1".to_string());
        blockchain.add_block("Transaccion 2".to_string());
        
        assert!(blockchain.verify_integrity());
    }

    #[test]
    fn test_quantum_audit() {
        let mut blockchain = Blockchain::new(1);
        blockchain.add_block("Test data".to_string());
        
        let results = quantum_security_audit(&blockchain);
        assert!(results.contains_key("quantum_resistant"));
    }
}

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("=== Algoritmo Cuántico de Shor - Auditoría de Blockchain ===\n");
    
    let mut blockchain = Blockchain::new(2);
    
    blockchain.add_block("Alice -> Bob: 10 coins".to_string());
    blockchain.add_block("Bob -> Charlie: 5 coins".to_string());
    blockchain.add_block("Charlie -> Alice: 3 coins".to_string());
    
    println!("Blockchain creado con {} bloques", blockchain.chain.len());
    println!("Integridad de la cadena: {}", blockchain.verify_integrity());
    
    println!("\n--- Auditoría de Seguridad Cuántica ---");
    let audit_results = quantum_security_audit(&blockchain);
    
    for (key, value) in &audit_results {
        println!("{}: {}", key, value);
    }
    
    println!("\n--- Prueba de Algoritmo de Shor ---");
    let test_numbers = vec![21, 35, 91, 143];
    
    for n in test_numbers {
        println!("Factorizando {}...", n);
        if let Some((p, q)) = shor_algorithm(n) {
            println!("  {} = {} x {}", n, p, q);
            
            let attack_success = simulate_quantum_attack(n, 32);
            println!("  Ataque cuántico simulado: {}", if attack_success { "EXITOSO" } else { "FALLIDO" });
        }
    }
    
    println!("\n--- Análisis de Resistencia Cuántica ---");
    let last_hash = &blockchain.chain.last().unwrap().hash;
    let hash_int = u64::from_str_radix(last_hash, 16).unwrap_or(0);
    
    if let Some((p, q)) = shor_algorithm(hash_int) {
        println!("El hash del último bloque puede ser factorizado:");
        println!("  {} = {} x {}", hash_int, p, q);
        println!("  ADVERTENCIA: La blockchain podría ser vulnerable a ataques cuánticos");
    } else {
        println!("El hash del último bloque es resistente a la factorización clásica");
    }
    
    println!("\n=== Auditoría Completada ===");
}