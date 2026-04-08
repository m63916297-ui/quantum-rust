use std::collections::HashMap;

pub struct QuantumState {
    pub amplitudes: HashMap<u64, f64>,
    pub num_qubits: usize,
}

impl QuantumState {
    pub fn new(num_qubits: usize) -> Self {
        let mut state = QuantumState {
            amplitudes: HashMap::new(),
            num_qubits,
        };
        state.amplitudes.insert(0, 1.0);
        state
    }

    pub fn from_amplitudes(amplitudes: HashMap<u64, f64>) -> Self {
        let num_qubits = if amplitudes.is_empty() {
            0
        } else {
            amplitudes.keys().max().unwrap().bits() as usize
        };
        QuantumState { amplitudes, num_qubits }
    }

    pub fn apply_hadamard(&mut self) {
        let factor = 1.0 / (2.0_f64.sqrt());
        let current_amplitudes = std::mem::take(&mut self.amplitudes);
        let mut new_amplitudes = HashMap::new();
        
        for (state, amp) in current_amplitudes {
            for i in 0..2 {
                let new_state = (state << 1) | i as u64;
                let entry = new_amplitudes.entry(new_state).or_insert(0.0);
                *entry += factor * amp;
            }
        }
        self.amplitudes = new_amplitudes;
    }

    pub fn apply_pauli_x(&mut self) {
        let current = std::mem::take(&mut self.amplitudes);
        let mut new = HashMap::new();
        
        for (state, amp) in current {
            let flipped = state ^ 1;
            new.insert(flipped, amp);
        }
        self.amplitudes = new;
    }

    pub fn apply_pauli_z(&mut self) {
        for (state, amp) in self.amplitudes.iter_mut() {
            if state & 1 == 1 {
                *amp = -*amp;
            }
        }
    }

    pub fn apply_cnot(&mut self) {
        let current = std::mem::take(&mut self.amplitudes);
        let mut new = HashMap::new();
        
        for (state, amp) in current {
            let control = (state >> 1) & 1;
            if control == 1 {
                let new_state = state ^ 1;
                new.insert(new_state, amp);
            } else {
                new.insert(state, amp);
            }
        }
        self.amplitudes = new;
    }

    pub fn apply_phase(&mut self, angle: f64) {
        for (state, amp) in self.amplitudes.iter_mut() {
            if state != 0 {
                *amp = amp.0.cos() * amp + amp.0.sin()();
            }
        }
    }

    pub fn measure(&self) -> u64 {
        let total_prob: f64 = self.amplitudes.values().map(|x| x * x).sum();
        
        if total_prob.is_nan() || total_prob == 0.0 {
            return 0;
        }
        
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        let rand_val = ((nanos as f64) % 1000.0 / 1000.0) * total_prob;
        
        let mut cumulative = 0.0;
        for (state, amp) in &self.amplitudes {
            cumulative += amp * amp;
            if rand_val <= cumulative {
                return *state;
            }
        }
        *self.amplitudes.keys().next().unwrap()
    }

    pub fn get_probability(&self, state: u64) -> f64 {
        self.amplitudes.get(&state).map(|a| a * a).unwrap_or(0.0)
    }

    pub fn normalize(&mut self) {
        let norm: f64 = self.amplitudes.values().map(|a| a * a).sum::<f64>().sqrt();
        if norm > 0.0 {
            for amp in self.amplitudes.values_mut() {
                *amp /= norm;
            }
        }
    }

    pub fn get_amplitude(&self, state: u64) -> Option<f64> {
        self.amplitudes.get(&state).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_state_init() {
        let state = QuantumState::new(3);
        assert_eq!(state.amplitudes.get(&0), Some(&1.0));
    }

    #[test]
    fn test_hadamard() {
        let mut state = QuantumState::new(1);
        state.apply_hadamard();
        let p0 = state.get_probability(0);
        let p1 = state.get_probability(1);
        assert!((p0 - 0.5).abs() < 0.01);
        assert!((p1 - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_pauli_x() {
        let mut state = QuantumState::new(1);
        state.amplitudes.insert(0, 0.0);
        state.amplitudes.insert(1, 1.0);
        state.apply_pauli_x();
        assert_eq!(state.amplitudes.get(&0), Some(&1.0));
    }

    #[test]
    fn test_normalize() {
        let mut state = QuantumState::new(1);
        state.amplitudes.insert(0, 2.0);
        state.amplitudes.insert(1, 0.0);
        state.normalize();
        assert!((state.get_probability(0) - 1.0).abs() < 0.01);
    }
}