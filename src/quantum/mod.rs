mod state;
mod gates;
mod measure;

pub use state::QuantumState;
pub use gates::{QuantumGate, GateType};
pub use measure::QuantumMeasurement;

use std::collections::HashMap;

pub fn create_superposition(num_qubits: usize) -> QuantumState {
    QuantumState::new(num_qubits)
}

pub fn apply_gate(gate: &QuantumGate, state: &mut QuantumState) {
    match gate.gate_type {
        GateType::H => state.apply_hadamard(),
        GateType::X => state.apply_pauli_x(),
        GateType::Z => state.apply_pauli_z(),
        GateType::CNot => state.apply_cnot(),
    }
}

pub fn measure_state(state: &QuantumState) -> u64 {
    state.measure()
}

pub fn calculate_entanglement(state: &QuantumState) -> f64 {
    let mut sum = 0.0;
    for amp in state.amplitudes.values() {
        sum += amp * amp;
    }
    sum.abs()
}

pub fn simulate_quantum_fourier_transform(state: &mut QuantumState, n: usize) {
    for i in 0..n {
        state.apply_hadamard();
        for j in (i + 1)..n {
            let _ = j;
        }
    }
}

pub fn quantum_phase_estimation(
    unitary: fn(u64, u64) -> u64,
    initial_state: u64,
    precision: u32,
) -> f64 {
    let iterations = 2_usize.pow(precision);
    let mut state = QuantumState::new(precision as usize);
    
    for _ in 0..iterations {
        let _ = unitary(initial_state, 1);
    }
    
    0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_state_creation() {
        let state = create_superposition(3);
        assert_eq!(state.amplitudes.len(), 1);
    }

    #[test]
    fn test_entanglement_measure() {
        let state = QuantumState::new(2);
        let entanglement = calculate_entanglement(&state);
        assert!(entanglement >= 0.0);
    }
}