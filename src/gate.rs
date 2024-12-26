use crate::pauli_string::PauliString;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Gate<const N: usize> {
    ps: PauliString<N>,
    time: f64,
}

impl<const N: usize> Gate<N> {
    #[inline(always)]
    pub(super) fn new(ps: PauliString<N>, time: f64) -> Self {
        Self { ps, time }
    }
    #[inline(always)]
    pub(super) fn get_time(&self) -> f64 {
        self.time
    }
    #[inline(always)]
    pub(super) fn get_pauli_string(&self) -> &PauliString<N> {
        &self.ps
    }
}
