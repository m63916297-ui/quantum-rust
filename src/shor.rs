use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ShorError {
    #[error("Entrada inválida: {0} debe ser mayor que 1")]
    InvalidInput(u64),
    
    #[error("No se encontró factorización para {0} después de múltiples intentos")]
    FactorizationFailed(u64),
    
    #[error("Orden no encontrado para a={0} n={1}")]
    OrderNotFound(u64, u64),
    
    #[error("Números no son coprimos: a={0}, n={1}")]
    NotCoprime(u64, u64),
    
    #[error("Overflow en cálculo modular para base={0}, exp={1}, mod={2}")]
    ModularOverflow(u64, u64, u64),
    
    #[error("Iteración máxima excedida: {0}")]
    MaxIterationsExceeded(usize),
}

impl std::fmt::Display for ShorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub type ShorResult<T> = Result<T, ShorError>;

pub fn modular_pow(base: u64, exp: u64, mod_val: u64) -> u64 {
    if mod_val == 1 {
        return 0;
    }
    if mod_val == 0 {
        panic!("Módulo no puede ser cero");
    }
    
    let mut result = 1u64;
    let mut base = base % mod_val;
    let mut exp = exp;
    
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % mod_val;
        }
        exp >>= 1;
        base = (base * base) % mod_val;
    }
    result
}

pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    (g, x, y)
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / gcd(a, b)) * b
}

pub fn coprime(a: u64, b: u64) -> bool {
    gcd(a, b) == 1
}

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn prime_factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = Vec::new();
    
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    
    factors
}

pub fn find_order_bruteforce(a: u64, n: u64) -> Option<u64> {
    if !coprime(a, n) {
        return None;
    }
    for r in 1..=n {
        if modular_pow(a, r, n) == 1 {
            return Some(r);
        }
    }
    None
}

pub fn find_order_optimized(a: u64, n: u64, max_iter: u64) -> Option<u64> {
    if !coprime(a, n) {
        return None;
    }
    
    let mut r = 1;
    let mut current = a % n;
    
    while r <= max_iter {
        if current == 1 {
            return Some(r);
        }
        current = (current * a) % n;
        r += 1;
    }
    None
}

pub fn find_order_with_cycle_detection(a: u64, n: u64, max_iter: u64) -> Option<u64> {
    if !coprime(a, n) {
        return None;
    }
    
    let mut r = 1u64;
    let mut current = a % n;
    let mut seen: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
    
    while r <= max_iter {
        if current == 1 {
            return Some(r);
        }
        
        if let Some(&first_seen) = seen.get(&current) {
            return None;
        }
        seen.insert(current, r);
        
        current = (current * a) % n;
        r += 1;
    }
    None
}

pub fn continuous_fractions(value: f64, max_terms: usize) -> Vec<u64> {
    let mut terms = Vec::new();
    let mut x = value;
    let mut a0 = x.floor() as u64;
    terms.push(a0);
    
    let mut i = 0;
    while i < max_terms - 1 {
        x = x.fract().recip();
        if x.is_infinite() || x.is_nan() {
            break;
        }
        let a = x.floor() as u64;
        terms.push(a);
        i += 1;
        
        if (x - x.floor()).abs() < 1e-10 {
            break;
        }
    }
    terms
}

pub fn convergent(terms: &[u64], k: usize) -> (u64, u64) {
    if k >= terms.len() {
        return (0, 1);
    }
    
    let mut p_prev = 0u64;
    let mut p_curr = 1u64;
    let mut q_prev = 1u64;
    let mut q_curr = 0u64;
    
    for i in 0..=k {
        let a = terms[i];
        let p = a * p_curr + p_prev;
        let q = a * q_curr + q_prev;
        p_prev = p_curr;
        p_curr = p;
        q_prev = q_curr;
        q_curr = q;
    }
    
    (p_curr, q_curr)
}

