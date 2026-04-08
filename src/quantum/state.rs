use thiserror::Error;
use std::collections::HashMap;

#[derive(Error, Debug, Clone)]
pub enum QuantumError {
    #[error("Número de qubits inválido: {0}")]
    InvalidQubitCount(usize),
    
    #[error("Qubit fuera de rango: {0}")]
    QubitOutOfRange(usize),
    
    #[error("Error de medición: {0}")]
    MeasurementError(String),
    
    #[error("Estado cuántico inválido: {0}")]
    InvalidState(String),
    
    #[error("Puerta no válida: {0}")]
    InvalidGate(String),
}

pub type QuantumResult<T> = Result<T, QuantumError>;

#[derive(Debug, Clone)]
pub struct QuantumState {
    pub amplitudes: HashMap<u64, f64>,
    pub num_qubits: usize,
}

impl QuantumState {
    pub fn new(num_qubits: usize) -> QuantumResult<Self> {
        if num_qubits == 0 {
            return Err(QuantumError::InvalidQubitCount(num_qubits));
        }
        if num_qubits > 20 {
            return Err(QuantumError::InvalidQubitCount(num_qubits));
        }
        
        let mut state = QuantumState {
            amplitudes: HashMap::new(),
            num_qubits,
        };
        state.amplitudes.insert(0, 1.0);
        Ok(state)
    }

    pub fn from_amplitudes(amplitudes: HashMap<u64, f64>, num_qubits: usize) -> QuantumResult<Self> {
        if num_qubits == 0 || num_qubits > 20 {
            return Err(QuantumError::InvalidQubitCount(num_qubits));
        }
        
        let state = QuantumState { amplitudes, num_qubits };
        Ok(state)
    }

    pub fn zero_state(num_qubits: usize) -> QuantumResult<Self> {
        Self::new(num_qubits)
    }

    pub fn one_state(num_qubits: usize) -> QuantumResult<Self> {
        let mut state = Self::new(num_qubits)?;
        state.amplitudes.clear();
        state.amplitudes.insert((1u64 << num_qubits) - 1, 1.0);
        Ok(state)
    }

    pub fn uniform_superposition(num_qubits: usize) -> QuantumResult<Self> {
        let mut state = Self::new(num_qubits)?;
        let num_states = 1u64 << num_qubits;
        let amplitude = 1.0 / (num_states as f64).sqrt();
        
        for i in 0..num_states {
            state.amplitudes.insert(i, amplitude);
        }
        
        Ok(state)
    }

    pub fn apply_hadamard(&mut self) -> QuantumResult<()> {
        if self.num_qubits == 0 {
            return Err(QuantumError::InvalidState("Estado no inicializado".to_string()));
        }
        
        let factor = 1.0 / (2.0_f64.sqrt());
        let current = std::mem::take(&mut self.amplitudes);
        let mut new = HashMap::new();
        
        for (state, amp) in current {
            for i in 0..2 {
                let new_state = (state << 1) | i as u64;
                let entry = new.entry(new_state).or_insert(0.0);
                *entry += factor * amp;
            }
        }
        
        self.amplitudes = new;
        Ok(())
    }

    pub fn apply_hadamard_to_qubit(&mut self, qubit: usize) -> QuantumResult<()> {
        if qubit >= self.num_qubits {
            return Err(QuantumError::QubitOutOfRange(qubit));
        }
        
        let mask = 1u64 << qubit;
        let other_mask = !mask;
        let mut new_amplitudes = HashMap::new();
        let factor = 1.0 / (2.0_f64.sqrt());
        
        let current = std::mem::take(&mut self.amplitudes);
        
        for (state, amp) in current {
            let without_qubit = state & other_mask;
            let qubit_val = (state >> qubit) & 1;
            
            let state_0 = without_qubit;
            let state_1 = without_qubit | mask;
            
            if qubit_val == 0 {
                *new_amplitudes.entry(state_0).or_insert(0.0) += factor * amp;
                *new_amplitudes.entry(state_1).or_insert(0.0) += factor * amp;
            } else {
                *new_amplitudes.entry(state_0).or_insert(0.0) += factor * amp;
                *new_amplitudes.entry(state_1).or_insert(0.0) -= factor * amp;
            }
        }
        
        self.amplitudes = new_amplitudes;
        Ok(())
    }

