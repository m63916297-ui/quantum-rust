use std::collections::HashMap;
use std::cmp::Ordering;

pub fn modular_pow(base: u64, exp: u64, mod_val: u64) -> u64 {
    if mod_val == 1 {
        return 0;
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

pub fn coprime(a: u64, b: u64) -> bool {
    gcd(a, b) == 1
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
    
    for _ in 0..100 {
        let a = random_coprime(n);
        
        let (g, _, _) = extended_gcd(a as i64, n as i64);
        if g > 1 {
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

fn random_coprime(n: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    
    let mut a = (seed % (n - 2)) + 2;
    while !coprime(a, n) {
        a = ((a * 1103515245 + 12345) % (n - 2)) + 2;
    }
    a
}

pub fn shor_factorization_batch(numbers: &[u64]) -> HashMap<u64, Option<(u64, u64)>> {
    let mut results = HashMap::new();
    for &n in numbers {
        results.insert(n, shor_algorithm(n));
    }
    results
}

pub fn estimate_qubit_requirements(n: u64) -> usize {
    let bits = (n as f64).log2().ceil() as usize;
    bits * 2
}

pub fn verify_factorization(n: u64, p: u64, q: u64) -> bool {
    p > 1 && q > 1 && p * q == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_pow() {
        assert_eq!(modular_pow(2, 10, 1024), 1024);
        assert_eq!(modular_pow(3, 4, 10), 1);
        assert_eq!(modular_pow(7, 3, 10), 3);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(100, 25), 25);
    }

    #[test]
    fn test_coprime() {
        assert!(coprime(7, 9));
        assert!(!coprime(8, 12));
        assert!(coprime(15, 31));
    }

    #[test]
    fn test_shor_15() {
        let result = shor_algorithm(15);
        assert!(result.is_some());
        let (p, q) = result.unwrap();
        assert_eq!(p * q, 15);
        assert!(p > 1 && q > 1);
    }

    #[test]
    fn test_shor_21() {
        let result = shor_algorithm(21);
        assert!(result.is_some());
        let (p, q) = result.unwrap();
        assert!(verify_factorization(21, p, q));
    }

    #[test]
    fn test_verify_factorization() {
        assert!(verify_factorization(15, 3, 5));
        assert!(verify_factorization(21, 3, 7));
        assert!(!verify_factorization(15, 2, 8));
    }

    #[test]
    fn test_estimate_qubits() {
        let qubits = estimate_qubit_requirements(15);
        assert!(qubits >= 8);
    }
}