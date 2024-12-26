use crate::{
    pauli_string::{CHUNK_SIZE, PauliStringError},
    spsim::{SPSim, SPSimTrait},
};
use num_complex::Complex64;
use pyo3::{
    create_exception,
    exceptions::{PyException, PyValueError},
    prelude::*,
};
use std::fmt::Write;

create_exception!(
    spsim,
    SPSimError,
    PyException,
    "This is the Pauli Simulator exception"
);

impl From<PauliStringError> for PyErr {
    fn from(value: PauliStringError) -> Self {
        let mut s = String::new();
        write!(&mut s, "{}", value).unwrap();
        SPSimError::new_err(s)
    }
}

#[pyclass(name = "SPSim")]
struct SPSimPy(Box<dyn SPSimTrait + Send + Sync>);

#[pymethods]
impl SPSimPy {
    #[new]
    #[pyo3(signature = (qubits_number,))]
    fn new(qubits_number: usize) -> PyResult<Self> {
        let n = qubits_number / CHUNK_SIZE + ((qubits_number % CHUNK_SIZE) != 0) as usize;
        let psim = match n {
            1 => SPSimPy(Box::new(SPSim::<1>::new())),
            2 => SPSimPy(Box::new(SPSim::<2>::new())),
            3 => SPSimPy(Box::new(SPSim::<3>::new())),
            4 => SPSimPy(Box::new(SPSim::<4>::new())),
            5 => SPSimPy(Box::new(SPSim::<5>::new())),
            6 => SPSimPy(Box::new(SPSim::<6>::new())),
            7 => SPSimPy(Box::new(SPSim::<7>::new())),
            8 => SPSimPy(Box::new(SPSim::<8>::new())),
            9 => SPSimPy(Box::new(SPSim::<9>::new())),
            10 => SPSimPy(Box::new(SPSim::<10>::new())),
            11 => SPSimPy(Box::new(SPSim::<11>::new())),
            12 => SPSimPy(Box::new(SPSim::<12>::new())),
            13 => SPSimPy(Box::new(SPSim::<13>::new())),
            14 => SPSimPy(Box::new(SPSim::<14>::new())),
            15 => SPSimPy(Box::new(SPSim::<15>::new())),
            16 => SPSimPy(Box::new(SPSim::<16>::new())),
            17 => SPSimPy(Box::new(SPSim::<17>::new())),
            18 => SPSimPy(Box::new(SPSim::<18>::new())),
            19 => SPSimPy(Box::new(SPSim::<19>::new())),
            20 => SPSimPy(Box::new(SPSim::<20>::new())),
            21 => SPSimPy(Box::new(SPSim::<21>::new())),
            22 => SPSimPy(Box::new(SPSim::<22>::new())),
            23 => SPSimPy(Box::new(SPSim::<23>::new())),
            24 => SPSimPy(Box::new(SPSim::<24>::new())),
            25 => SPSimPy(Box::new(SPSim::<25>::new())),
            26 => SPSimPy(Box::new(SPSim::<26>::new())),
            27 => SPSimPy(Box::new(SPSim::<27>::new())),
            28 => SPSimPy(Box::new(SPSim::<28>::new())),
            29 => SPSimPy(Box::new(SPSim::<29>::new())),
            30 => SPSimPy(Box::new(SPSim::<30>::new())),
            31 => SPSimPy(Box::new(SPSim::<31>::new())),
            32 => SPSimPy(Box::new(SPSim::<32>::new())),
            _ => return Err(PyValueError::new_err("To many qubits")),
        };
        Ok(psim)
    }
    #[pyo3(signature = (pauli_string_discreption, time))]
    fn add_gate(
        &mut self,
        pauli_string_discreption: Vec<(char, usize)>,
        time: f64,
    ) -> PyResult<()> {
        self.0.add_gate(&pauli_string_discreption, time)?;
        Ok(())
    }
    #[pyo3(signature = ())]
    fn qubits_number(&self) -> usize {
        self.0.qubits_number()
    }
    #[pyo3(signature = (observable_description, layers_number, threshold, decay=1.))]
    fn execute(
        &self,
        observable_description: Vec<(char, usize)>,
        layers_number: usize,
        threshold: f64,
        decay: f64,
    ) -> PyResult<Vec<Complex64>> {
        self.0
            .execute(&observable_description, layers_number, threshold, decay)
            .map_err(Into::<PyErr>::into)
    }
    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

#[pymodule]
fn spsim(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SPSimPy>()?;
    Ok(())
}