    pub fn apply_pauli_x(&mut self) -> QuantumResult<()> {
        let current = std::mem::take(&mut self.amplitudes);
        let mut new = HashMap::new();
        
        for (state, amp) in current {
            let flipped = state ^ 1;
            new.insert(flipped, amp);
        }
        
        self.amplitudes = new;
        Ok(())
    }

    pub fn apply_pauli_x_to_qubit(&mut self, qubit: usize) -> QuantumResult<()> {
        if qubit >= self.num_qubits {
            return Err(QuantumError::QubitOutOfRange(qubit));
        }
        
        let mask = 1u64 << qubit;
        let mut new = HashMap::new();
        
        let current = std::mem::take(&mut self.amplitudes);
        for (state, amp) in current {
            let flipped = state ^ mask;
            new.insert(flipped, amp);
        }
        
        self.amplitudes = new;
        Ok(())
    }

    pub fn apply_pauli_y(&mut self) -> QuantumResult<()> {
        let current = std::mem::take(&mut self.amplitudes);
        let mut new = HashMap::new();
        
        for (state, amp) in current {
            let flipped = state ^ 1;
            let phase = if state & 1 == 1 { -1.0 } else { 1.0 };
            new.insert(flipped, phase * amp);
        }
        
        self.amplitudes = new;
        Ok(())
    }

    pub fn apply_pauli_z(&mut self) -> QuantumResult<()> {
        for (state, amp) in self.amplitudes.iter_mut() {
            if state & 1 == 1 {
                *amp = -*amp;
            }
        }
        Ok(())
    }

    pub fn apply_cnot(&mut self, control: usize, target: usize) -> QuantumResult<()> {
        if control >= self.num_qubits || target >= self.num_qubits {
            return Err(QuantumError::QubitOutOfRange(
                control.max(target)
            ));
        }
        
        let control_mask = 1u64 << control;
        let target_mask = 1u64 << target;
        
        let current = std::mem::take(&mut self.amplitudes);
        let mut new = HashMap::new();
        
        for (state, amp) in current {
            if state & control_mask != 0 {
                let new_state = state ^ target_mask;
                new.insert(new_state, amp);
            } else {
                new.insert(state, amp);
            }
        }
        
        self.amplitudes = new;
        Ok(())
    }

    pub fn apply_phase(&mut self, angle: f64) -> QuantumResult<()> {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        
        for (state, amp) in self.amplitudes.iter_mut() {
            if *state != 0 {
                let real = cos_a * amp;
                let imag = sin_a * amp;
                *amp = (real.powi(2) + imag.powi(2)).sqrt();
            }
        }
        
        Ok(())
    }

    pub fn apply_rotation(&mut self, qubit: usize, angle: f64) -> QuantumResult<()> {
        if qubit >= self.num_qubits {
            return Err(QuantumError::QubitOutOfRange(qubit));
        }
        
        self.apply_phase(angle)
    }

    pub fn measure(&self) -> QuantumResult<u64> {
        let total_prob: f64 = self.amplitudes.values().map(|x| x * x).sum();
        
        if total_prob.is_nan() || total_prob <= 0.0 {
            return Err(QuantumError::MeasurementError("Probabilidad total inválida".to_string()));
        }
        
        let rand_val = Self::random_f64() * total_prob;
        
        let mut cumulative = 0.0;
        for (state, amp) in &self.amplitudes {
            cumulative += amp * amp;
            if rand_val <= cumulative {
                return Ok(*state);
            }
        }
        
        Ok(*self.amplitudes.keys().next().unwrap())
    }

