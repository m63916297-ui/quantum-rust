use std::collections::HashMap;
use super::state::QuantumState;

#[derive(Debug, Clone)]
pub struct QuantumMeasurement {
    pub result: u64,
    pub probability: f64,
    pub measurement_basis: Basis,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Basis {
    Computational,
    Hadamard,
    Circular,
}

impl QuantumMeasurement {
    pub fn new(result: u64, probability: f64) -> Self {
        QuantumMeasurement {
            result,
            probability,
            Basis::Computational,
        }
    }

    pub fn with_basis(result: u64, probability: f64, basis: Basis) -> Self {
        QuantumMeasurement {
            result,
            probability,
            measurement_basis: basis,
        }
    }

    pub fn measure_in_hadamard_basis(state: &mut QuantumState) -> Self {
        let original_amplitudes = state.amplitudes.clone();
        state.apply_hadamard();
        let result = state.measure();
        
        let probability = original_amplitudes
            .get(&result)
            .map(|a| a * a)
            .unwrap_or(0.0);
        
        state.amplitudes = original_amplitudes;
        
        QuantumMeasurement::with_basis(result, probability, Basis::Hadamard)
    }

    pub fn expectation_value(&self, observable: &HashMap<u64, f64>) -> f64 {
        let mut expectation = 0.0;
        for (state, eigenvalue) in observable {
            if *state == self.result {
                expectation = *eigenvalue;
                break;
            }
        }
        expectation * self.probability
    }
}

pub fn measure_all(state: &QuantumState) -> Vec<QuantumMeasurement> {
    let mut measurements = Vec::new();
    let total_prob: f64 = state.amplitudes.values().map(|a| a * a).sum();
    
    if total_prob > 0.0 {
        for (state_val, amp) in &state.amplitudes {
            let prob = amp * amp / total_prob;
            measurements.push(QuantumMeasurement::new(*state_val, prob));
        }
    }
    
    measurements.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());
    measurements
}

pub fn measure_partial(state: &QuantumState, qubits: &[usize]) -> Vec<QuantumMeasurement> {
    let mask: u64 = qubits.iter().fold(0u64, |acc, &q| acc | (1 << q));
    let mut marginal_probs: HashMap<u64, f64> = HashMap::new();
    
    for (state_val, amp) in &state.amplitudes {
        let partial_state = state_val & mask;
        *marginal_probs.entry(partial_state).or_insert(0.0) += amp * amp;
    }
    
    marginal_probs
        .into_iter()
        .map(|(state_val, prob)| QuantumMeasurement::new(state_val, prob))
        .collect()
}

pub fn calculate_entropy(state: &QuantumState) -> f64 {
    let mut entropy = 0.0;
    for amp in state.amplitudes.values() {
        let prob = amp * amp;
        if prob > 0.0 {
            entropy -= prob * prob.log2();
        }
    }
    entropy
}

pub fn calculate_purity(state: &QuantumState) -> f64 {
    let mut purity = 0.0;
    for amp in state.amplitudes.values() {
        purity += amp * amp;
    }
    purity
}

pub fn collapse_state(state: &mut QuantumState, measured_qubit: usize) -> u64 {
    let result = state.measure();
    let bit = (result >> measured_qubit) & 1;
    
    let new_amplitudes: HashMap<u64, f64> = state
        .amplitudes
        .iter()
        .filter(|(&s, _)| ((s >> measured_qubit) & 1) == bit)
        .map(|(&s, &a)| (s, a))
        .collect();
    
    state.amplitudes = new_amplitudes;
    state.normalize();
    
    bit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement_creation() {
        let measurement = QuantumMeasurement::new(5, 0.3);
        assert_eq!(measurement.result, 5);
        assert!((measurement.probability - 0.3).abs() < 0.001);
    }

    #[test]
    fn test_calculate_entropy() {
        let state = QuantumState::new(2);
        let entropy = calculate_entropy(&state);
        assert!(entropy >= 0.0);
    }

    #[test]
    fn test_calculate_purity() {
        let mut state = QuantumState::new(1);
        state.amplitudes.insert(0, 1.0);
        let purity = calculate_purity(&state);
        assert!((purity - 1.0).abs() < 0.001);
    }
}