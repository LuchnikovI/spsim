use crate::{
    gate::Gate,
    observable::Observable,
    pauli_string::{CHUNK_SIZE, PauliString, PauliStringResult},
};
use num_complex::Complex64;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub(super) struct SPSim<const N: usize>(Vec<Gate<N>>);

impl<const N: usize> SPSim<N> {
    pub(super) fn new() -> Self {
        Self(Vec::new())
    }
}

pub(super) trait SPSimTrait {
    fn add_gate(
        &mut self,
        pauli_string_discreption: &[(char, usize)],
        time: f64,
    ) -> PauliStringResult<()>;
    fn qubits_number(&self) -> usize;
    fn to_string(&self) -> String;
    fn execute(
        &self,
        observable_description: &[(char, usize)],
        layers_number: usize,
        threshold: f64,
        decay: f64,
    ) -> PauliStringResult<Vec<Complex64>>;
}

impl<const N: usize> SPSimTrait for SPSim<N> {
    fn add_gate(
        &mut self,
        pauli_string_discreption: &[(char, usize)],
        time: f64,
    ) -> PauliStringResult<()> {
        let mut ps = PauliString::new();
        for &(code, pos) in pauli_string_discreption {
            ps = ps.set_pauli(code, pos as u64)?;
        }
        let gate = Gate::new(ps, time);
        self.0.push(gate);
        Ok(())
    }
    fn qubits_number(&self) -> usize {
        N * CHUNK_SIZE
    }
    fn to_string(&self) -> String {
        let mut s = String::new();
        writeln!(&mut s, "qubits_number: {}", self.qubits_number()).unwrap();
        write!(&mut s, "layer discription:").unwrap();
        for gate in &self.0 {
            write!(&mut s, "\n\t- gate:").unwrap();
            write!(&mut s, "\n\t\tpauli_string: {}", gate.get_pauli_string()).unwrap();
            write!(&mut s, "\n\t\ttime: {}", gate.get_time()).unwrap();
        }
        s
    }
    fn execute(
        &self,
        observable_description: &[(char, usize)],
        layers_number: usize,
        threshold: f64,
        decay: f64,
    ) -> PauliStringResult<Vec<Complex64>> {
        let mut dynamics = Vec::new();
        let mut ps = PauliString::new();
        for &(code, pos) in observable_description {
            ps = ps.set_pauli(code, pos as u64)?;
        }
        let mut obs = Observable::new(ps);
        for layer in 0..layers_number {
            dynamics.push(obs.get_average());
            for gate in self.0.iter().rev() {
                obs.apply_gate(gate, threshold, decay);
            }
            println!(
                "layer_number: {layer}, pauli_strings_number: {}",
                obs.size()
            );
        }
        dynamics.push(obs.get_average());
        Ok(dynamics)
    }
}