    fn random_f64() -> f64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        (nanos as f64 % 1000000.0) / 1000000.0
    }

    pub fn measure_qubit(&self, qubit: usize) -> QuantumResult<u64> {
        if qubit >= self.num_qubits {
            return Err(QuantumError::QubitOutOfRange(qubit));
        }
        
        let result = self.measure()?;
        Ok((result >> qubit) & 1)
    }

    pub fn get_probability(&self, state: u64) -> f64 {
        self.amplitudes.get(&state).map(|a| a * a).unwrap_or(0.0)
    }

    pub fn get_probability_of_qubit(&self, qubit: usize) -> QuantumResult<(f64, f64)> {
        if qubit >= self.num_qubits {
            return Err(QuantumError::QubitOutOfRange(qubit));
        }
        
        let mask = 1u64 << qubit;
        
        let mut prob_0 = 0.0f64;
        let mut prob_1 = 0.0f64;
        
        for (state, amp) in &self.amplitudes {
            let p = amp * amp;
            if state & mask == 0 {
                prob_0 += p;
            } else {
                prob_1 += p;
            }
        }
        
        Ok((prob_0, prob_1))
    }

    pub fn normalize(&mut self) -> QuantumResult<()> {
        let norm: f64 = self.amplitudes.values().map(|a| a * a).sum::<f64>().sqrt();
        
        if norm.is_nan() || norm == 0.0 {
            return Err(QuantumError::InvalidState("Normalización imposible".to_string()));
        }
        
        for amp in self.amplitudes.values_mut() {
            *amp /= norm;
        }
        
        Ok(())
    }

    pub fn get_amplitude(&self, state: u64) -> Option<f64> {
        self.amplitudes.get(&state).copied()
    }

    pub fn get_num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn get_num_states(&self) -> usize {
        self.amplitudes.len()
    }

    pub fn calculate_entropy(&self) -> f64 {
        let mut entropy = 0.0f64;
        
        for amp in self.amplitudes.values() {
            let prob = amp * amp;
            if prob > 1e-10 {
                entropy -= prob * prob.log2();
            }
        }
        
        entropy
    }

    pub fn calculate_purity(&self) -> f64 {
        let mut purity = 0.0f64;
        for amp in self.amplitudes.values() {
            purity += amp * amp;
        }
        purity
    }

    pub fn clone(&self) -> Self {
        QuantumState {
            amplitudes: self.amplitudes.clone(),
            num_qubits: self.num_qubits,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_state_init() -> QuantumResult<()> {
        let state = QuantumState::new(3)?;
        assert_eq!(state.amplitudes.get(&0), Some(&1.0));
        Ok(())
    }

    #[test]
    fn test_invalid_qubit_count() {
        let result = QuantumState::new(0);
        assert!(result.is_err());
        
        let result_large = QuantumState::new(25);
        assert!(result_large.is_err());
    }

    #[test]
    fn test_hadamard() -> QuantumResult<()> {
        let mut state = QuantumState::new(1)?;
        state.apply_hadamard()?;
        
        let p0 = state.get_probability(0);
        let p1 = state.get_probability(1);
        assert!((p0 - 0.5).abs() < 0.01);
        assert!((p1 - 0.5).abs() < 0.01);
        Ok(())
    }

    #[test]
    fn test_pauli_x() -> QuantumResult<()> {
        let mut state = QuantumState::new(1)?;
        state.amplitudes.clear();
        state.amplitudes.insert(0, 0.0);
        state.amplitudes.insert(1, 1.0);
        state.apply_pauli_x()?;
        assert_eq!(state.amplitudes.get(&0), Some(&1.0));
        Ok(())
    }

    #[test]
    fn test_cnot() -> QuantumResult<()> {
        let mut state = QuantumState::new(2)?;
        state.amplitudes.clear();
        state.amplitudes.insert(0b10, 1.0); // |10>
        state.apply_cnot(0, 1)?; // Control=0, Target=1
        // Should become |11>
        assert_eq!(state.amplitudes.get(&0b11), Some(&1.0));
        Ok(())
    }

    #[test]
    fn test_measure() -> QuantumResult<()> {
        let state = QuantumState::uniform_superposition(2)?;
        let result = state.measure()?;
        assert!(result < 4);
        Ok(())
    }

    #[test]
    fn test_normalize() -> QuantumResult<()> {
        let mut state = QuantumState::new(1)?;
        state.amplitudes.insert(0, 2.0);
        state.amplitudes.insert(1, 0.0);
        state.normalize()?;
        assert!((state.get_probability(0) - 1.0).abs() < 0.01);
        Ok(())
    }

    #[test]
    fn test_entropy() -> QuantumResult<()> {
        let state = QuantumState::uniform_superposition(2)?;
        let entropy = state.calculate_entropy();
        assert!(entropy >= 0.0);
        Ok(())
    }

    #[test]
    fn test_uniform_superposition() -> QuantumResult<()> {
        let state = QuantumState::uniform_superposition(3)?;
        assert_eq!(state.get_num_states(), 8);
        assert!((state.calculate_purity() - 1.0).abs() < 0.01);
        Ok(())
    }
}