use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GateType {
    H,
    X,
    Y,
    Z,
    CNot,
    Swap,
    T,
    S,
    Phase,
    Toffoli,
}

#[derive(Debug, Clone)]
pub struct QuantumGate {
    pub gate_type: GateType,
    pub target_qubit: usize,
    pub control_qubits: Vec<usize>,
    pub params: Option<Vec<f64>>,
}

impl QuantumGate {
    pub fn hadamard(target: usize) -> Self {
        QuantumGate {
            gate_type: GateType::H,
            target_qubit: target,
            control_qubits: Vec::new(),
            params: None,
        }
    }

    pub fn pauli_x(target: usize) -> Self {
        QuantumGate {
            gate_type: GateType::X,
            target_qubit: target,
            control_qubits: Vec::new(),
            params: None,
        }
    }

    pub fn pauli_y(target: usize) -> Self {
        QuantumGate {
            gate_type: GateType::Y,
            target_qubit: target,
            control_qubits: Vec::new(),
            params: None,
        }
    }

    pub fn pauli_z(target: usize) -> Self {
        QuantumGate {
            gate_type: GateType::Z,
            target_qubit: target,
            control_qubits: Vec::new(),
            params: None,
        }
    }

    pub fn cnot(control: usize, target: usize) -> Self {
        QuantumGate {
            gate_type: GateType::CNot,
            target_qubit: target,
            control_qubits: vec![control],
            params: None,
        }
    }

    pub fn swap(q1: usize, q2: usize) -> Self {
        QuantumGate {
            gate_type: GateType::Swap,
            target_qubit: q1,
            control_qubits: vec![q2],
            params: None,
        }
    }

    pub fn phase(target: usize, angle: f64) -> Self {
        QuantumGate {
            gate_type: GateType::Phase,
            target_qubit: target,
            control_qubits: Vec::new(),
            params: Some(vec![angle]),
        }
    }

    pub fn toffoli(control1: usize, control2: usize, target: usize) -> Self {
        QuantumGate {
            gate_type: GateType::Toffoli,
            target_qubit: target,
            control_qubits: vec![control1, control2],
            params: None,
        }
    }

    pub fn get_matrix(&self) -> Vec<Vec<f64>> {
        match self.gate_type {
            GateType::H => vec![
                vec![1.0 / 2.0_f64.sqrt(), 1.0 / 2.0_f64.sqrt()],
                vec![1.0 / 2.0_f64.sqrt(), -1.0 / 2.0_f64.sqrt()],
            ],
            GateType::X => vec![
                vec![0.0, 1.0],
                vec![1.0, 0.0],
            ],
            GateType::Y => vec![
                vec![0.0, -1.0_f64.sqrt()],
                vec![1.0_f64.sqrt(), 0.0],
            ],
            GateType::Z => vec![
                vec![1.0, 0.0],
                vec![0.0, -1.0],
            ],
            _ => vec![vec![1.0, 0.0], vec![0.0, 1.0]],
        }
    }

    pub fn num_qubits_affected(&self) -> usize {
        match self.gate_type {
            GateType::CNot => 2,
            GateType::Swap => 2,
            GateType::Toffoli => 3,
            _ => 1,
        }
    }
}

pub struct GateSequence {
    pub gates: Vec<QuantumGate>,
}

impl GateSequence {
    pub fn new() -> Self {
        GateSequence { gates: Vec::new() }
    }

    pub fn add_gate(&mut self, gate: QuantumGate) {
        self.gates.push(gate);
    }

    pub fn add_hadamard(&mut self, target: usize) {
        self.add_gate(QuantumGate::hadamard(target));
    }

    pub fn add_cnot(&mut self, control: usize, target: usize) {
        self.add_gate(QuantumGate::cnot(control, target));
    }

    pub fn execute(&self, state: &mut super::state::QuantumState) {
        for gate in &self.gates {
            match gate.gate_type {
                GateType::H => state.apply_hadamard(),
                GateType::X => state.apply_pauli_x(),
                GateType::Y => {},
                GateType::Z => state.apply_pauli_z(),
                GateType::CNot => state.apply_cnot(),
                GateType::Swap => {},
                GateType::Phase => {
                    if let Some(params) = &gate.params {
                        state.apply_phase(params[0]);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn depth(&self) -> usize {
        self.gates.len()
    }

    pub fn gate_count(&self) -> HashMap<GateType, usize> {
        let mut counts = HashMap::new();
        for gate in &self.gates {
            *counts.entry(gate.gate_type).or_insert(0) += 1;
        }
        counts
    }
}

impl Default for GateSequence {
    fn default() -> Self {
        Self::new()
    }
}

pub fn create_shor_circuit(n: u64) -> GateSequence {
    let mut sequence = GateSequence::new();
    let num_qubits = ((n as f64).log2().ceil() as usize).max(2);
    
    for i in 0..num_qubits {
        sequence.add_hadamard(i);
    }
    
    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard_gate() {
        let gate = QuantumGate::hadamard(0);
        assert_eq!(gate.gate_type, GateType::H);
    }

    #[test]
    fn test_cnot_gate() {
        let gate = QuantumGate::cnot(0, 1);
        assert_eq!(gate.gate_type, GateType::CNot);
    }

    #[test]
    fn test_gate_sequence() {
        let mut seq = GateSequence::new();
        seq.add_hadamard(0);
        seq.add_cnot(0, 1);
        assert_eq!(seq.depth(), 2);
    }
}