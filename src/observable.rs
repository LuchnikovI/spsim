use crate::{gate::Gate, pauli_string::PauliString};
use fxhash::FxHashMap;
use num_complex::{Complex64, ComplexFloat};
use std::f64::consts::FRAC_PI_2;

pub(super) const EPS: f64 = 1e-10;

#[derive(Debug, Clone)]
pub(super) struct Observable<const N: usize>(FxHashMap<PauliString<N>, Complex64>);

impl<const N: usize> Observable<N> {
    pub(super) fn new(ps: PauliString<N>) -> Self {
        let mut map = FxHashMap::default();
        map.insert(ps, Complex64::ONE);
        Self(map)
    }
    #[inline(always)]
    pub(super) fn size(&self) -> usize {
        self.0.len()
    }
    pub(super) fn get_average(&self) -> Complex64 {
        let mut av_val = Complex64::ZERO;
        for (ps, val) in self.0.iter() {
            if ps.average() == 1 {
                av_val += val;
            }
        }
        av_val
    }
    pub(super) fn apply_gate(&mut self, gate: &Gate<N>, threshold: f64, decay: f64) {
        let cos_weight = Complex64::new((2f64 * gate.get_time()).cos(), 0f64);
        let sin_weight = Complex64::new(0f64, -(2f64 * gate.get_time()).sin());
        let mut tmp = Vec::new();
        if sin_weight.abs() < EPS {
            for (key, val) in self.0.iter_mut() {
                if !key.comute(gate.get_pauli_string()) {
                    *val *= cos_weight
                }
            }
        } else if cos_weight.abs() < EPS {
            for (key, val) in self
                .0
                .extract_if(|key, _| !key.comute(gate.get_pauli_string()))
            {
                let (phase, new_key) = gate.get_pauli_string() * &key;
                let new_val =
                    Complex64::new(0f64, FRAC_PI_2 * phase as f64).exp() * sin_weight * val;
                tmp.push((new_key, new_val));
            }
        } else {
            for (key, val) in self.0.iter_mut() {
                if !key.comute(gate.get_pauli_string()) {
                    let (phase, new_key) = gate.get_pauli_string() * key;
                    let new_val =
                        Complex64::new(0f64, FRAC_PI_2 * phase as f64).exp() * sin_weight * *val;
                    tmp.push((new_key, new_val));
                    *val *= cos_weight;
                }
            }
        }
        if !tmp.is_empty() {
            for (key, value) in tmp {
                self.0
                    .entry(key)
                    .and_modify(|v| {
                        *v += value;
                    })
                    .or_insert(value);
            }
            for _ in self.0.extract_if(|key, value| {
                value.abs() / 2f64.powf(key.hamming() as f64 * decay / 2f64) < threshold
            }) {}
        }
    }
}