pub fn shor_algorithm(n: u64) -> Option<(u64, u64)> {
    if n <= 1 {
        return None;
    }
    if n % 2 == 0 {
        return Some((2, n / 2));
    }
    
    if is_prime(n) {
        return None;
    }
    
    let max_attempts = 100;
    let mut attempts = 0;
    
    while attempts < max_attempts {
        attempts += 1;
        
        let a = random_coprime(n);
        
        let (g, _, _) = extended_gcd(a as i64, n as i64);
        if g > 1 && g < n {
            return Some((g as u64, n / g as u64));
        }
        
        if let Some(r) = find_order_optimized(a, n, n) {
            if r % 2 == 0 && r > 0 {
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

pub fn shor_algorithm_detailed(n: u64) -> ShorResult<(u64, u64, Vec<ShorDebugInfo>)> {
    let mut debug_info = Vec::new();
    
    if n <= 1 {
        return Err(ShorError::InvalidInput(n));
    }
    if n % 2 == 0 {
        return Ok((2, n / 2, vec![ShorDebugInfo::SimpleFactor(2)]));
    }
    if is_prime(n) {
        return Err(ShorError::FactorizationFailed(n));
    }
    
    let max_attempts = 100;
    
    for attempt in 0..max_attempts {
        let a = random_coprime(n);
        debug_info.push(ShorDebugInfo::Attempt { attempt: attempt + 1, a });
        
        let (g, x, y) = extended_gcd(a as i64, n as i64);
        if g > 1 && g < n {
            debug_info.push(ShorDebugInfo::GcdFound(g as u64));
            return Ok((g as u64, n / g as u64, debug_info));
        }
        
        if let Some(r) = find_order_optimized(a, n, n) {
            debug_info.push(ShorDebugInfo::OrderFound(r));
            
            if r % 2 == 0 && r > 0 {
                let x = modular_pow(a, r / 2, n);
                debug_info.push(ShorDebugInfo::MidPoint(x));
                
                let (g1, _, _) = extended_gcd(x as i64 + 1, n as i64);
                let (g2, _, _) = extended_gcd(x as i64 - 1, n as i64);
                
                if g1 > 1 && g1 < n {
                    debug_info.push(ShorDebugInfo::FactorFound(g1 as u64));
                    return Ok((g1 as u64, n / g1 as u64, debug_info));
                }
                if g2 > 1 && g2 < n {
                    debug_info.push(ShorDebugInfo::FactorFound(g2 as u64));
                    return Ok((g2 as u64, n / g2 as u64, debug_info));
                }
            }
        }
    }
    
    Err(ShorError::FactorizationFailed(n))
}

#[derive(Debug, Clone)]
pub enum ShorDebugInfo {
    Attempt { attempt: usize, a: u64 },
    GcdFound(u64),
    OrderFound(u64),
    MidPoint(u64),
    FactorFound(u64),
    SimpleFactor(u64),
}

fn random_coprime(n: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    
    let mut a = (seed % (n.saturating_sub(2)).max(1)) + 2;
    
    let mut attempts = 0;
    while !coprime(a, n) && attempts < 1000 {
        a = ((a * 1103515245 + 12345) % (n.saturating_sub(2)).max(1)) + 2;
        attempts += 1;
    }
    
    a
}

pub fn shor_factorization_batch(numbers: &[u64]) -> std::collections::HashMap<u64, Option<(u64, u64)>> {
    let mut results = std::collections::HashMap::new();
    for &n in numbers {
        results.insert(n, shor_algorithm(n));
    }
    results
}

pub fn estimate_qubit_requirements(n: u64) -> usize {
    let bits = (n as f64).log2().ceil() as usize;
    bits * 2 + 1
}

pub fn estimate_circuit_depth(n: u64) -> usize {
    let bits = (n as f64).log2().ceil() as usize;
    bits * bits
}

pub fn verify_factorization(n: u64, p: u64, q: u64) -> bool {
    p > 1 && q > 1 && p * q == n
}

pub fn factorization_complexity(n: u64) -> (f64, String) {
    if n < 100 {
        return (0.1, "Trivial".to_string());
    }
    if n < 1000 {
        return (0.3, "Fácil".to_string());
    }
    if n < 10000 {
        return (0.5, "Moderado".to_string());
    }
    if n < 100000 {
        return (0.7, "Difícil".to_string());
    }
    (0.9, "Muy difícil".to_string())
}

pub struct ShorConfig {
    pub max_attempts: usize,
    pub max_order_iterations: u64,
    pub use_cycle_detection: bool,
    pub verbose: bool,
}

impl Default for ShorConfig {
    fn default() -> Self {
        ShorConfig {
            max_attempts: 100,
            max_order_iterations: 10000,
            use_cycle_detection: true,
            verbose: false,
        }
    }
}

pub fn shor_algorithm_with_config(n: u64, config: &ShorConfig) -> Option<(u64, u64)> {
    if n <= 1 || n % 2 == 0 {
        return if n > 1 { Some((2, n / 2)) } else { None };
    }
    
    for _ in 0..config.max_attempts {
        let a = random_coprime(n);
        
        let (g, _, _) = extended_gcd(a as i64, n as i64);
        if g > 1 && g < n {
            return Some((g as u64, n / g as u64));
        }
        
        let order = if config.use_cycle_detection {
            find_order_with_cycle_detection(a, n, config.max_order_iterations)
        } else {
            find_order_optimized(a, n, config.max_order_iterations)
        };
        
        if let Some(r) = order {
            if r % 2 == 0 && r > 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_pow() {
        assert_eq!(modular_pow(2, 10, 1024), 1024);
        assert_eq!(modular_pow(3, 4, 10), 1);
        assert_eq!(modular_pow(7, 3, 10), 3);
        assert_eq!(modular_pow(5, 0, 13), 1);
        assert_eq!(modular_pow(2, 1, 2), 0);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(100, 25), 25);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(5, 7), 35);
    }

    #[test]
    fn test_coprime() {
        assert!(coprime(7, 9));
        assert!(!coprime(8, 12));
        assert!(coprime(15, 31));
        assert!(coprime(1, 100));
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(is_prime(17));
        assert!(!is_prime(100));
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
        assert_eq!(prime_factors(15), vec![3, 5]);
        assert_eq!(prime_factors(7), vec![7]);
    }

    #[test]
    fn test_shor_15() {
        let result = shor_algorithm(15);
        assert!(result.is_some());
        let (p, q) = result.unwrap();
        assert!(verify_factorization(15, p, q));
    }

    #[test]
    fn test_shor_21() {
        let result = shor_algorithm(21);
        assert!(result.is_some());
        let (p, q) = result.unwrap();
        assert!(verify_factorization(21, p, q));
    }

    #[test]
    fn test_shor_35() {
        let result = shor_algorithm(35);
        assert!(result.is_some());
        let (p, q) = result.unwrap();
        assert!(verify_factorization(35, p, q));
    }

    #[test]
    fn test_shor_even() {
        let result = shor_algorithm(14);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), (2, 7));
    }

    #[test]
    fn test_shor_91() {
        let result = shor_algorithm(91);
        assert!(result.is_some());
        let (p, q) = result.unwrap();
        assert!(verify_factorization(91, p, q));
    }

    #[test]
    fn test_shor_143() {
        let result = shor_algorithm(143);
        assert!(result.is_some());
        let (p, q) = result.unwrap();
        assert!(verify_factorization(143, p, q));
    }

    #[test]
    fn test_shor_detailed() {
        let result = shor_algorithm_detailed(15);
        assert!(result.is_ok());
        let (p, q, debug) = result.unwrap();
        assert!(verify_factorization(15, p, q));
        assert!(!debug.is_empty());
    }

    #[test]
    fn test_shor_config() {
        let config = ShorConfig::default();
        let result = shor_algorithm_with_config(21, &config);
        assert!(result.is_some());
    }

    #[test]
    fn test_verify_factorization() {
        assert!(verify_factorization(15, 3, 5));
        assert!(verify_factorization(21, 3, 7));
        assert!(!verify_factorization(15, 2, 8));
        assert!(!verify_factorization(15, 1, 15));
    }

    #[test]
    fn test_estimate_qubits() {
        let qubits = estimate_qubit_requirements(15);
        assert!(qubits >= 8);
        let qubits_large = estimate_qubit_requirements(1000);
        assert!(qubits_large >= qubits);
    }

    #[test]
    fn test_continuous_fractions() {
        let terms = continuous_fractions(3.14159, 10);
        assert!(!terms.is_empty());
        assert_eq!(terms[0], 3);
    }

    #[test]
    fn test_convergent() {
        let terms = vec![3, 7, 15, 1];
        let (p, q) = convergent(&terms, 2);
        assert!(p > 0 && q > 0);
    }

    #[test]
    fn test_find_order() {
        let order = find_order_bruteforce(2, 15);
        assert_eq!(order, Some(4));
        
        let order2 = find_order_bruteforce(2, 7);
        assert_eq!(order2, Some(3));
    }

    #[test]
    fn test_batch_factorization() {
        let numbers = vec![15, 21, 35, 91];
        let results = shor_factorization_batch(&numbers);
        
        for (n, result) in results.iter() {
            if let Some((p, q)) = result {
                assert!(verify_factorization(*n, *p, *q));
            }
        }
    }

    #[test]
    fn test_factorization_complexity() {
        let (level, desc) = factorization_complexity(50);
        assert!(level < 0.5);
        
        let (level2, desc2) = factorization_complexity(50000);
        assert!(level2 > 0.5);
    }
}